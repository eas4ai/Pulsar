<script setup lang="ts">
import { ref } from 'vue'
import { router, useForm } from '@inertiajs/vue3'
import type { AdminTaxonomyProps } from '../../../types/inertia-props'

defineProps<AdminTaxonomyProps>()

const activeSection = ref('categories')
const editingCategoryId = ref<number | null>(null)
const editingTopicId = ref<number | null>(null)
const editingTagId = ref<number | null>(null)

const categoryForm = useForm({
  name: '',
  slug: '',
  description: '',
  sort_order: 0,
  is_visible: true,
})

const topicForm = useForm({
  name: '',
  slug: '',
  description: '',
  sort_order: 0,
  is_visible: true,
})

const tagForm = useForm({
  name: '',
  slug: '',
})

function resetCategoryForm() {
  editingCategoryId.value = null
  categoryForm.clearErrors()
  categoryForm.name = ''
  categoryForm.slug = ''
  categoryForm.description = ''
  categoryForm.sort_order = 0
  categoryForm.is_visible = true
}

function resetTopicForm() {
  editingTopicId.value = null
  topicForm.clearErrors()
  topicForm.name = ''
  topicForm.slug = ''
  topicForm.description = ''
  topicForm.sort_order = 0
  topicForm.is_visible = true
}

function resetTagForm() {
  editingTagId.value = null
  tagForm.clearErrors()
  tagForm.name = ''
  tagForm.slug = ''
}

function editCategory(category: AdminTaxonomyTermRow) {
  editingCategoryId.value = category.id
  categoryForm.clearErrors()
  categoryForm.name = category.name
  categoryForm.slug = category.slug
  categoryForm.description = category.description ?? ''
  categoryForm.sort_order = category.sort_order
  categoryForm.is_visible = category.is_visible
}

function editTopic(topic: AdminTaxonomyTermRow) {
  editingTopicId.value = topic.id
  topicForm.clearErrors()
  topicForm.name = topic.name
  topicForm.slug = topic.slug
  topicForm.description = topic.description ?? ''
  topicForm.sort_order = topic.sort_order
  topicForm.is_visible = topic.is_visible
}

function editTag(tag: AdminTagRow) {
  editingTagId.value = tag.id
  tagForm.clearErrors()
  tagForm.name = tag.name
  tagForm.slug = tag.slug
}

function submitCategory() {
  const options = { preserveScroll: true, onSuccess: resetCategoryForm }
  if (editingCategoryId.value !== null) {
    categoryForm.put(`/admin/categories/${editingCategoryId.value}`, options)
  } else {
    categoryForm.post('/admin/categories', options)
  }
}

function submitTopic() {
  const options = { preserveScroll: true, onSuccess: resetTopicForm }
  if (editingTopicId.value !== null) {
    topicForm.put(`/admin/topics/${editingTopicId.value}`, options)
  } else {
    topicForm.post('/admin/topics', options)
  }
}

function submitTag() {
  const options = { preserveScroll: true, onSuccess: resetTagForm }
  if (editingTagId.value !== null) {
    tagForm.put(`/admin/tags/${editingTagId.value}`, options)
  } else {
    tagForm.post('/admin/tags', options)
  }
}

function deleteCategory(category: AdminTaxonomyTermRow) {
  if (!window.confirm(`Delete category "${category.name}"?`)) {
    return
  }
  router.delete(`/admin/categories/${category.id}`, {
    preserveScroll: true,
    onSuccess: () => {
      if (editingCategoryId.value === category.id) {
        resetCategoryForm()
      }
    },
  })
}

function deleteTopic(topic: AdminTaxonomyTermRow) {
  if (!window.confirm(`Delete topic "${topic.name}"?`)) {
    return
  }
  router.delete(`/admin/topics/${topic.id}`, {
    preserveScroll: true,
    onSuccess: () => {
      if (editingTopicId.value === topic.id) {
        resetTopicForm()
      }
    },
  })
}

function deleteTag(tag: AdminTagRow) {
  if (!window.confirm(`Delete tag "${tag.name}"?`)) {
    return
  }
  router.delete(`/admin/tags/${tag.id}`, {
    preserveScroll: true,
    onSuccess: () => {
      if (editingTagId.value === tag.id) {
        resetTagForm()
      }
    },
  })
}
</script>

