<script setup lang="ts">
import { computed, onMounted, watch } from 'vue'
import { useTheme } from 'vuetify'

const theme = useTheme()

const isDark = computed(() => theme.global.current.value.dark)
const icon = computed(() => (isDark.value ? 'mdi-weather-sunny' : 'mdi-weather-night'))

function syncThemeAttribute(name: string) {
  document.documentElement.dataset.theme = name === 'dark' ? 'dark' : 'light'
}

onMounted(() => {
  const saved = localStorage.getItem('pulsar-theme')
  if (saved === 'light' || saved === 'dark') {
    theme.global.name.value = saved
  }
  syncThemeAttribute(theme.global.name.value)
})

watch(
  () => theme.global.name.value,
  (name) => syncThemeAttribute(name),
)

function toggleTheme() {
  const next = isDark.value ? 'light' : 'dark'
  theme.global.name.value = next
  localStorage.setItem('pulsar-theme', next)
}
</script>

<template>
  <v-btn
    :icon="icon"
    variant="text"
    aria-label="Toggle theme"
    @click="toggleTheme"
  />
</template>
