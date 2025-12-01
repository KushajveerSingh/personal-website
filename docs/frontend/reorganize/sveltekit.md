---
title: SvelteKit
parent: Reorganize
grand_parent: Frontend
nav_order: 4
---

<!-- prettier-ignore-start -->
# SvelteKit
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

## File Structure

-   `src/lib/` - Reference files under this using `$lib/`.
    -   `src/lib/server` - Can only be imported in server files using the alias `$lib/server`.
-   `src/params/` - Param matchers
-   `src/routes/` - Routes of the application.
-   `src/app.html` - Page template.
-   `src/error.html`
-   `static` - static assets like robots.txt, favicon.png.

## src/routes files

-   `+page.svelte` - Defines a page of your app.
    -   `+page.ts` - Define `load` function, which is used to fetch the data before rendering the page.
        -   Use `const { data }: PageProps = $props()` to get the data in `+page.svelte`.
    -   `+page.server.ts` - Define `load` function that can only run on the server.
-   `+error.svelte`- If error occurs during `load`, render this page.
-   `+layout.svelte` - Layout that applies to every page and is not destroyed when user navigates to another page.
    -   `+layout.ts` - Define `load` function for the layout page.
    -   `+layout.server.ts` - Define `load` function for the layout page, that is ran only on the server.
-   `+server.ts` - To define API routes.

## Static Site Generation

-   Add `export const prerender = true` in `src/routes/+layout.js`.
