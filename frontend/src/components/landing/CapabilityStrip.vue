<script setup lang="ts">
defineProps<{
  capabilities: LandingCapability[]
}>()
</script>

<template>
  <section class="capability-strip" aria-label="Pulsar kit capabilities">
    <div
      v-for="capability in capabilities"
      :key="capability.title"
      class="capability-strip__item"
    >
      <v-icon :icon="capability.icon" size="28" />
      <strong>{{ capability.title }}</strong>
      <span>{{ capability.body }}</span>
    </div>
  </section>
</template>
