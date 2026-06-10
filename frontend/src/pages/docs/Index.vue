<script setup lang="ts">
import DocsLayout from '../../components/docs/DocsLayout.vue'
import type { DocsProps } from '../../types/inertia-props'

defineProps<DocsProps>()
</script>

<template>
  <DocsLayout :catalog="catalog" :chapter="chapter" />
</template>
