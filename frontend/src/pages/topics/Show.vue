<script setup lang="ts">
import { computed } from 'vue'
import type { TopicShowProps } from '../../types/inertia-props'

const props = defineProps<TopicShowProps>()

const counts = computed(() => [
  {
    label: 'Articles',
    value: props.topic.contribution_counts.articles,
    icon: 'mdi-file-document-outline',
  },
  {
    label: 'Resources',
    value: props.topic.contribution_counts.resources,
    icon: 'mdi-library-outline',
  },
  {
    label: 'Questions',
    value: props.topic.contribution_counts.questions,
    icon: 'mdi-comment-question-outline',
  },
])
</script>

<template>
  <v-container class="taxonomy-page taxonomy-page--show">
    <v-btn to="/topics" variant="text" prepend-icon="mdi-arrow-left" class="mb-4">
      Topics
    </v-btn>

    <article class="taxonomy-detail">
      <header class="taxonomy-detail__header">
        <v-avatar color="primary" variant="tonal" size="76">
          <v-icon icon="mdi-shape-outline" size="42" />
        </v-avatar>
        <div>
          <div class="console-eyebrow">Topic</div>
          <h1>{{ topic.name }}</h1>
          <p>{{ topic.slug }}</p>
        </div>
      </header>

      <p class="taxonomy-detail__description">
        {{ topic.description || 'No description yet.' }}
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
          No contributions are attached to this topic yet.
        </v-alert>
      </section>
    </article>
  </v-container>
</template>
