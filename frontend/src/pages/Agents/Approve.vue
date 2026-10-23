<script setup lang="ts">
import { Form, Head, usePage } from '@inertiajs/vue3'
import type { ApprovalProps } from '../../types/inertia-props'

defineProps<ApprovalProps>()
const page = usePage()
</script>

<template>
  <Head title="Allow an app to act for you?" />
  <v-container class="py-8" style="max-width: 720px">
    <v-card class="pa-6">
      <h1 class="text-h5 mb-4">Allow <bdi>{{ approval.client_name }}</bdi> to act for you?</h1>
      <p class="mb-4">From <bdi>{{ approval.client_host }}</bdi>, it wants to use this application as you.</p>
      <p v-if="approval.scopes.length === 0" class="mb-4">It asks for no permissions: it can use only what needs none.</p>
      <ul v-else class="pl-6 mb-4">
        <li v-for="scope in approval.scopes" :key="scope.name">{{ scope.description }} (<code>{{ scope.name }}</code>)</li>
      </ul>
      <p class="mb-4">If you allow it, you return to <bdi style="overflow-wrap: anywhere">{{ approval.redirect_uri }}</bdi>.</p>
      <div class="d-flex ga-3">
        <Form v-for="decision in ['approve', 'deny']" :key="decision" :action="`${page.props.root}/oauth/approve`" method="post" v-slot="{ processing }">
          <input type="hidden" name="authorization" :value="approval.id" />
          <input type="hidden" name="decision" :value="decision" />
          <v-btn type="submit" :disabled="processing" :color="decision === 'approve' ? 'primary' : undefined">{{ decision === 'approve' ? 'Allow' : 'Deny' }}</v-btn>
        </Form>
      </div>
    </v-card>
  </v-container>
</template>
