<script setup lang="ts">
import { computed } from 'vue'
import { usePage } from '@inertiajs/vue3'
import BrandMark from '../components/BrandMark.vue'
import CtaBand from '../components/landing/CtaBand.vue'
import FeatureGrid from '../components/landing/FeatureGrid.vue'
import HeroCodePanel from '../components/landing/HeroCodePanel.vue'
import CapabilityStrip from '../components/landing/CapabilityStrip.vue'
import type { AuthUser } from '../types/auth'
import type { HomeProps } from '../types/inertia-props'

defineProps<HomeProps>()

const page = usePage()
const user = computed(
  () => (page.props.auth as { user?: AuthUser | null } | undefined)?.user ?? null,
)
</script>

<template>
  <div class="landing-page">
    <section class="landing-hero">
      <div class="landing-hero__visual" aria-hidden="true">
        <BrandMark :size="560" />
      </div>
      <v-container class="landing-hero__inner">
        <div class="landing-hero__copy">
          <v-chip color="teal" variant="tonal" label class="mb-5">
            Suprnova starter kit
          </v-chip>
          <h1>{{ headline }}</h1>
          <p>{{ subheadline }}</p>
          <div class="landing-hero__actions">
            <v-btn
              v-if="user"
              to="/dashboard"
              color="primary"
              size="large"
              prepend-icon="mdi-view-dashboard-outline"
            >
              Go to dashboard
            </v-btn>
            <template v-else>
              <v-btn
                to="/register"
                color="primary"
                size="large"
                prepend-icon="mdi-account-plus-outline"
              >
                Get started
              </v-btn>
              <v-btn
                to="/login"
                variant="outlined"
                size="large"
                prepend-icon="mdi-login"
              >
                Sign in
              </v-btn>
            </template>
          </div>
        </div>
        <HeroCodePanel :sample="sample" />
      </v-container>
    </section>

    <CapabilityStrip :capabilities="capabilities" />

    <v-container class="landing-content">
      <FeatureGrid :features="features" />
      <CtaBand :authenticated="Boolean(user)" />
    </v-container>
  </div>
</template>
