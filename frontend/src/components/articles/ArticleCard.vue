<script setup lang="ts">
defineProps<{
  article: ArticleSummary
}>()
</script>

<template>
  <v-card class="article-card" :to="`/blog/${article.slug}`" variant="flat">
    <v-card-item>
      <div class="article-card__meta">
        <span>{{ article.category }}</span>
        <span>{{ new Date(article.published_at).toLocaleDateString() }}</span>
      </div>
      <v-card-title>{{ article.title }}</v-card-title>
      <v-card-subtitle>{{ article.excerpt }}</v-card-subtitle>
    </v-card-item>

    <v-card-text>
      <div class="article-card__tags">
        <v-chip
          v-for="tag in article.tags"
          :key="tag"
          size="small"
          variant="tonal"
          color="primary"
        >
          {{ tag }}
        </v-chip>
      </div>
    </v-card-text>
  </v-card>
</template>
