<script setup lang="ts">
import type { ArticleShowProps } from '../../types/inertia-props'

defineProps<ArticleShowProps>()
</script>

<template>
  <v-container class="article-page article-page--show">
    <v-btn to="/blog" variant="text" prepend-icon="mdi-arrow-left" class="mb-4">
      Blog
    </v-btn>

    <article class="docs-article">
      <header class="docs-article__header">
        <div class="article-card__meta">
          <span>{{ article.category }}</span>
          <span>{{ new Date(article.published_at).toLocaleDateString() }}</span>
        </div>
        <h1>{{ article.title }}</h1>
        <p>{{ article.excerpt }}</p>
        <div class="article-card__tags">
          <v-chip
            v-for="tag in article.tags"
            :key="tag"
            size="small"
            color="primary"
            variant="tonal"
          >
            {{ tag }}
          </v-chip>
        </div>
      </header>

      <div class="docs-content" v-html="article.body_html" />
    </article>
  </v-container>
</template>
