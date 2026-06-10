<script setup lang="ts">
import { computed, ref } from 'vue'
import { router, usePage } from '@inertiajs/vue3'
import { useDisplay } from 'vuetify'
import BrandMark from '../components/BrandMark.vue'
import AppNav from '../components/shell/AppNav.vue'
import ThemeToggle from '../components/shell/ThemeToggle.vue'
import UserMenu from '../components/shell/UserMenu.vue'
import type { AuthUser } from '../types/auth'

const page = usePage()
const { mobile } = useDisplay()
const drawer = ref(false)

const user = computed(
  () => (page.props.auth as { user?: AuthUser | null } | undefined)?.user ?? null,
)
const flashSuccess = computed(() => page.flash?.success ?? null)
const flashError = computed(() => page.flash?.error ?? null)
const navOpen = computed({
  get: () => Boolean(user.value) && (!mobile.value || drawer.value),
  set: (value: boolean) => {
    if (mobile.value) {
      drawer.value = value
    }
  },
})

function logout() {
  router.post('/logout')
}

function closeMobileNav() {
  if (mobile.value) {
    drawer.value = false
  }
}
</script>

<template>
  <v-app>
    <v-app-bar flat border="b" class="app-shell__bar">
      <v-container class="d-flex align-center py-0">
        <v-app-bar-nav-icon
          v-if="user && mobile"
          variant="text"
          aria-label="Open navigation"
          @click.stop="drawer = !drawer"
        />

        <v-btn to="/" variant="text" class="brand-link px-2" aria-label="Pulsar home">
          <BrandMark show-text :decorative="false" direction="row" :size="32" />
        </v-btn>

        <v-btn
          v-if="!mobile"
          to="/docs"
          variant="text"
          prepend-icon="mdi-book-open-page-variant-outline"
        >
          Docs
        </v-btn>
        <v-btn
          v-if="!mobile"
          to="/blog"
          variant="text"
          prepend-icon="mdi-newspaper-variant-outline"
        >
          Blog
        </v-btn>

        <v-spacer />

        <ThemeToggle />

        <template v-if="user">
          <v-btn
            v-if="!mobile"
            to="/dashboard"
            variant="text"
            prepend-icon="mdi-view-dashboard-outline"
          >
            Dashboard
          </v-btn>
          <UserMenu :user="user" @logout="logout" />
        </template>
        <template v-else>
          <v-btn to="/login" variant="text">Log in</v-btn>
          <v-btn to="/register" color="primary">Register</v-btn>
        </template>
      </v-container>
    </v-app-bar>

    <v-navigation-drawer
      v-if="user"
      v-model="navOpen"
      :permanent="!mobile"
      :temporary="mobile"
      :location="mobile ? 'bottom' : undefined"
      width="288"
      class="app-shell__drawer"
    >
      <AppNav :user="user" @navigate="closeMobileNav" />
    </v-navigation-drawer>

    <v-main class="app-shell__main">
      <v-container v-if="flashSuccess || flashError" class="pt-4 pb-0">
        <v-alert
          v-if="flashSuccess"
          type="success"
          variant="tonal"
          :text="flashSuccess"
          class="mb-2"
        />
        <v-alert
          v-if="flashError"
          type="error"
          variant="tonal"
          :text="flashError"
        />
      </v-container>

      <slot />
    </v-main>
  </v-app>
</template>
