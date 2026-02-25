<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import type { MemeInfo } from '../types'
import { getMemeInfos, searchMemes, getImageUrl, getMemePreview } from '../api'
import MemeCard from '../components/MemeCard.vue'

const router = useRouter()
const route = useRoute()

const allMemes = ref<MemeInfo[]>([])
const searchQuery = ref('')
const searchResults = ref<string[] | null>(null)
const loading = ref(true)
const previewCache = ref<Record<string, string>>({})
const selectedTag = ref<string | null>(null)
const sortMode = ref<'default' | 'newest' | 'oldest' | 'az' | 'za'>('default')
const showSortMenu = ref(false)
const sortLabels: Record<string, string> = {
  default: '默认排序',
  newest: '最新创建',
  oldest: '最早创建',
  az: '名称 A→Z',
  za: '名称 Z→A',
}
const TAG_DISPLAY_COUNT = 6
const displayedTags = ref<string[]>([])
const tagFading = ref(false)

function shuffleTags() {
  const all = allTags.value
  // Keep selected tag if any, fill rest randomly
  const pool = selectedTag.value ? all.filter((t) => t !== selectedTag.value) : [...all]
  // Fisher-Yates shuffle
  for (let i = pool.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1))
    ;[pool[i], pool[j]] = [pool[j], pool[i]]
  }
  const picked = pool.slice(0, selectedTag.value ? TAG_DISPLAY_COUNT - 1 : TAG_DISPLAY_COUNT)
  if (selectedTag.value) picked.unshift(selectedTag.value)
  displayedTags.value = picked
}

function refreshTags() {
  tagFading.value = true
  setTimeout(() => {
    shuffleTags()
    tagFading.value = false
  }, 200)
}

// Collect all tags
const allTags = computed(() => {
  const tagSet = new Set<string>()
  allMemes.value.forEach((m) => m.tags.forEach((t) => tagSet.add(t)))
  return Array.from(tagSet).sort()
})

const displayedMemes = computed(() => {
  let memes = [...allMemes.value]

  if (searchResults.value !== null) {
    const keySet = new Set(searchResults.value)
    memes = memes.filter((m) => keySet.has(m.key))
    // Sort by search result order when in search mode
    if (sortMode.value === 'default') {
      memes.sort(
        (a, b) => searchResults.value!.indexOf(a.key) - searchResults.value!.indexOf(b.key),
      )
    }
  }

  if (selectedTag.value) {
    memes = memes.filter((m) => m.tags.includes(selectedTag.value!))
  }

  // Apply sorting
  switch (sortMode.value) {
    case 'newest':
      memes.sort((a, b) => new Date(b.date_created).getTime() - new Date(a.date_created).getTime())
      break
    case 'oldest':
      memes.sort((a, b) => new Date(a.date_created).getTime() - new Date(b.date_created).getTime())
      break
    case 'az':
      memes.sort((a, b) => (a.keywords[0] || a.key).localeCompare(b.keywords[0] || b.key, 'zh-CN'))
      break
    case 'za':
      memes.sort((a, b) => (b.keywords[0] || b.key).localeCompare(a.keywords[0] || a.key, 'zh-CN'))
      break
  }

  return memes
})

let searchTimeout: ReturnType<typeof setTimeout> | null = null
watch(searchQuery, (val) => {
  if (searchTimeout) clearTimeout(searchTimeout)
  if (!val.trim()) {
    searchResults.value = null
    return
  }
  searchTimeout = setTimeout(async () => {
    try {
      searchResults.value = await searchMemes(val.trim(), true)
    } catch {
      searchResults.value = null
    }
  }, 300)
})

function selectTag(tag: string) {
  selectedTag.value = selectedTag.value === tag ? null : tag
}

function selectSort(key: string) {
  sortMode.value = key as typeof sortMode.value
  showSortMenu.value = false
}

function goToMeme(key: string) {
  router.push({ name: 'meme', params: { memeKey: key } })
}

function goToRandomMeme() {
  const pool = displayedMemes.value.length ? displayedMemes.value : allMemes.value
  if (!pool.length) return
  const pick = pool[Math.floor(Math.random() * pool.length)]
  router.push({ name: 'meme', params: { memeKey: pick.key } })
}

async function loadPreview(key: string) {
  if (previewCache.value[key]) return
  try {
    const resp = await getMemePreview(key)
    previewCache.value[key] = getImageUrl(resp.image_id)
  } catch {
    // ignore
  }
}

onMounted(async () => {
  try {
    allMemes.value = await getMemeInfos()
  } catch (err) {
    console.error('Failed to load memes:', err)
  } finally {
    loading.value = false
  }
  // Check for tag query param
  const tagParam = route.query.tag as string | undefined
  if (tagParam && allTags.value.includes(tagParam)) {
    selectedTag.value = tagParam
  }
  // Initialize random tags
  shuffleTags()
  // Close sort menu on outside click
  document.addEventListener('click', onClickOutsideSortMenu)
})

function onClickOutsideSortMenu(e: MouseEvent) {
  if (showSortMenu.value) {
    const target = e.target as HTMLElement
    if (!target.closest('.relative')) {
      showSortMenu.value = false
    }
  }
}

onUnmounted(() => {
  document.removeEventListener('click', onClickOutsideSortMenu)
})
</script>

