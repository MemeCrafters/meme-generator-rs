<script setup lang="ts">
import { ref, onMounted } from 'vue'
import type { MemeInfo } from '../types'

const props = defineProps<{
  meme: MemeInfo
  previewUrl?: string
}>()

const emit = defineEmits<{
  click: []
  loadPreview: []
}>()

const visible = ref(false)
const cardRef = ref<HTMLElement>()

onMounted(() => {
  if (!cardRef.value) return
  const observer = new IntersectionObserver(
    (entries) => {
      if (entries[0].isIntersecting) {
        visible.value = true
        emit('loadPreview')
        observer.disconnect()
      }
    },
    { rootMargin: '200px' }
  )
  observer.observe(cardRef.value)
})
</script>

<template>
  <div
    ref="cardRef"
    @click="emit('click')"
    class="card cursor-pointer group overflow-hidden"
  >
    <!-- Preview Image -->
    <div class="aspect-square bg-gray-50 relative overflow-hidden">
      <Transition name="fade">
        <img
          v-if="previewUrl"
          :src="previewUrl"
          :alt="meme.key"
          class="w-full h-full object-contain p-2"
          loading="lazy"
        />
        <div v-else class="w-full h-full flex items-center justify-center">
          <div class="text-center">
            <div class="text-3xl mb-1">🖼️</div>
            <span class="text-xs text-gray-400">点击预览</span>
          </div>
        </div>
      </Transition>
      <!-- Hover overlay -->
      <div
        class="absolute inset-0 bg-primary-500/0 group-hover:bg-primary-500/5 transition-colors duration-200"
      />
    </div>

    <!-- Info -->
    <div class="p-3">
      <h3 class="text-sm font-medium text-gray-900 truncate mb-1">
        {{ meme.keywords[0] || meme.key }}
      </h3>
      <div class="flex items-center gap-1 flex-wrap">
        <span
          v-if="meme.params.min_images > 0 || meme.params.max_images > 0"
          class="badge-blue text-[10px]"
        >
          <template v-if="meme.params.min_images === meme.params.max_images">{{ meme.params.min_images }} 图</template>
          <template v-else>{{ meme.params.min_images }}-{{ meme.params.max_images }} 图</template>
        </span>
        <span
          v-if="meme.params.min_texts > 0 || meme.params.max_texts > 0"
          class="badge-green text-[10px]"
        >
          <template v-if="meme.params.min_texts === meme.params.max_texts">{{ meme.params.min_texts }} 文</template>
          <template v-else>{{ meme.params.min_texts }}-{{ meme.params.max_texts }} 文</template>
        </span>
      </div>
    </div>
  </div>
</template>
