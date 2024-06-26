// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: false },
  srcDir: "src/",
  ssr: false,
  telemetry: false,
  modules: [
    "@nuxtjs/tailwindcss",
    "@pinia/nuxt",
    "@vueuse/nuxt",
    'radix-vue/nuxt', // TODO: Remove radix if not used
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