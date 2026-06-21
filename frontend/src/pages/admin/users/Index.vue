<script setup lang="ts">
import type { AdminUsersIndexProps } from '../../../types/inertia-props'

defineProps<AdminUsersIndexProps>()
</script>

<template>
  <v-container class="console-page">
    <header class="console-hero">
      <div>
        <div class="console-eyebrow">Users</div>
        <h1>Member directory</h1>
        <p>Review accounts, verification state, and assigned roles across Pulsar.</p>
      </div>
    </header>

    <v-card class="user-admin-table mt-8" variant="flat">
      <v-table>
        <thead>
          <tr>
            <th>Name</th>
            <th>Email</th>
            <th>Verified</th>
            <th>Roles</th>
            <th>Created</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="user in users" :key="user.id">
            <td>
              <strong>{{ user.name }}</strong>
            </td>
            <td>{{ user.email }}</td>
            <td>
              <v-chip
                :color="user.verified ? 'success' : 'warning'"
                size="small"
                variant="tonal"
              >
                {{ user.verified ? 'Yes' : 'No' }}
              </v-chip>
            </td>
            <td>{{ user.roles.join(', ') }}</td>
            <td>{{ new Date(user.created_at).toLocaleString() }}</td>
          </tr>
        </tbody>
      </v-table>
    </v-card>
  </v-container>
</template>
