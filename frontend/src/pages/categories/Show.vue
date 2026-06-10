<script setup lang="ts">
import { computed } from 'vue'
import type { CategoryShowProps } from '../../types/inertia-props'

const props = defineProps<CategoryShowProps>()

const counts = computed(() => [
  {
    label: 'Articles',
    value: props.category.contribution_counts.articles,
    icon: 'mdi-file-document-outline',
  },
  {
    label: 'Resources',
    value: props.category.contribution_counts.resources,
    icon: 'mdi-library-outline',
  },
  {
    label: 'Questions',
    value: props.category.contribution_counts.questions,
    icon: 'mdi-comment-question-outline',
  },
])
</script>

<template>
  <v-container class="taxonomy-page taxonomy-page--show">
    <article class="taxonomy-detail">
      <header class="taxonomy-detail__header">
        <v-avatar color="primary" variant="tonal" size="76">
          <v-icon icon="mdi-folder-outline" size="42" />
        </v-avatar>
        <div>
          <div class="console-eyebrow">Category</div>
          <h1>{{ category.name }}</h1>
          <p>{{ category.slug }}</p>
        </div>
      </header>

      <p class="taxonomy-detail__description">
        {{ category.description || 'No description yet.' }}
      </p>

      <div class="taxonomy-counts">
        <div v-for="count in counts" :key="count.label" class="taxonomy-counts__item">
          <v-icon :icon="count.icon" />
          <strong>{{ count.value }}</strong>
          <span>{{ count.label }}</span>
        </div>
      </div>

      <section class="taxonomy-contributions">
        <div class="console-eyebrow">Contributions</div>
        <v-alert class="mt-3" type="info" variant="tonal">
          No contributions are attached to this category yet.
        </v-alert>
      </section>
    </article>
  </v-container>
</template>
