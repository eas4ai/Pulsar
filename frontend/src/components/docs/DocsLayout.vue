<script setup lang="ts">
import { computed } from 'vue'
import DocsSearch from './DocsSearch.vue'
import DocsToc from './DocsToc.vue'

const props = defineProps<{
  catalog: DocsCatalog
  chapter: DocsChapter
}>()

const activeIndex = computed(() =>
  props.catalog.chapters.findIndex((entry) => entry.slug === props.chapter.slug),
)

const progressLabel = computed(() => {
  const current = activeIndex.value + 1
  return current > 0 ? `${current} of ${props.catalog.chapters.length}` : 'Docs'
})

const previousHref = computed(() =>
  props.chapter.previous ? `/docs/${props.chapter.previous}` : null,
)
const nextHref = computed(() =>
  props.chapter.next ? `/docs/${props.chapter.next}` : null,
)
</script>

<template>
  <v-container class="docs-page">
    <div class="docs-grid">
      <aside class="docs-sidebar" aria-label="Documentation navigation">
        <DocsSearch :entries="catalog.search" />

        <nav class="docs-chapters">
          <div class="docs-section-label">Guides</div>
          <v-list nav density="compact" class="docs-chapters__list">
            <v-list-item
              v-for="entry in catalog.chapters"
              :key="entry.slug"
              :to="`/docs/${entry.slug}`"
              :title="entry.title"
              :subtitle="entry.excerpt"
              :active="entry.slug === chapter.slug"
              rounded="lg"
            />
          </v-list>
        </nav>
      </aside>

      <main class="docs-main">
        <div class="docs-article">
          <div class="docs-article__header">
            <div>
              <div class="docs-section-label">{{ progressLabel }}</div>
              <h1>{{ chapter.title }}</h1>
              <p>{{ chapter.excerpt }}</p>
            </div>
          </div>

          <div class="docs-content" v-html="chapter.html" />

          <div class="docs-pagination">
            <v-btn
              v-if="previousHref"
              :to="previousHref"
              variant="outlined"
              prepend-icon="mdi-arrow-left"
            >
              Previous
            </v-btn>
            <span v-else />
            <v-btn
              v-if="nextHref"
              :to="nextHref"
              color="primary"
              append-icon="mdi-arrow-right"
            >
              Next
            </v-btn>
          </div>
        </div>
      </main>

      <DocsToc :headings="chapter.headings" />
    </div>
  </v-container>
</template>
