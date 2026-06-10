<script setup lang="ts">
import type { LandingFeature } from '../../types/inertia-props'

defineProps<{
  features: LandingFeature[]
}>()
</script>

<template>
  <section class="landing-section">
    <div class="landing-section__eyebrow">Foundation</div>
    <h2 class="landing-section__title">The boring product plumbing is already wired.</h2>
    <v-row class="mt-8" dense>
      <v-col
        v-for="feature in features"
        :key="feature.title"
        cols="12"
        sm="6"
        lg="3"
      >
        <article class="landing-feature">
          <v-icon :icon="feature.icon" size="28" class="landing-feature__icon" />
          <h3>{{ feature.title }}</h3>
          <p>{{ feature.body }}</p>
        </article>
      </v-col>
    </v-row>
  </section>
</template>
