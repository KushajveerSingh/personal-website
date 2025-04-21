---
title: General
nav_order: 1
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
-   **Search engine** - Web service that helps people find web pages, such as Google, Bing.
-   **Interoperable** - Different implementations behave exactly the same for a given case.

## Folder structure

-   `index.html`
-   `images` folder
-   `styles` folder
-   `scripts` folder

## Naming files/folders

-   Lowercase with no spaces.
    -   Web servers are case-sensitive. Having all names in lowercase helps reduce random errors.
    -   In terminal, file names need to be quoted if containing spaces.
    -   In URL, the space is converted to `%20`, which can create bugs if the system assumes file names and URLs match perfectly.
-   Use hyphen (`-`) instead of space.
-   Do not use backward slashes to reference file paths on Windows. HTML can handle forward slashes on Windows.

## Web Platform

Collection of standardized _application programming interfaces (APIs)_ that programmers use to make web pages and web applications.

-   Browser
    -   W3C
    -   WHATWG
-   Ecma International
    -   Ecma-262, Ecma-402, Ecma-404 (collectively ECMAScript)
-   OpenJS Foundation
    -   Node.js
    -   Node Global
-   IETF
    -   HTTP
-   Unicode Consortium
    -   Unicode standard
    -   Text related issues like bi-directional text, line-breaking
-   IEEE
    -   IEEE 802.11 WLAN
    -   IEEE 754 Floating-Point Arithmetic
-   ISO
    -   Standards organizations like W3C will publish ISO standard, to allow governments to require these standards for policy and procurement purposes.
-   Khronos
    -   WebGL
-   IANA
    -   DNS Root, IP addressing

## Resilient Web Design

Core principle - One material should not be used as a substitute for another. For example, using `table` for layout is materially dishonest.
