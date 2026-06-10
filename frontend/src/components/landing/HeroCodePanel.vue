<script setup lang="ts">
import type { LandingCodeSample } from '../../types/inertia-props'

defineProps<{
  sample: LandingCodeSample
}>()
</script>

<template>
  <section class="hero-code-panel" aria-label="Suprnova route sample">
    <div class="hero-code-panel__toolbar">
      <div class="hero-code-panel__dots" aria-hidden="true">
        <span />
        <span />
        <span />
      </div>
      <v-chip size="small" color="teal" variant="tonal" label>
        {{ sample.language }}
      </v-chip>
      <v-chip size="small" color="amber" variant="tonal" label>
        static fallback
      </v-chip>
    </div>
    <pre class="hero-code-panel__code"><code>{{ sample.code }}</code></pre>
    <div class="hero-code-panel__status">
      <span>routes loaded</span>
      <span>assets cached</span>
      <span>auth guarded</span>
    </div>
  </section>
</template>
