<script setup lang="ts">
import { router } from '@inertiajs/vue3'
import ArticleEditor from '../../../components/articles/ArticleEditor.vue'
import type { AdminArticleEditProps } from '../../../types/inertia-props'

const props = defineProps<AdminArticleEditProps>()

function publish() {
  if (props.article.id !== null) {
    router.post(`/admin/articles/${props.article.id}/publish`)
  }
}
</script>

<template>
  <v-container class="console-page">
    <header class="console-hero">
      <div>
        <div class="console-eyebrow">Editor</div>
        <h1>{{ article.id === null ? 'New article' : article.title }}</h1>
        <p>Write in Markdown, preview rendered output, and publish when ready.</p>
      </div>
      <v-btn
        v-if="article.id !== null"
        color="primary"
        prepend-icon="mdi-send-outline"
        @click="publish"
      >
        Publish
      </v-btn>
    </header>

    <v-card class="profile-panel mt-8" variant="flat">
      <v-card-text>
        <ArticleEditor :article="article" />
      </v-card-text>
    </v-card>
  </v-container>
</template>
