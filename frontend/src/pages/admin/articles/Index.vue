<script setup lang="ts">
import type { AdminArticlesIndexProps } from '../../../types/inertia-props'

defineProps<AdminArticlesIndexProps>()
</script>

<template>
  <v-container class="console-page">
    <header class="console-hero">
      <div>
        <div class="console-eyebrow">Articles</div>
        <h1>Publishing queue</h1>
        <p>Create drafts, review rendered content, and publish first-party Pulsar articles.</p>
      </div>
      <v-btn to="/admin/articles/create" color="primary" prepend-icon="mdi-plus">
        New article
      </v-btn>
    </header>

    <v-card class="article-admin-table mt-8" variant="flat">
      <v-table>
        <thead>
          <tr>
            <th>Title</th>
            <th>Status</th>
            <th>Category</th>
            <th>Updated</th>
            <th />
          </tr>
        </thead>
        <tbody>
          <tr v-for="article in articles" :key="article.id">
            <td>
              <strong>{{ article.title }}</strong>
              <div class="text-medium-emphasis">{{ article.slug }}</div>
            </td>
            <td>{{ article.status }}</td>
            <td>{{ article.category }}</td>
            <td>{{ new Date(article.updated_at).toLocaleString() }}</td>
            <td class="text-right">
              <v-btn
                :to="`/admin/articles/${article.id}/edit`"
                icon="mdi-pencil-outline"
                variant="text"
                aria-label="Edit article"
              />
            </td>
          </tr>
        </tbody>
      </v-table>
    </v-card>
  </v-container>
</template>
