---
title: Overview
parent: HTML
grand_parent: Frontend
nav_order: 1
---

<!-- prettier-ignore-start -->
# Overview
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

## Resources

- [link](https://web.dev/learn/html) web.dev LearnHTML

---

`<!DOCTYPE html>`

- Use standard mode instead of quirks mode.

`<html lang="en-US">`

- Langauge used by screen readers, search engines and translation services to know the document langauge.
- Use css `:lang(en)` to target.

`<meta charset="utf-8" />`

- Specify it before the `<title>`, to ensure browser can render the characters in the title.
- Default encoding is _windows-1252_ depending on the locale.
- The encoding is inherited by style and script tags, allowing you to use emojis in JavaScript.

`<title>...</title>`

- Showed in browser tab, list of open windows, history, search results, social media cards (if appropriate meta tags are not provided).

`<meta name="viewport" content="width=device-width" />`

- Make the site responsive, by making the width of the content the width of the screen.
- Zoom and scale value default to accessible values.
- If this meta element is not provided, then the content will have a width of 960px.

`<link rel="stylesheet" href="styles.css">`

- Include styles from `<link>`, `<style>` in head for performance, to prevent the unncessary repainting if an element is styled after it is first rendered.

```html
<style>
    @import "styles.css" layer(firstLayer);
</style>
```

- To import styles in cascade layers.

`<link rel="icon" sizes="16x16 32x32 48x48" type="image/png" href="/images.png" />`

- Appear in browser tab.
- By default browser looks for `favicon.ico` in the top-level directory.

```html
<link rel="apple-touch-icon" sizes="180x180" href="/images/mlwicon.png" />
<link rel="mask-icon" href="/images/mlwicon.svg" color="#226DAA" />
```

- For safari browser in ios devices (when user adds a site to home screen) and pinned tabs on macos respectively.

`<link rel="canonical" href="https://www.machinelearning.com" />`

- Point to the authoritative source.
- Use self-referencing canonical link as well.
- Mostly used for cross-posting with publications and blogging platforms to credit the original source.

`<script>`

- When using `defer`, content is downloaded in parallel to page render. But the content is only executed after the page is finished rendering.
- When using `async`, content is downloaded in parallel to page render. As soon as the content is downloaded, the page render is stopped, and content is executed.

`<base>`

- To provide base path for relative urls.
- The problem is when using fragments `<a href="#ref">`, the path is converted to `<a href="https://kushaj.com#ref">` which triggers an HTTP request.
