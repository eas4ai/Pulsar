import '@inertiajs/core'

declare module '@inertiajs/core' {
  export interface InertiaConfig {
    errorValueType: string[]
    flashDataType: { success?: string; error?: string }
  }
}
