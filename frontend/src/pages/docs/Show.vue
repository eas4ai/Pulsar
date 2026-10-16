<script setup lang="ts">
import DocsLayout from '../../components/docs/DocsLayout.vue'
import SiteHead from '../../components/SiteHead.vue'
import type { DocsPageProps } from '../../types/inertia-props'

defineProps<DocsPageProps>()
</script>

<template>
  <SiteHead :title="chapter.title" />
  <DocsLayout :catalog="catalog" :chapter="chapter" />
</template>
