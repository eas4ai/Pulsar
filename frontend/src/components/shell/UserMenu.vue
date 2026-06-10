<script setup lang="ts">
import { useDisplay } from 'vuetify'
import type { AuthUser } from '../../types/auth'

defineProps<{
  user: AuthUser
}>()

const emit = defineEmits<{
  logout: []
}>()

const { mobile } = useDisplay()
</script>

<template>
  <v-menu location="bottom end">
    <template #activator="{ props: menuProps }">
      <v-btn
        v-if="mobile"
        v-bind="menuProps"
        icon="mdi-account-circle-outline"
        variant="text"
        aria-label="Open user menu"
      />
      <v-btn
        v-else
        v-bind="menuProps"
        variant="text"
        append-icon="mdi-chevron-down"
        class="user-menu__button"
      >
        {{ user.name }}
      </v-btn>
    </template>

    <v-list min-width="240">
      <v-list-item
        to="/profile"
        prepend-icon="mdi-account-outline"
        title="Profile"
        subtitle="Account settings"
      />
      <v-divider />
      <v-list-item
        prepend-icon="mdi-logout"
        title="Log out"
        @click="emit('logout')"
      />
    </v-list>
  </v-menu>
</template>
