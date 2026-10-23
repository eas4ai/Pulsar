import '@inertiajs/core'
import type { AuthUser } from './auth'
import type { SharedProps } from './inertia-props'

declare module '@inertiajs/core' {
  export interface InertiaConfig {
    sharedPageProps: SharedProps & { auth: { user: AuthUser | null } }
    errorValueType: string[]
    flashDataType: { success?: string; error?: string }
  }
}
