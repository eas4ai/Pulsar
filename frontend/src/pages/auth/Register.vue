<script setup lang="ts">
import { Link, useForm } from '@inertiajs/vue3'
import BrandMark from '../../components/BrandMark.vue'

const form = useForm({
  name: '',
  email: '',
  password: '',
  password_confirmation: '',
})

function submit() {
  form.post('/register')
}
</script>

<template>
  <v-container class="auth-page">
    <v-card class="auth-card">
      <v-card-item class="text-center pt-6">
        <BrandMark />
        <v-card-title class="mt-3">Create your account</v-card-title>
        <v-card-subtitle class="text-wrap">
          Join Pulsar. It only takes a minute.
        </v-card-subtitle>
      </v-card-item>

      <v-card-text>
        <v-form @submit.prevent="submit">
          <v-text-field
            v-model="form.name"
            label="Name"
            type="text"
            autocomplete="name"
            :error-messages="form.errors.name"
            required
          />
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
            autocomplete="new-password"
            hint="At least 8 characters."
            :error-messages="form.errors.password"
            required
          />
          <v-text-field
            v-model="form.password_confirmation"
            label="Confirm password"
            type="password"
            autocomplete="new-password"
            :error-messages="form.errors.password_confirmation"
            required
          />

          <v-btn type="submit" color="primary" block :loading="form.processing">
            Register
          </v-btn>
        </v-form>
      </v-card-text>

      <v-card-actions class="auth-card__links justify-center pb-4">
        <span class="text-body-2">Already have an account?</span>
        <Link href="/login" class="text-primary text-body-2 ml-1">Sign in</Link>
      </v-card-actions>
    </v-card>
  </v-container>
</template>
