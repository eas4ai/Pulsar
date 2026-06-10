<script setup lang="ts">
import ArticleCard from '../../components/articles/ArticleCard.vue'
import type { ArticlesIndexProps } from '../../types/inertia-props'

defineProps<ArticlesIndexProps>()
</script>

<template>
  <v-container class="article-page">
    <header class="console-hero">
      <div>
        <div class="console-eyebrow">Blog</div>
        <h1>Articles and release notes</h1>
        <p>
          First-party updates, implementation notes, and product guidance from
          the Pulsar starter kit.
        </p>
      </div>
      <v-btn to="/feed.xml" variant="outlined" prepend-icon="mdi-rss">
        RSS
      </v-btn>
    </header>

    <div class="article-grid">
      <ArticleCard
        v-for="article in articles"
        :key="article.slug"
        :article="article"
      />
    </div>
  </v-container>
</template>
