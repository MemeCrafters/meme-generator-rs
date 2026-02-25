<script setup lang="ts">
import { ref, reactive, computed, onMounted, watch } from 'vue'
import { useRouter } from 'vue-router'
import type { MemeInfo, MemeOption, ImageItem } from '../types'
import { getMemeInfo, getMemePreview, uploadImage, generateMeme, getImageUrl, MemeError } from '../api'
import ImageUploader from '../components/ImageUploader.vue'
import OptionField from '../components/OptionField.vue'

const props = defineProps<{ memeKey: string }>()
const router = useRouter()

const memeInfo = ref<MemeInfo | null>(null)
const loading = ref(true)
const generating = ref(false)
const error = ref('')
const errorHint = ref('')

function formatError(err: unknown): { msg: string; hint: string } {
  if (err instanceof MemeError) {
    switch (err.code) {
      case 550: {
        const { min, max, actual } = err.data || {}
        const range = min === max ? `${min}` : `${min}~${max}`
        return {
          msg: `图片数量不符合要求`,
          hint: `需要 ${range} 张图片，当前提供了 ${actual} 张`,
        }
      }
      case 551: {
        const { min, max, actual } = err.data || {}
        const range = min === max ? `${min}` : `${min}~${max}`
        return {
          msg: `文字数量不符合要求`,
          hint: `需要 ${range} 段文字，当前提供了 ${actual} 段`,
        }
      }
      case 560:
        return {
          msg: `文字内容过长`,
          hint: `请缩短文字后重试`,
        }
      case 570:
        return {
          msg: err.data?.feedback || '生成失败',
          hint: '',
        }
      case 510:
        return {
          msg: `图片解码失败`,
          hint: `请检查图片格式是否正确（支持 PNG、JPG、GIF）`,
        }
      case 520:
        return {
          msg: `图片编码失败`,
          hint: `生成过程中出现错误，请重试`,
        }
      case 530:
        return {
          msg: `模板资源缺失`,
          hint: `该模板的素材文件缺失，请联系管理员`,
        }
      case 540:
        return {
          msg: `参数解析失败`,
          hint: `请检查输入的参数是否正确`,
        }
      case 410:
        return {
          msg: `网络请求失败`,
          hint: `无法下载图片资源，请检查网络连接`,
        }
      case 420:
        return {
          msg: `文件读写失败`,
          hint: `服务器内部错误，请稍后重试`,
        }
      default:
        return {
          msg: err.message || '生成失败',
          hint: '',
        }
    }
  }
  return {
    msg: '生成失败',
    hint: '发生未知错误，请重试',
  }
}
const previewUrl = ref('')
const resultUrl = ref('')
const resultId = ref('')

// Form state
const images = ref<ImageItem[]>([])
const texts = ref<string[]>([])
const options = reactive<Record<string, any>>({})

const canGenerate = computed(() => {
  if (!memeInfo.value) return false
  const p = memeInfo.value.params
  const imgCount = images.value.length
  const txtCount = texts.value.filter((t) => t.trim()).length
  const hasEnoughImages = imgCount >= p.min_images
  const hasEnoughTexts = txtCount >= p.min_texts || (p.min_texts === 0)
  return hasEnoughImages && hasEnoughTexts && !generating.value
})

const needsImages = computed(() => {
  if (!memeInfo.value) return false
  return memeInfo.value.params.max_images > 0
})

const needsTexts = computed(() => {
  if (!memeInfo.value) return false
  return memeInfo.value.params.max_texts > 0
})

function initFormState() {
  if (!memeInfo.value) return
  const p = memeInfo.value.params

  // Init texts
  texts.value = p.default_texts.length > 0
    ? [...p.default_texts]
    : Array(Math.max(p.min_texts, 1)).fill('').slice(0, p.max_texts || 1)

  if (!needsTexts.value) {
    texts.value = []
  }

  // Init options
  for (const opt of p.options) {
    switch (opt.type) {
      case 'boolean':
        options[opt.name] = opt.default ?? false
        break
      case 'string':
        options[opt.name] = opt.default ?? ''
        break
      case 'integer':
        options[opt.name] = opt.default ?? 0
        break
      case 'float':
        options[opt.name] = opt.default ?? 0
        break
    }
  }
}

