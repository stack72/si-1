module.exports = { 
  env: {
    node: true,
    browser: true,
  },
  parser: "vue-eslint-parser",
  parserOptions: {
    parser: "@typescript-eslint/parser",
    parserOptions: {
      // TODO: figure our correct settings here
      // project: ['./packages/*/tsconfig.json', './apps/*/tsconfig.json'], 
      // tsconfigRootDir: __dirname, 
    },
  },
  plugins: ["@typescript-eslint", "vue"],
  extends: [
    "eslint:recommended",
    "@vue/typescript/recommended",
    "@vue/typescript/recommended",
    "plugin:vue/vue3-recommended",
    "@vue/prettier",
  ],
  globals: {
    defineProps: "readonly",
    defineEmits: "readonly",
    defineExpose: "readonly",
    withDefaults: "readonly",
  },  
  rules: {
    camelcase: "off",
    "no-console": "off",
    "no-debugger": "off",
    "no-alert": "error",
    "@typescript-eslint/no-unused-vars": [
      "warn",
      {
        argsIgnorePattern: "^_",
        varsIgnorePattern: "^_",
      },
    ],
    '@typescript-eslint/ban-ts-comment': "off",
    
    "vue/script-setup-uses-vars": "error",
    "vue/multi-word-component-names": "off",
  },
}