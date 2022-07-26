const yaml = require('yaml');

const buildPath = "build/";
module.exports = {
  parsers: [{
    pattern: /\.yaml$/,
    parse: ({contents, filePath}) => yaml.parse(contents)
  }],
  source: [`src/**/*.yaml`, `src/**/*.js`],
  platforms: {
    json: {
      buildPath,
      files: [{
        destination: "style-dictionary-verbose.json",
        format: "json"
      }]
    },
    jsonNested: {
      buildPath,
      files: [{
        destination: "style-dictionary-simple.json",
        format: "json/nested"
      }]
    },
    scss: {
      transformGroup: "scss",
      buildPath,
      files: [{
        "destination": "_variables.scss",
        "format": "scss/variables",
        "options": { "outputReferences": true },
      }]
    },
    less: {
      transformGroup: "less",
      buildPath,
      files: [{
        "destination": "_variables.less",
        "format": "less/variables",
        "options": { "outputReferences": true },
      }]
    },
    css: {
      transformGroup: "css",
      buildPath,
      files: [{
        "destination": "_variables.css",
        "format": "css/variables",
        "options": { "outputReferences": true },
      }]
    },
  }
}