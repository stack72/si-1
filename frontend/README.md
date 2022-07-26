# SI Frontend Monorepo

Monorepo containing all of SI's frontend (webapp, public site, component library, etc)

> Note that this "monorepo" actually lives within the SI monorepo containing the backend code too.

## Monorepo layout
- **apps** - web applications meant to be built and run on their own (ex: website, admin dashboards, etc)
- **packages** - shared packages meant to be reused by our apps, or even published and used elsewhere (ex: component library, utils, etc)
- **config** - shared config to be reused (eslint config, etc)

## Setup
- Please use/install [volta](https://volta.sh/) to install node in order to make sure you're using the right node version (pinned in this repo's package.json file).
- Please install [PNPM](https://pnpm.io/) (`npm i pnpm -g`) to use as your package manager.
- Run `pnpm install` to install package dependencies