function addText() {
  if (!memeInfo.value) return
  if (texts.value.length < memeInfo.value.params.max_texts) {
    texts.value.push('')
  }
}

function removeText(index: number) {
  texts.value.splice(index, 1)
}

async function handleImagesChange(newImages: ImageItem[]) {
  images.value = newImages
}

async function generate() {
  if (!memeInfo.value || generating.value) return

  generating.value = true
  error.value = ''
  errorHint.value = ''
  resultUrl.value = ''

  try {
    // Upload any images that don't have an ID yet
    const uploadedImages: { name: string; id: string }[] = []
    for (const img of images.value) {
      if (img.id) {
        uploadedImages.push({ name: img.name, id: img.id })
      } else if (img.file) {
        const resp = await uploadImage(img.file)
        img.id = resp.image_id
        uploadedImages.push({ name: img.name, id: resp.image_id })
      }
    }

    // Filter non-empty texts
    const filteredTexts = texts.value.filter((t) => t.trim() !== '' || memeInfo.value!.params.min_texts > 0)
    const finalTexts = filteredTexts.length > 0 ? filteredTexts : texts.value

    // Build options, only send non-default and non-empty values
    const finalOptions: Record<string, any> = {}
    for (const opt of memeInfo.value.params.options) {
      const val = options[opt.name]
      // Skip empty strings (unselected dropdowns)
      if (val === undefined || val === null || val === '') continue
      // Skip values equal to default
      if (val === opt.default) continue
      finalOptions[opt.name] = val
    }

    const resp = await generateMeme(
      memeInfo.value.key,
      uploadedImages,
      finalTexts,
      finalOptions
    )
    resultId.value = resp.image_id
    resultUrl.value = getImageUrl(resp.image_id)
  } catch (err: any) {
    const { msg, hint } = formatError(err)
    error.value = msg
    errorHint.value = hint
  } finally {
    generating.value = false
  }
}

async function downloadResult() {
  if (!resultUrl.value) return
  const a = document.createElement('a')
  a.href = resultUrl.value
  a.download = `meme-${memeInfo.value?.key || 'result'}`
  document.body.appendChild(a)
  a.click()
  document.body.removeChild(a)
}

onMounted(async () => {
  try {
    memeInfo.value = await getMemeInfo(props.memeKey)
    initFormState()

    // Load preview
    try {
      const prev = await getMemePreview(props.memeKey)
      previewUrl.value = getImageUrl(prev.image_id)
    } catch {
      // Preview may not be available
    }
  } catch (err: any) {
    error.value = err.message || '加载失败'
  } finally {
    loading.value = false
  }
})
</script>

