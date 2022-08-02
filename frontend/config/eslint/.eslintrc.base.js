module.exports = {
  env: {
    node: true,
  },
  parser: "@typescript-eslint/parser",
  parserOptions: {
    project: ['./packages/*/tsconfig.json'], 
    tsconfigRootDir: __dirname, 
  },
  plugins: ["@typescript-eslint"],
  extends: [
    "eslint:recommended",
    "@vue/typescript/recommended",
  ],
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
  },
}