<template>
  <v-container class="console-page taxonomy-admin">
    <header class="console-hero">
      <div>
        <div class="console-eyebrow">Taxonomy</div>
        <h1>Categories, topics, and tags</h1>
        <p>Maintain the public navigation vocabulary used across Pulsar V2.</p>
      </div>
    </header>

    <v-card class="taxonomy-admin__workspace mt-8" variant="flat">
      <v-tabs v-model="activeSection" color="primary">
        <v-tab value="categories" prepend-icon="mdi-folder-outline">Categories</v-tab>
        <v-tab value="topics" prepend-icon="mdi-shape-outline">Topics</v-tab>
        <v-tab value="tags" prepend-icon="mdi-tag-outline">Tags</v-tab>
      </v-tabs>

      <v-window v-model="activeSection">
        <v-window-item value="categories">
          <section class="taxonomy-admin__section">
            <v-form class="taxonomy-admin__form" @submit.prevent="submitCategory">
              <div class="taxonomy-admin__form-grid">
                <v-text-field
                  v-model="categoryForm.name"
                  label="Name"
                  :error-messages="categoryForm.errors.name"
                  variant="outlined"
                />
                <v-text-field
                  v-model="categoryForm.slug"
                  label="Slug"
                  :error-messages="categoryForm.errors.slug"
                  variant="outlined"
                />
                <v-text-field
                  v-model.number="categoryForm.sort_order"
                  label="Sort order"
                  type="number"
                  :error-messages="categoryForm.errors.sort_order"
                  variant="outlined"
                />
                <v-switch
                  v-model="categoryForm.is_visible"
                  class="taxonomy-admin__switch"
                  color="primary"
                  label="Visible"
                  hide-details
                />
              </div>
              <v-textarea
                v-model="categoryForm.description"
                label="Description"
                rows="3"
                auto-grow
                :error-messages="categoryForm.errors.description"
                variant="outlined"
              />
              <div class="taxonomy-admin__actions">
                <v-btn type="submit" color="primary" :loading="categoryForm.processing">
                  {{ editingCategoryId === null ? 'Create category' : 'Update category' }}
                </v-btn>
                <v-btn variant="outlined" @click="resetCategoryForm">Cancel</v-btn>
              </div>
            </v-form>

            <v-table class="taxonomy-admin__table">
              <thead>
                <tr>
                  <th>Name</th>
                  <th>Slug</th>
                  <th>Sort</th>
                  <th>Visible</th>
                  <th />
                </tr>
              </thead>
              <tbody>
                <tr v-for="category in categories" :key="category.id">
                  <td>
                    <strong>{{ category.name }}</strong>
                    <div class="text-medium-emphasis">{{ category.description || 'No description' }}</div>
                  </td>
                  <td>{{ category.slug }}</td>
                  <td>{{ category.sort_order }}</td>
                  <td>
                    <v-chip
                      :color="category.is_visible ? 'primary' : 'default'"
                      size="small"
                      variant="tonal"
                    >
                      {{ category.is_visible ? 'Visible' : 'Hidden' }}
                    </v-chip>
                  </td>
                  <td class="text-right">
                    <v-btn
                      icon="mdi-pencil-outline"
                      variant="text"
                      aria-label="Edit category"
                      @click="editCategory(category)"
                    />
                    <v-btn
                      icon="mdi-delete-outline"
                      variant="text"
                      color="error"
                      aria-label="Delete category"
                      @click="deleteCategory(category)"
                    />
                  </td>
                </tr>
              </tbody>
            </v-table>
          </section>
        </v-window-item>

        <v-window-item value="topics">
          <section class="taxonomy-admin__section">
            <v-form class="taxonomy-admin__form" @submit.prevent="submitTopic">
              <div class="taxonomy-admin__form-grid">
                <v-text-field
                  v-model="topicForm.name"
                  label="Name"
                  :error-messages="topicForm.errors.name"
                  variant="outlined"
                />
                <v-text-field
                  v-model="topicForm.slug"
                  label="Slug"
                  :error-messages="topicForm.errors.slug"
                  variant="outlined"
                />
                <v-text-field
                  v-model.number="topicForm.sort_order"
                  label="Sort order"
                  type="number"
                  :error-messages="topicForm.errors.sort_order"
                  variant="outlined"
                />
                <v-switch
                  v-model="topicForm.is_visible"
                  class="taxonomy-admin__switch"
                  color="primary"
                  label="Visible"
                  hide-details
                />
              </div>
              <v-textarea
                v-model="topicForm.description"
                label="Description"
                rows="3"
                auto-grow
                :error-messages="topicForm.errors.description"
                variant="outlined"
              />
              <div class="taxonomy-admin__actions">
                <v-btn type="submit" color="primary" :loading="topicForm.processing">
                  {{ editingTopicId === null ? 'Create topic' : 'Update topic' }}
                </v-btn>
                <v-btn variant="outlined" @click="resetTopicForm">Cancel</v-btn>
              </div>
            </v-form>

            <v-table class="taxonomy-admin__table">
              <thead>
                <tr>
                  <th>Name</th>
                  <th>Slug</th>
                  <th>Sort</th>
                  <th>Visible</th>
                  <th />
                </tr>
              </thead>
              <tbody>
                <tr v-for="topic in topics" :key="topic.id">
                  <td>
                    <strong>{{ topic.name }}</strong>
                    <div class="text-medium-emphasis">{{ topic.description || 'No description' }}</div>
                  </td>
                  <td>{{ topic.slug }}</td>
                  <td>{{ topic.sort_order }}</td>
                  <td>
                    <v-chip
                      :color="topic.is_visible ? 'primary' : 'default'"
                      size="small"
                      variant="tonal"
                    >
                      {{ topic.is_visible ? 'Visible' : 'Hidden' }}
                    </v-chip>
                  </td>
                  <td class="text-right">
                    <v-btn
                      icon="mdi-pencil-outline"
                      variant="text"
                      aria-label="Edit topic"
                      @click="editTopic(topic)"
                    />
                    <v-btn
                      icon="mdi-delete-outline"
                      variant="text"
                      color="error"
                      aria-label="Delete topic"
                      @click="deleteTopic(topic)"
                    />
                  </td>
                </tr>
              </tbody>
            </v-table>
          </section>
        </v-window-item>

        <v-window-item value="tags">
          <section class="taxonomy-admin__section">
            <v-form class="taxonomy-admin__form" @submit.prevent="submitTag">
              <div class="taxonomy-admin__tag-grid">
                <v-text-field
                  v-model="tagForm.name"
                  label="Name"
                  :error-messages="tagForm.errors.name"
                  variant="outlined"
                />
                <v-text-field
                  v-model="tagForm.slug"
                  label="Slug"
                  :error-messages="tagForm.errors.slug"
                  variant="outlined"
                />
              </div>
              <div class="taxonomy-admin__actions">
                <v-btn type="submit" color="primary" :loading="tagForm.processing">
                  {{ editingTagId === null ? 'Create tag' : 'Update tag' }}
                </v-btn>
                <v-btn variant="outlined" @click="resetTagForm">Cancel</v-btn>
              </div>
            </v-form>

            <v-table class="taxonomy-admin__table">
              <thead>
                <tr>
                  <th>Name</th>
                  <th>Slug</th>
                  <th />
                </tr>
              </thead>
              <tbody>
                <tr v-for="tag in tags" :key="tag.id">
                  <td><strong>{{ tag.name }}</strong></td>
                  <td>{{ tag.slug }}</td>
                  <td class="text-right">
                    <v-btn
                      icon="mdi-pencil-outline"
                      variant="text"
                      aria-label="Edit tag"
                      @click="editTag(tag)"
                    />
                    <v-btn
                      icon="mdi-delete-outline"
                      variant="text"
                      color="error"
                      aria-label="Delete tag"
                      @click="deleteTag(tag)"
                    />
                  </td>
                </tr>
              </tbody>
            </v-table>
          </section>
        </v-window-item>
      </v-window>
    </v-card>
  </v-container>
</template>
