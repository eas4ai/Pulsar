<script setup lang="ts">
import { computed, ref } from 'vue'
import { useForm } from '@inertiajs/vue3'
import type { ArticleFormState } from '../../types/inertia-props'

const props = defineProps<{
  article: ArticleFormState
}>()

const activeTab = ref('write')
const isEditing = computed(() => props.article.id !== null)
const submitHref = computed(() =>
  isEditing.value ? `/admin/articles/${props.article.id}` : '/admin/articles',
)

const form = useForm({
  title: props.article.title,
  slug: props.article.slug,
  category: props.article.category,
  tags: props.article.tags,
  status: props.article.status,
  body_markdown: props.article.body_markdown,
})

function submit() {
  if (isEditing.value) {
    form.put(submitHref.value)
  } else {
    form.post(submitHref.value)
  }
}
</script>

<template>
  <v-form class="article-editor" @submit.prevent="submit">
    <div class="article-editor__grid">
      <v-text-field
        v-model="form.title"
        label="Title"
        :error-messages="form.errors.title"
        variant="outlined"
      />
      <v-text-field
        v-model="form.slug"
        label="Slug"
        hint="Leave blank to generate it from the title"
        persistent-hint
        :error-messages="form.errors.slug"
        variant="outlined"
      />
      <v-text-field
        v-model="form.category"
        label="Category"
        :error-messages="form.errors.category"
        variant="outlined"
      />
      <v-text-field
        v-model="form.tags"
        label="Tags"
        hint="Comma separated"
        persistent-hint
        :error-messages="form.errors.tags"
        variant="outlined"
      />
      <v-select
        v-model="form.status"
        label="Status"
        :items="['draft', 'published']"
        :error-messages="form.errors.status"
        variant="outlined"
      />
    </div>

    <v-tabs v-model="activeTab" class="mt-2" color="primary">
      <v-tab value="write">Write</v-tab>
      <v-tab value="preview">Preview</v-tab>

      <v-window v-model="activeTab" class="article-editor__tabs">
        <v-window-item value="write">
          <v-textarea
            v-model="form.body_markdown"
            label="Markdown"
            rows="18"
            auto-grow
            :error-messages="form.errors.body_markdown"
            variant="outlined"
          />
        </v-window-item>
        <v-window-item value="preview">
          <div
            class="docs-content article-editor__preview"
            v-html="article.body_html || '<p>Save the article to render a preview.</p>'"
          />
        </v-window-item>
      </v-window>
    </v-tabs>

    <div class="article-editor__actions">
      <v-btn type="submit" color="primary" :loading="form.processing">
        Save
      </v-btn>
      <v-btn to="/admin/articles" variant="outlined">Cancel</v-btn>
    </div>
  </v-form>
</template>
