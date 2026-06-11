<script setup lang="ts">
import { Link, useForm } from '@inertiajs/vue3'
import BrandMark from '../../components/BrandMark.vue'

const form = useForm({
  email: '',
  password: '',
  remember: false,
})

function submit() {
  form.post('/login')
}
</script>

<template>
  <v-container class="auth-page">
    <v-card class="auth-card">
      <v-card-item class="text-center pt-6">
        <BrandMark />
        <v-card-title class="mt-3">Sign in to Pulsar</v-card-title>
        <v-card-subtitle class="text-wrap">
          Welcome back. Enter your credentials to continue.
        </v-card-subtitle>
      </v-card-item>

      <v-card-text>
        <v-form @submit.prevent="submit">
          <v-text-field
            v-model="form.email"
            label="Email address"
            type="email"
            autocomplete="email"
            :error-messages="form.errors.email"
            required
          />
          <v-text-field
            v-model="form.password"
            label="Password"
            type="password"
            autocomplete="current-password"
            :error-messages="form.errors.password"
            required
          />

          <div class="d-flex align-center justify-space-between mb-4">
            <v-checkbox
              v-model="form.remember"
              label="Remember me"
              density="compact"
              hide-details
            />
            <Link href="/forgot-password" class="text-primary text-body-2">
              Forgot your password?
            </Link>
          </div>

          <v-btn type="submit" color="primary" block :loading="form.processing">
            Sign in
          </v-btn>
        </v-form>
      </v-card-text>

      <v-card-actions class="auth-card__links justify-center pb-4">
        <span class="text-body-2">Don't have an account?</span>
        <Link href="/register" class="text-primary text-body-2 ml-1">Register</Link>
      </v-card-actions>
    </v-card>
  </v-container>
</template>
