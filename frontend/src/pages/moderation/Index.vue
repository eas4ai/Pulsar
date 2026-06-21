<script setup lang="ts">
import type { ModerationIndexProps } from '../../types/inertia-props'

defineProps<ModerationIndexProps>()
</script>

<template>
  <v-container class="console-page">
    <header class="console-hero">
      <div>
        <div class="console-eyebrow">Moderation</div>
        <h1>Review queue</h1>
        <p>First-party content awaiting review before it can be published.</p>
      </div>
    </header>

    <v-card class="moderation-table mt-8" variant="flat">
      <v-table>
        <thead>
          <tr>
            <th>Title</th>
            <th>Status</th>
            <th>Author</th>
            <th>Updated</th>
            <th />
          </tr>
        </thead>
        <tbody>
          <tr v-for="item in items" :key="item.id">
            <td>
              <strong>{{ item.title }}</strong>
              <div class="text-medium-emphasis">{{ item.slug }}</div>
            </td>
            <td>{{ item.status }}</td>
            <td>{{ item.author_id }}</td>
            <td>{{ new Date(item.updated_at).toLocaleString() }}</td>
            <td class="text-right">
              <v-btn
                :to="`/admin/articles/${item.id}/edit`"
                icon="mdi-pencil-outline"
                variant="text"
                aria-label="Review article"
              />
            </td>
          </tr>
        </tbody>
      </v-table>
    </v-card>
  </v-container>
</template>
