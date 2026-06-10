<script setup lang="ts">
import { computed, ref } from 'vue'
import { router } from '@inertiajs/vue3'
import type { DocsSearchEntry } from '../../types/inertia-props'

const props = defineProps<{
  entries: DocsSearchEntry[]
}>()

const query = ref('')
const normalizedQuery = computed(() => query.value.trim().toLowerCase())

const results = computed(() => {
  const needle = normalizedQuery.value
  if (needle.length < 2) {
    return []
  }

  return props.entries
    .filter((entry) => {
      const haystack = [
        entry.title,
        entry.excerpt,
        entry.plain_text,
        ...entry.headings.map((heading) => heading.title),
      ]
        .join(' ')
        .toLowerCase()
      return haystack.includes(needle)
    })
    .slice(0, 6)
})

function navigate(slug: string) {
  query.value = ''
  router.visit(`/docs/${slug}`)
}

function submitFirstResult() {
  const first = results.value[0]
  if (first) {
    navigate(first.slug)
  }
}
</script>

<template>
  <div class="docs-search">
    <v-text-field
      v-model="query"
      label="Search docs"
      prepend-inner-icon="mdi-magnify"
      variant="outlined"
      density="compact"
      hide-details
      clearable
      @keydown.enter.prevent="submitFirstResult"
    />

    <v-list v-if="results.length" density="compact" class="docs-search__results">
      <v-list-item
        v-for="result in results"
        :key="result.slug"
        :title="result.title"
        :subtitle="result.excerpt"
        prepend-icon="mdi-file-document-outline"
        rounded="lg"
        @click="navigate(result.slug)"
      />
    </v-list>
  </div>
</template>
