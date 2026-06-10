<script setup lang="ts">
import type { TopicsIndexProps } from '../../types/inertia-props'

defineProps<TopicsIndexProps>()
</script>

<template>
  <v-container class="taxonomy-page">
    <header class="console-hero">
      <div>
        <div class="console-eyebrow">Topics</div>
        <h1>Public topics</h1>
        <p>Browse visible Pulsar topics and the work attached to each area.</p>
      </div>
    </header>

    <div v-if="topics.length" class="taxonomy-grid">
      <v-card
        v-for="topic in topics"
        :key="topic.slug"
        class="taxonomy-card"
        :to="`/topics/${topic.slug}`"
        variant="flat"
      >
        <v-card-item>
          <template #prepend>
            <v-avatar color="primary" variant="tonal">
              <v-icon icon="mdi-shape-outline" />
            </v-avatar>
          </template>
          <v-card-title>{{ topic.name }}</v-card-title>
          <v-card-subtitle>{{ topic.slug }}</v-card-subtitle>
        </v-card-item>
        <v-card-text>
          <p>{{ topic.description || 'No description yet.' }}</p>
          <div class="taxonomy-card__counts">
            <span>{{ topic.contribution_counts.articles }} articles</span>
            <span>{{ topic.contribution_counts.resources }} resources</span>
            <span>{{ topic.contribution_counts.questions }} questions</span>
          </div>
        </v-card-text>
      </v-card>
    </div>

    <v-alert v-else class="mt-6" type="info" variant="tonal">
      No topics are visible yet.
    </v-alert>
  </v-container>
</template>