<template>
  <div class="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <!-- Back button -->
    <button @click="router.push('/')" class="btn-secondary mb-6 gap-2">
      <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
      </svg>
      返回列表
    </button>

    <!-- Loading -->
    <div v-if="loading" class="flex justify-center py-20">
      <div class="flex items-center gap-3 text-gray-500">
        <svg class="animate-spin w-5 h-5" viewBox="0 0 24 24" fill="none">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
        </svg>
        <span>加载中...</span>
      </div>
    </div>

    <template v-else-if="memeInfo">
      <!-- Header -->
      <div class="mb-8">
        <h2 class="text-2xl font-bold text-gray-900 mb-2">
          {{ memeInfo.keywords[0] || memeInfo.key }}
        </h2>
        <div class="flex flex-wrap items-center gap-2">
          <span v-for="kw in memeInfo.keywords" :key="kw" class="badge-blue">{{ kw }}</span>
          <router-link
            v-for="tag in memeInfo.tags"
            :key="tag"
            :to="{ name: 'home', query: { tag } }"
            class="badge-gray hover:bg-gray-200 transition-colors cursor-pointer"
          >{{ tag }}</router-link>
        </div>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
        <!-- Left: Form -->
        <div class="space-y-6">
          <!-- Image Upload Section -->
          <div v-if="needsImages" class="card p-6">
            <h3 class="text-base font-semibold text-gray-900 mb-1">
              上传图片
            </h3>
            <p class="text-sm text-gray-500 mb-4">
              需要 {{ memeInfo.params.min_images }}
              <template v-if="memeInfo.params.min_images !== memeInfo.params.max_images">
                ~ {{ memeInfo.params.max_images }}
              </template>
              张图片
            </p>
            <ImageUploader
              :min="memeInfo.params.min_images"
              :max="memeInfo.params.max_images"
              :images="images"
              @update="handleImagesChange"
            />
          </div>

          <!-- Text Input Section -->
          <div v-if="needsTexts" class="card p-6">
            <h3 class="text-base font-semibold text-gray-900 mb-1">
              输入文字
            </h3>
            <p class="text-sm text-gray-500 mb-4">
              需要 {{ memeInfo.params.min_texts }}
              <template v-if="memeInfo.params.min_texts !== memeInfo.params.max_texts">
                ~ {{ memeInfo.params.max_texts }}
              </template>
              段文字
            </p>
            <div class="space-y-3">
              <div v-for="(_, idx) in texts" :key="idx" class="flex gap-2">
                <input
                  v-model="texts[idx]"
                  :placeholder="`文字 ${idx + 1}`"
                  class="input flex-1"
                />
                <button
                  v-if="texts.length > memeInfo.params.min_texts"
                  @click="removeText(idx)"
                  class="p-2 text-gray-400 hover:text-red-500 transition-colors"
                  title="删除"
                >
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                  </svg>
                </button>
              </div>
              <button
                v-if="texts.length < memeInfo.params.max_texts"
                @click="addText"
                class="btn-secondary w-full gap-1 text-xs"
              >
                <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                </svg>
                添加文字
              </button>
            </div>
          </div>

          <!-- Options Section -->
          <div v-if="memeInfo.params.options.length > 0" class="card p-6">
            <h3 class="text-base font-semibold text-gray-900 mb-4">选项</h3>
            <div class="space-y-4">
              <OptionField
                v-for="opt in memeInfo.params.options"
                :key="opt.name"
                :option="opt"
                :modelValue="options[opt.name]"
                @update:modelValue="(val: any) => (options[opt.name] = val)"
              />
            </div>
          </div>

          <!-- Generate Button -->
          <button
            @click="generate"
            :disabled="!canGenerate"
            class="btn-primary w-full py-3 text-base gap-2"
          >
            <svg v-if="generating" class="animate-spin w-5 h-5" viewBox="0 0 24 24" fill="none">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
            </svg>
            <svg v-else class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
            </svg>
            {{ generating ? '生成中...' : '生成表情包' }}
          </button>

          <!-- Error -->
          <div v-if="error" class="rounded-lg bg-red-50 border border-red-200 p-4">
            <div class="flex gap-2">
              <svg class="w-5 h-5 text-red-500 shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
              <div>
                <p class="text-sm font-medium text-red-700">{{ error }}</p>
                <p v-if="errorHint" class="text-xs text-red-500 mt-1">{{ errorHint }}</p>
              </div>
            </div>
          </div>
        </div>

        <!-- Right: Preview / Result -->
        <div class="space-y-6">
          <!-- Result -->
          <div v-if="resultUrl" class="card p-6">
            <div class="flex items-center justify-between mb-4">
              <h3 class="text-base font-semibold text-gray-900">生成结果</h3>
              <button @click="downloadResult" class="btn-secondary text-xs gap-1">
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
                </svg>
                下载
              </button>
            </div>
            <div class="bg-gray-50 rounded-lg p-4 flex justify-center">
              <img :src="resultUrl" alt="Generated meme" class="max-w-full max-h-96 object-contain rounded" />
            </div>
          </div>

          <!-- Preview -->
          <div class="card p-6">
            <h3 class="text-base font-semibold text-gray-900 mb-4">预览</h3>
            <div class="bg-gray-50 rounded-lg p-4 flex justify-center min-h-48">
              <img
                v-if="previewUrl"
                :src="previewUrl"
                alt="Preview"
                class="max-w-full max-h-80 object-contain rounded"
              />
              <div v-else class="flex items-center justify-center text-gray-400">
                <div class="text-center">
                  <div class="text-4xl mb-2">🖼️</div>
                  <p class="text-sm">暂无预览</p>
                </div>
              </div>
            </div>
          </div>

        </div>
      </div>
    </template>
  </div>
</template>
