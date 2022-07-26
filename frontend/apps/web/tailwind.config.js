const commonConfig = require('@si/ui-lib/tailwind.config.cjs');

commonConfig.content.push(...[
  // add our shared ui-lib to our local tailwind's build content
  "./node_modules/@si/ui-lib/**/*.{vue,ts,css}",
  "!./node_modules/@si/ui-lib/**/*.story.vue" // skip story files
]);

module.exports = commonConfig;