<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  headings: DocsHeading[]
}>()

const visibleHeadings = computed(() =>
  props.headings.filter((heading) => heading.level > 1 && heading.level <= 3),
)
</script>

<template>
  <aside class="docs-toc" aria-label="Current page">
    <div class="docs-section-label">On this page</div>
    <a
      v-for="heading in visibleHeadings"
      :key="heading.id"
      :href="`#${heading.id}`"
      :class="['docs-toc__link', `docs-toc__link--level-${heading.level}`]"
    >
      {{ heading.title }}
    </a>
  </aside>
</template>
