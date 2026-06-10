<script setup lang="ts">
import type { LandingMetric } from '../../types/inertia-props'

defineProps<{
  metrics: LandingMetric[]
}>()
</script>

<template>
  <section class="metric-strip" aria-label="Pulsar defaults">
    <div
      v-for="metric in metrics"
      :key="metric.label"
      class="metric-strip__item"
    >
      <strong>{{ metric.value }}</strong>
      <span>{{ metric.label }}</span>
    </div>
  </section>
</template>
