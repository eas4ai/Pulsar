<script setup lang="ts">
import DocsLayout from '../../components/docs/DocsLayout.vue'
import type { DocsPageProps } from '../../types/inertia-props'

defineProps<DocsPageProps>()
</script>

<template>
  <DocsLayout :catalog="catalog" :chapter="chapter" />
</template>
