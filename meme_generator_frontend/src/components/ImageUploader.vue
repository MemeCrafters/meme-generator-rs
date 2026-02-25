<script setup lang="ts">
import { ref, computed } from 'vue'
import type { ImageItem } from '../types'
import { uploadImage } from '../api'

const props = defineProps<{
  min: number
  max: number
  images: ImageItem[]
}>()

const emit = defineEmits<{
  update: [images: ImageItem[]]
}>()

const uploading = ref(false)
const dragOver = ref(false)

const canAdd = computed(() => props.images.length < props.max)

async function handleFiles(files: FileList | File[]) {
  const fileArray = Array.from(files)
  const remaining = props.max - props.images.length
  const toProcess = fileArray.slice(0, remaining)

  if (toProcess.length === 0) return

  uploading.value = true
  const newImages: ImageItem[] = [...props.images]

  for (const file of toProcess) {
    try {
      const preview = URL.createObjectURL(file)
      const resp = await uploadImage(file)
      newImages.push({
        name: file.name.replace(/\.[^.]+$/, ''),
        id: resp.image_id,
        preview,
        file,
      })
    } catch (err) {
      console.error('Upload failed:', err)
    }
  }

  emit('update', newImages)
  uploading.value = false
}

function onFileInput(event: Event) {
  const input = event.target as HTMLInputElement
  if (input.files) {
    handleFiles(input.files)
    input.value = ''
  }
}

function onDrop(event: DragEvent) {
  dragOver.value = false
  if (event.dataTransfer?.files) {
    handleFiles(event.dataTransfer.files)
  }
}

function removeImage(index: number) {
  const newImages = [...props.images]
  const removed = newImages.splice(index, 1)
  if (removed[0]?.preview) {
    URL.revokeObjectURL(removed[0].preview)
  }
  emit('update', newImages)
}

function updateName(index: number, name: string) {
  const newImages = [...props.images]
  newImages[index] = { ...newImages[index], name }
  emit('update', newImages)
}

function moveImage(index: number, direction: -1 | 1) {
  const newIndex = index + direction
  if (newIndex < 0 || newIndex >= props.images.length) return
  const newImages = [...props.images]
  ;[newImages[index], newImages[newIndex]] = [newImages[newIndex], newImages[index]]
  emit('update', newImages)
}
</script>

<template>
  <div class="space-y-3">
    <!-- Uploaded Images List -->
    <div v-if="images.length > 0" class="space-y-2">
      <div
        v-for="(img, idx) in images"
        :key="idx"
        class="flex items-center gap-3 p-3 bg-gray-50 rounded-lg group"
      >
        <!-- Thumbnail -->
        <div class="w-12 h-12 rounded-lg overflow-hidden bg-white border border-gray-200 shrink-0">
          <img
            v-if="img.preview"
            :src="img.preview"
            class="w-full h-full object-cover"
          />
          <div v-else class="w-full h-full flex items-center justify-center text-gray-400 text-xs">
            ?
          </div>
        </div>

        <!-- Name Input -->
        <input
          :value="img.name"
          @input="(e) => updateName(idx, (e.target as HTMLInputElement).value)"
          class="input flex-1 text-sm py-1.5"
          placeholder="图片名称"
        />

        <!-- Controls -->
        <div class="flex items-center gap-1">
          <button
            v-if="images.length > 1"
            @click="moveImage(idx, -1)"
            :disabled="idx === 0"
            class="p-1.5 text-gray-400 hover:text-gray-600 disabled:opacity-30 transition-colors"
            title="上移"
          >
            <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 15l7-7 7 7" />
            </svg>
          </button>
          <button
            v-if="images.length > 1"
            @click="moveImage(idx, 1)"
            :disabled="idx === images.length - 1"
            class="p-1.5 text-gray-400 hover:text-gray-600 disabled:opacity-30 transition-colors"
            title="下移"
          >
            <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
            </svg>
          </button>
          <button
            @click="removeImage(idx)"
            class="p-1.5 text-gray-400 hover:text-red-500 transition-colors"
            title="删除"
          >
            <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
            </svg>
          </button>
        </div>
      </div>
    </div>

    <!-- Drop Zone -->
    <div
      v-if="canAdd"
      @dragenter.prevent="dragOver = true"
      @dragover.prevent="dragOver = true"
      @dragleave.prevent="dragOver = false"
      @drop.prevent="onDrop"
      :class="[
        'relative border-2 border-dashed rounded-xl p-6 text-center transition-colors duration-200 cursor-pointer',
        dragOver
          ? 'border-primary-400 bg-primary-50'
          : 'border-gray-300 hover:border-primary-300 hover:bg-gray-50',
      ]"
      @click="($refs.fileInput as HTMLInputElement).click()"
    >
      <input
        ref="fileInput"
        type="file"
        accept="image/*"
        multiple
        class="hidden"
        @change="onFileInput"
      />
      <div v-if="uploading" class="flex items-center justify-center gap-2 text-primary-500">
        <svg class="animate-spin w-5 h-5" viewBox="0 0 24 24" fill="none">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
        </svg>
        <span class="text-sm">上传中...</span>
      </div>
      <div v-else>
        <svg class="mx-auto w-8 h-8 text-gray-400 mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
        </svg>
        <p class="text-sm text-gray-500">
          点击或拖放图片到这里上传
        </p>
        <p class="text-xs text-gray-400 mt-1">
          还可上传 {{ max - images.length }} 张
        </p>
      </div>
    </div>
  </div>
</template>
