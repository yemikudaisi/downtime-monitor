// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: true },
  srcDir: "src/",
  ssr: false,
  telemetry: false,
  modules: [
    "@nuxtjs/tailwindcss",
    "@pinia/nuxt",
    "@vueuse/nuxt",
    'radix-vue/nuxt',
    "@nuxt/eslint",
    'nuxt-icon'
  ],
  eslint: {
    config: {
      stylistic: true // <---,
    }
  },
  css: [
    'assets/scss/main.scss'
  ],
  sourcemap: {
    server: true,
    client: true
  }
})