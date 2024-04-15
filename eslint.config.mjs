import withNuxt from './.nuxt/eslint.config.mjs'

export default withNuxt(
    extends: ["@nuxt/eslint-config", "plugin:tailwindcss/recommended", "plugin:perfectionist/recommended-natural"],
    ignorePatterns: ["src-tauri/**/*", "dist/**/*"],
    plugins: ["tailwindcss", "perfectionist"],
    root: true,
    rules: {
        "no-undef": "off",
        "perfectionist/sort-vue-attributes": "off",
        "tailwindcss/no-custom-classname": "off",
        "vue/html-indent": ["warn", 2],
        "indent": ["warn", 2]
        "vue/multi-word-component-names": "off",
        "vue/v-on-event-hyphenation": "off",
        "@typescript-eslint/quotes": [
          "error",
          "single",
          {
            "allowTemplateLiterals": true
          }
        ]
    }
)
