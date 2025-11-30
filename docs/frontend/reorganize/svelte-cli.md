---
title: Svelte CLI
parent: Reorganize
grand_parent: Frontend
nav_order: 2
---

<!-- prettier-ignore-start -->
# Svelte CLI
{: .no_toc }

<details open markdown="block">
  <summary>
    Table of contents
  </summary>
  {: .text-delta }
1. TOC
{:toc}
</details>

<!-- prettier-ignore-end -->

-   `pnpx sv create [options] [path]` - Create SvelteKit Project
    -   `--template minimal`
    -   `--types ts`
    -   `--install pnpm` - use pnpm package manager
    -   `--add [add-ons...]` - the add-ons that you select in interactive prompt
    -   `--no-add-ons` - run the command without interactive add-ons prompt
-   `pnpx sv check`
    -   Install dependency `npm i -D svelte-check`.
    -   It will check for unused CSS, accessibility hints, JS/TS compile errors.
    -   Add it to build script to be run automatically.
    -   Options
        -   `--output <format>`
            -   human
            -   human-verbose
            -   machine
            -   machine-verbose
        -   `--watch`
        -   `--preserveWatchOutput` - prevent screen from being cleared in watch mode
        -   `--fail-on-warnings`
-   Add-ons
    -   `devtools-json` - Modify source files from chrome devtools
    -   `drizzle`
    -   `eslint`
    -   `mdsvex`
    -   `paraglide` - i18n
    -   `playwright`
    -   `prettier`
    -   `storybook`
    -   `sveltekit-adpater="adapter-static"`
        -   auto
        -   node - create standalone node server
        -   static - static site generator
        -   vercel - deploy to vercel
        -   cloudfare
        -   netlify
    -   `tailwindcss="plugins:typography,forms`
    -   `vitest`
