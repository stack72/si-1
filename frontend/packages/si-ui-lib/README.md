# SystemInit - Shared UI Library

This repo will contain a shared component library, meant to be used in our webapp, website, internal admin tools, and any other web-based projects we may build.

It will also contain some resuable config (ex: tailwind) and some basic assets (logos, favicons).

This will hopefully help us build an awesome library of reusable "dumb" components that are fully decoupled from the app itself.

Much to do here and to figure out as we migrate over some of these components and figure out what patterns we want to follow.

## How to use this repo
HMR should work on these components while working on an actual "app", but you can also work on the components themselves.

## Scripts
- `yarn dev` - runs a local dev environment w/ docs site, powered by [Histoire](https://histoire.dev/) (runs on http://127.0.0.1:5173)
- `yarn docs:build` - builds the histoire site in a format ready to deploy it
- `yarn docs:preview` - preview the built files from `docs:build`