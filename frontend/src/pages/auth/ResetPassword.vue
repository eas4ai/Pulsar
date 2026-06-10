<script setup lang="ts">
import { computed } from 'vue'
import { Link, useForm } from '@inertiajs/vue3'
import BrandMark from '../../components/BrandMark.vue'
import type { ResetPasswordProps } from '../../types/inertia-props'

const props = defineProps<ResetPasswordProps>()

const form = useForm({
  token: props.token,
  password: '',
  password_confirmation: '',
})

const tokenError = computed(() => form.errors.token?.[0] ?? null)

function submit() {
  form.post('/reset-password')
}
</script>

<template>
  <v-container class="d-flex justify-center py-12">
    <v-card class="w-100" max-width="448">
      <v-card-item class="text-center pt-6">
        <BrandMark />
        <v-card-title class="mt-3">Reset your password</v-card-title>
        <v-card-subtitle class="text-wrap">
          Choose a new password for your account.
        </v-card-subtitle>
      </v-card-item>

      <v-card-text>
        <template v-if="tokenError">
          <v-alert
            type="error"
            variant="tonal"
            icon="mdi-link-variant-off"
            :title="tokenError"
            class="mb-4"
          />
          <v-btn to="/forgot-password" color="primary" block>
            Request a new link
          </v-btn>
        </template>

        <v-form v-else @submit.prevent="submit">
          <v-text-field
            v-model="form.password"
            label="New password"
            type="password"
            autocomplete="new-password"
            hint="At least 8 characters."
            :error-messages="form.errors.password"
            required
          />
          <v-text-field
            v-model="form.password_confirmation"
            label="Confirm new password"
            type="password"
            autocomplete="new-password"
            :error-messages="form.errors.password_confirmation"
            required
          />
          <v-btn type="submit" color="primary" block :loading="form.processing">
            Reset password
          </v-btn>
        </v-form>
      </v-card-text>

      <v-card-actions class="justify-center pb-4">
        <Link href="/login" class="text-primary text-body-2">Back to sign in</Link>
      </v-card-actions>
    </v-card>
  </v-container>
</template>
