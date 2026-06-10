<script setup lang="ts">
import { useForm } from '@inertiajs/vue3'
import type { ProfileFormState } from '../../types/inertia-props'

const props = defineProps<{
  name: string
  email: string
  emailVerified: boolean
  profile: ProfileFormState
}>()

const form = useForm({
  name: props.name,
  email: props.email,
  display_name: props.profile.display_name,
  handle: props.profile.handle,
  bio: props.profile.bio ?? '',
  avatar_url: props.profile.avatar_url ?? '',
  website_url: props.profile.website_url ?? '',
  github_url: props.profile.github_url ?? '',
  location: props.profile.location ?? '',
  timezone: props.profile.timezone ?? '',
})

function submit() {
  form.patch('/profile')
}
</script>

<template>
  <v-card class="profile-panel">
    <v-card-item>
      <div class="profile-panel__title-row">
        <div>
          <v-card-title>Profile information</v-card-title>
          <v-card-subtitle class="text-wrap">
            Update your account identity and public member profile.
          </v-card-subtitle>
        </div>
        <v-chip
          :color="emailVerified ? 'success' : 'warning'"
          variant="tonal"
          size="small"
        >
          {{ emailVerified ? 'Email verified' : 'Email not verified' }}
        </v-chip>
      </div>
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
          v-model="form.display_name"
          label="Display name"
          type="text"
          autocomplete="name"
          :error-messages="form.errors.display_name"
        />
        <v-text-field
          v-model="form.handle"
          label="Handle"
          type="text"
          prefix="@"
          hint="Lowercase letters, numbers, and hyphens."
          :error-messages="form.errors.handle"
          required
        />
        <v-text-field
          v-model="form.email"
          label="Email address"
          type="email"
          autocomplete="email"
          hint="Changing your email requires verifying the new address."
          :error-messages="form.errors.email"
          required
        />
        <v-textarea
          v-model="form.bio"
          label="Bio"
          rows="3"
          auto-grow
          :error-messages="form.errors.bio"
        />
        <v-text-field
          v-model="form.avatar_url"
          label="Avatar URL"
          type="url"
          autocomplete="photo"
          :error-messages="form.errors.avatar_url"
        />
        <v-row>
          <v-col cols="12" md="6">
            <v-text-field
              v-model="form.website_url"
              label="Website URL"
              type="url"
              autocomplete="url"
              :error-messages="form.errors.website_url"
            />
          </v-col>
          <v-col cols="12" md="6">
            <v-text-field
              v-model="form.github_url"
              label="GitHub URL"
              type="url"
              :error-messages="form.errors.github_url"
            />
          </v-col>
        </v-row>
        <v-row>
          <v-col cols="12" md="6">
            <v-text-field
              v-model="form.location"
              label="Location"
              type="text"
              autocomplete="address-level2"
              :error-messages="form.errors.location"
            />
          </v-col>
          <v-col cols="12" md="6">
            <v-text-field
              v-model="form.timezone"
              label="Timezone"
              type="text"
              autocomplete="off"
              :error-messages="form.errors.timezone"
            />
          </v-col>
        </v-row>
        <div class="profile-form-actions">
          <v-btn
            type="submit"
            color="primary"
            prepend-icon="mdi-content-save-outline"
            :loading="form.processing"
          >
            Save
          </v-btn>
          <span v-if="form.recentlySuccessful" class="text-body-2 text-medium-emphasis">
            Saved.
          </span>
        </div>
      </v-form>
    </v-card-text>
  </v-card>
</template>
