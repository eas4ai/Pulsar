<script setup lang="ts">
import { computed } from 'vue'
import { Head } from '@inertiajs/vue3'

const props = withDefaults(
  defineProps<{
    title: string
    suffix?: boolean
  }>(),
  { suffix: true },
)

const siteName = import.meta.env.VITE_APP_NAME || 'Pulsar'
const fullTitle = computed(() =>
  props.suffix ? `${props.title} · ${siteName}` : props.title,
)
</script>

<template>
  <Head>
    <title>{{ fullTitle }}</title>
  </Head>
</template>