<template>
  <div class="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
    <!-- Hero section -->
    <div class="mb-10 text-center">
      <h2 class="mb-3 text-3xl font-bold text-gray-900">选择一个模板开始制作</h2>
      <p class="text-lg text-gray-500">从 {{ allMemes.length }} 个表情包模板中挑选</p>
    </div>

    <!-- Search bar -->
    <div class="mx-auto mb-8 max-w-2xl">
      <div class="flex gap-1">
        <div class="relative flex-1">
          <svg
            class="absolute left-3.5 top-1/2 h-5 w-5 -translate-y-1/2 text-gray-400"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
            />
          </svg>
          <input
            v-model="searchQuery"
            type="text"
            placeholder="搜索表情包（关键词、标签）..."
            class="input w-full rounded-xl py-3 pl-11 pr-10 text-base"
          />
          <button
            v-if="searchQuery"
            @click="searchQuery = ''"
            class="absolute right-3 top-1/2 -translate-y-1/2 text-gray-400 hover:text-gray-600"
          >
            <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M6 18L18 6M6 6l12 12"
              />
            </svg>
          </button>
        </div>
        <button
          @click="goToRandomMeme"
          class="dice-btn flex h-12 w-12 shrink-0 items-center justify-center rounded-xl text-gray-400 transition-colors duration-200 hover:text-gray-700"
          title="随机表情"
        >
          <svg
            class="h-5 w-5"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          >
            <path d="M16 3h5v5" />
            <path d="M4 20L21 3" />
            <path d="M21 16v5h-5" />
            <path d="M15 15l6 6" />
            <path d="M4 4l5 5" />
          </svg>
        </button>
      </div>
    </div>

    <!-- Tags filter -->
    <div v-if="allTags.length" class="mb-8 flex items-center justify-center gap-2">
      <div
        class="flex items-center gap-2 transition-opacity duration-200"
        :class="tagFading ? 'opacity-0' : 'opacity-100'"
      >
        <button
          v-for="tag in displayedTags"
          :key="tag"
          @click="selectTag(tag)"
          :class="[
            'whitespace-nowrap rounded-lg px-4 py-1.5 text-[13px] font-medium transition-colors duration-150',
            selectedTag === tag
              ? 'bg-gray-900 text-white'
              : 'bg-gray-100 text-gray-600 hover:bg-gray-200',
          ]"
        >
          {{ tag }}
        </button>
      </div>
      <button
        @click="refreshTags"
        class="flex h-8 w-8 shrink-0 items-center justify-center rounded-lg bg-gray-100 text-gray-400 transition-all duration-200 hover:bg-gray-200 hover:text-gray-600"
        title="换一批"
      >
        <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M4 4v5h4.586M20 20v-5h-4.586M4.929 9A9 9 0 0119.071 9M19.071 15A9 9 0 014.929 15"
          />
        </svg>
      </button>
    </div>

    <!-- Sort bar -->
    <div class="mb-6 flex items-center justify-end">
      <div class="relative">
        <button
          @click="showSortMenu = !showSortMenu"
          class="flex items-center gap-2 rounded-lg px-3 py-1.5 text-sm text-gray-600 transition-colors hover:bg-gray-100"
        >
          <svg class="h-4 w-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M3 4h13M3 8h9m-9 4h6m4 0l4-4m0 0l4 4m-4-4v12"
            />
          </svg>
          <span>{{ sortLabels[sortMode] }}</span>
          <svg
            class="h-3.5 w-3.5 text-gray-400 transition-transform"
            :class="showSortMenu ? 'rotate-180' : ''"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M19 9l-7 7-7-7"
            />
          </svg>
        </button>
        <Transition name="fade">
          <div
            v-if="showSortMenu"
            class="absolute right-0 top-full z-30 mt-1 w-36 rounded-lg border border-gray-200 bg-white py-1 shadow-lg"
          >
            <button
              v-for="(label, key) in sortLabels"
              :key="key"
              @click="selectSort(key as string)"
              :class="[
                'w-full px-3 py-2 text-left text-sm transition-colors',
                sortMode === key
                  ? 'bg-primary-50 font-medium text-primary-600'
                  : 'text-gray-600 hover:bg-gray-50',
              ]"
            >
              {{ label }}
            </button>
          </div>
        </Transition>
      </div>
    </div>

    <!-- Loading -->
    <div v-if="loading" class="flex justify-center py-20">
      <div class="flex items-center gap-3 text-gray-500">
        <svg class="h-5 w-5 animate-spin" viewBox="0 0 24 24" fill="none">
          <circle
            class="opacity-25"
            cx="12"
            cy="12"
            r="10"
            stroke="currentColor"
            stroke-width="4"
          />
          <path
            class="opacity-75"
            fill="currentColor"
            d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
          />
        </svg>
        <span>加载中...</span>
      </div>
    </div>

    <!-- Empty state -->
    <div v-else-if="displayedMemes.length === 0" class="py-20 text-center">
      <div class="mb-4 text-5xl">🔍</div>
      <h3 class="mb-2 text-lg font-medium text-gray-900">没有找到匹配的表情包</h3>
      <p class="text-gray-500">尝试使用其他关键词搜索</p>
    </div>

    <!-- Meme grid -->
    <div
      v-else
      class="grid grid-cols-2 gap-4 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6"
    >
      <MemeCard
        v-for="meme in displayedMemes"
        :key="meme.key"
        :meme="meme"
        :preview-url="previewCache[meme.key]"
        @click="goToMeme(meme.key)"
        @load-preview="loadPreview(meme.key)"
      />
    </div>

    <!-- Result count -->
    <div
      v-if="!loading && displayedMemes.length > 0"
      class="mt-8 text-center text-sm text-gray-400"
    >
      {{ displayedMemes.length }} 个结果
    </div>
  </div>
</template>
