# SystemInit - Style Dictionary

This repo defines our [design tokens](https://uxdesign.cc/design-tokens-for-dummies-8acebf010d71) in simple config files, which can then be transformed by [Style Dictionary](https://amzn.github.io/style-dictionary/#/) into different formats for consumption in different ways.

It's absolutely overkill for right now since we don't have different platforms (mobile apps), but it's fairly harmless, keeps the definitions isolated, and I feel like it may be useful at some point.

**NOTE** - you must rerun the build process after changing the source files

## Scripts
- `pnpm build` - runs the build process and generates the build folder containing the various formats