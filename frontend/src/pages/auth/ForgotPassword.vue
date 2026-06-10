<script setup lang="ts">
import { ref } from 'vue'
import { Link, useForm } from '@inertiajs/vue3'
import BrandMark from '../../components/BrandMark.vue'

const form = useForm({ email: '' })
const linkSent = ref(false)

function submit() {
  linkSent.value = false
  form.post('/forgot-password', {
    preserveState: true,
    onSuccess: () => {
      linkSent.value = true
    },
  })
}
</script>

<template>
  <v-container class="d-flex justify-center py-12">
    <v-card class="w-100" max-width="448">
      <v-card-item class="text-center pt-6">
        <BrandMark />
        <v-card-title class="mt-3">Forgot your password?</v-card-title>
        <v-card-subtitle class="text-wrap">
          Enter your email address and we'll send you a link to reset it.
        </v-card-subtitle>
      </v-card-item>

      <v-card-text>
        <v-alert
          v-if="linkSent"
          type="success"
          variant="tonal"
          icon="mdi-email"
          title="Check your inbox"
          text="If that email address is in our system, a password reset link is on its way."
          class="mb-4"
        />

        <v-form @submit.prevent="submit">
          <v-text-field
            v-model="form.email"
            label="Email address"
            type="email"
            autocomplete="email"
            :error-messages="form.errors.email"
            required
          />
          <v-btn type="submit" color="primary" block :loading="form.processing">
            Email password reset link
          </v-btn>
        </v-form>
      </v-card-text>

      <v-card-actions class="justify-center pb-4">
        <Link href="/login" class="text-primary text-body-2">Back to sign in</Link>
      </v-card-actions>
    </v-card>
  </v-container>
</template>
