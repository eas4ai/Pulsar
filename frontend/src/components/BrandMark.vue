<script setup lang="ts">
import { computed } from 'vue'

const props = withDefaults(
  defineProps<{
    showText?: boolean
    decorative?: boolean
    direction?: 'row' | 'column'
    size?: number | string
  }>(),
  {
    showText: false,
    decorative: true,
    direction: 'column',
    size: 40,
  },
)

const iconSize = computed(() =>
  typeof props.size === 'number' ? `${props.size}px` : props.size,
)
const markSrc = '/android-chrome-512x512.png'
</script>

<template>
  <span
    class="brand-mark"
    :class="[`brand-mark--${direction}`, { 'brand-mark--with-text': showText }]"
    :aria-hidden="decorative ? 'true' : undefined"
  >
    <span class="brand-mark__symbol" :style="{ width: iconSize, height: iconSize }">
      <img :src="markSrc" alt="" />
    </span>
    <span v-if="showText" class="brand-mark__word">Pulsar</span>
  </span>
</template>
