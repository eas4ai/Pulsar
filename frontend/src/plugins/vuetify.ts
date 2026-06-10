import 'vuetify/styles'
import '@mdi/font/css/materialdesignicons.css'
import { createVuetify } from 'vuetify'

export default createVuetify({
  theme: {
    defaultTheme: 'dark',
    themes: {
      dark: {
        dark: true,
        colors: {
          primary: '#2dd4bf',
          secondary: '#f59e0b',
          accent: '#38bdf8',
          background: '#101828',
          surface: '#172033',
          error: '#f87171',
          info: '#38bdf8',
          success: '#34d399',
          warning: '#fbbf24',
        },
      },
      light: {
        dark: false,
        colors: {
          primary: '#0f766e',
          secondary: '#b45309',
          accent: '#0284c7',
          background: '#f5f7f3',
          surface: '#ffffff',
          error: '#b91c1c',
          info: '#0369a1',
          success: '#047857',
          warning: '#b45309',
        },
      },
    },
  },
  defaults: {
    VTextField: { variant: 'outlined', density: 'comfortable' },
    VBtn: { variant: 'flat', class: 'text-none' },
  },
})
