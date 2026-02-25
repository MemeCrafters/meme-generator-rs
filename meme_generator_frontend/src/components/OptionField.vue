<script setup lang="ts">
import type { MemeOption } from '../types'

const props = defineProps<{
  option: MemeOption
  modelValue: any
  enabled: boolean
}>()

const emit = defineEmits<{
  'update:modelValue': [value: any]
  'update:enabled': [value: boolean]
}>()

function onInput(value: any) {
  if (!props.enabled) {
    emit('update:enabled', true)
  }
  emit('update:modelValue', value)
}

function toggleEnabled() {
  emit('update:enabled', !props.enabled)
}
</script>

<template>
  <div>
    <label class="flex items-center gap-2 text-sm font-medium text-gray-700 mb-1 cursor-pointer select-none">
      <input
        type="checkbox"
        :checked="enabled"
        @change="toggleEnabled"
        class="w-4 h-4 rounded border-gray-300 text-primary-500 focus:ring-primary-500 cursor-pointer"
      />
      {{ option.description || option.name }}
      <span v-if="!enabled" class="text-xs text-gray-400 font-normal">（未指定）</span>
    </label>

    <div :class="{ 'opacity-40 pointer-events-none': !enabled }">
    <!-- Boolean -->
    <div v-if="option.type === 'boolean'" class="flex items-center">
      <button
        @click="onInput(!modelValue)"
        :class="[
          'relative inline-flex h-6 w-11 items-center rounded-full transition-colors duration-200',
          modelValue ? 'bg-primary-500' : 'bg-gray-300',
        ]"
      >
        <span
          :class="[
            'inline-block h-4 w-4 rounded-full bg-white transition-transform duration-200 shadow-sm',
            modelValue ? 'translate-x-6' : 'translate-x-1',
          ]"
        />
      </button>
      <span class="ml-2 text-sm text-gray-600">{{ modelValue ? '开启' : '关闭' }}</span>
    </div>

    <!-- String with choices -->
    <select
      v-else-if="option.type === 'string' && option.choices && option.choices.length > 0"
      :value="modelValue"
      @change="onInput(($event.target as HTMLSelectElement).value)"
      class="input"
    >
      <option value="">请选择</option>
      <option v-for="choice in option.choices" :key="choice" :value="choice">
        {{ choice }}
      </option>
    </select>

    <!-- String without choices -->
    <input
      v-else-if="option.type === 'string'"
      type="text"
      :value="modelValue"
      @input="onInput(($event.target as HTMLInputElement).value)"
      class="input"
      :placeholder="option.default || ''"
    />

    <!-- Integer -->
    <div v-else-if="option.type === 'integer'" class="flex items-center gap-3">
      <input
        type="range"
        :value="modelValue"
        @input="onInput(Number(($event.target as HTMLInputElement).value))"
        :min="option.minimum ?? 0"
        :max="option.maximum ?? 100"
        step="1"
        class="flex-1 h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer accent-primary-500"
      />
      <input
        type="number"
        :value="modelValue"
        @input="onInput(Number(($event.target as HTMLInputElement).value))"
        :min="option.minimum ?? undefined"
        :max="option.maximum ?? undefined"
        class="input w-20 text-center"
      />
    </div>

    <!-- Float -->
    <div v-else-if="option.type === 'float'" class="flex items-center gap-3">
      <input
        type="range"
        :value="modelValue"
        @input="onInput(Number(($event.target as HTMLInputElement).value))"
        :min="option.minimum ?? 0"
        :max="option.maximum ?? 1"
        step="0.01"
        class="flex-1 h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer accent-primary-500"
      />
      <input
        type="number"
        :value="modelValue"
        @input="onInput(Number(($event.target as HTMLInputElement).value))"
        :min="option.minimum ?? undefined"
        :max="option.maximum ?? undefined"
        step="0.01"
        class="input w-20 text-center"
      />
    </div>
    </div>
  </div>
</template>
