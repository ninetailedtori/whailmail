<!--
SPDX-FileCopyrightText: 2026 2026-Present ninetailedtori <ninetailedtori@uwu.gal>
SPDX-FileContributor: WhailMail contributors

SPDX-License-Identifier: GPL-3.0-or-later
-->

# WhailMail

> The app that makes handling your mail, a whale of a time!

## Aims

At the root of the project, the aims are such:

- To provide a smooth, native-like experience, with modern intuitive stylish UI
  design
  - For cross-platform including all desktop, mobile and web
- Good sync capabilities
- Support for as many major auth and mail protocols as possible
- A background service for faster indexing, background auto-handling and
  archival
- Auto-starting capabilities
- Notifications on all platforms, of course :)
- Ability to "minimise to tray" with notification badges
- Support for mailserver creation/self-hosting
- Support for mail filtering both in incoming buffer
- Support for filters for local mailservers.
- Importantly, the app needs to be fast, so the whole user experience feels like
  a native user experience.
- And above all, it needs to be a comfortable product on all platforms!

## Project Setup

```sh
pnpm install
```

### Compile and Hot-Reload for Development

```sh
pnpm dev
```

### Type-Check, Compile and Minify for Production

```sh
pnpm build
```

### Run Unit Tests with [Vitest](https://vitest.dev/)

```sh
pnpm test:unit
```

### Run End-to-End Tests with [Playwright](https://playwright.dev)

```sh
# Install browsers for the first run
npx playwright install

# When testing on CI, must build the project first
pnpm build

# Runs the end-to-end tests
pnpm test:e2e
# Runs the tests only on Chromium
pnpm test:e2e --project=chromium
# Runs the tests of a specific file
pnpm test:e2e tests/example.spec.ts
# Runs the tests in debug mode
pnpm test:e2e --debug
```

### Lint with [ESLint](https://eslint.org/)

```sh
pnpm lint
```
