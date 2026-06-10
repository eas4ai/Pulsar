<script setup lang="ts">
import { computed } from 'vue'
import type { TagShowProps } from '../../types/inertia-props'

const props = defineProps<TagShowProps>()

const counts = computed(() => [
  {
    label: 'Articles',
    value: props.tag.contribution_counts.articles,
    icon: 'mdi-file-document-outline',
  },
  {
    label: 'Resources',
    value: props.tag.contribution_counts.resources,
    icon: 'mdi-library-outline',
  },
  {
    label: 'Questions',
    value: props.tag.contribution_counts.questions,
    icon: 'mdi-comment-question-outline',
  },
])
</script>

<template>
  <v-container class="taxonomy-page taxonomy-page--show">
    <article class="taxonomy-detail">
      <header class="taxonomy-detail__header">
        <v-avatar color="primary" variant="tonal" size="76">
          <v-icon icon="mdi-tag-outline" size="42" />
        </v-avatar>
        <div>
          <div class="console-eyebrow">Tag</div>
          <h1>{{ tag.name }}</h1>
          <p>{{ tag.slug }}</p>
        </div>
      </header>

      <p class="taxonomy-detail__description">
        No description is stored for tags.
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
          No contributions are attached to this tag yet.
        </v-alert>
      </section>
    </article>
  </v-container>
</template>
