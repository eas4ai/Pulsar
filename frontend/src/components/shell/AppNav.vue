<script setup lang="ts">
import { computed } from 'vue'
import type { AuthUser } from '../../types/auth'

const props = defineProps<{
  user: AuthUser | null
}>()

const emit = defineEmits<{
  navigate: []
}>()

const authedLinks = [
  { title: 'Dashboard', href: '/dashboard', icon: 'mdi-view-dashboard-outline' },
  { title: 'Docs', href: '/docs', icon: 'mdi-book-open-page-variant-outline' },
  { title: 'Blog', href: '/blog', icon: 'mdi-newspaper-variant-outline' },
  { title: 'Articles', href: '/admin/articles', icon: 'mdi-file-document-edit-outline' },
  { title: 'Profile', href: '/profile', icon: 'mdi-account-outline' },
]

const publicLinks = [
  { title: 'Home', href: '/', icon: 'mdi-home-outline' },
  { title: 'Docs', href: '/docs', icon: 'mdi-book-open-page-variant-outline' },
  { title: 'Blog', href: '/blog', icon: 'mdi-newspaper-variant-outline' },
]

const links = computed(() => (props.user ? authedLinks : publicLinks))
</script>

<template>
  <div class="app-nav">
    <div v-if="user" class="app-nav__account">
      <v-avatar color="primary" size="40">
        {{ user.name.charAt(0).toUpperCase() }}
      </v-avatar>
      <div>
        <div class="app-nav__name">{{ user.name }}</div>
        <div class="app-nav__email">{{ user.email }}</div>
      </div>
    </div>

    <v-list nav density="comfortable" class="app-nav__list">
      <v-list-item
        v-for="link in links"
        :key="link.href"
        :to="link.href"
        :prepend-icon="link.icon"
        :title="link.title"
        rounded="lg"
        @click="emit('navigate')"
      />
    </v-list>
  </div>
</template>
