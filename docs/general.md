---
title: General
nav_order: 0.1
---

<!-- prettier-ignore-start -->
# General
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

## Terminology

-   **Web page** (or "page") - Document that can be displayed in a web browser, like a HTML document.
-   **Website** (or "site") - Collection of web pages grouped together into a single resource, with links connecting them together.
-   **Web server** - Computer that hosts a website on the internet.
-   **Search engine** - Web servvice that helps you find web pages, such as Google, Bing.

## Folder structure

-   `index.html`
-   `images` folder
-   `styles` folder
-   `scripts` folder

## Naming files/folders

-   Lowercase with no spaces.
    -   Web servers are case-sensitive. Having all names in lowercase helps reduce random errors.
    -   In terminal, you have to quote the file name if it has spaces.
    -   In URL, the space would be converted to `%20`, which can create bugs if the system assumes file names and URLs match perfectly.
-   Use hyphen (`-`) rather than space.

Do not use backward slashes to reference file paths on Windows. HTML can handle forward slahses on Windows.
