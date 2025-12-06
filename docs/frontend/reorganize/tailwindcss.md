---
title: Tailwind CSS
parent: Reorganize
grand_parent: Frontend
nav_order: 5
---

<!-- prettier-ignore-start -->
# Tailwind CSS
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

## Breakpoints

-   `sm` - 640px
-   `md` - 768px
-   `lg` - 1024px
-   `xl` - 1280px
-   `2xl` - 1536px

Notes

-   Usage `md:flex`, will make the element flex for screens `md` and larger.
-   To target mobile use `text-center sm:text-left`.
-   To constrain the style to a range use `md:max-xl:flex`. There is `max-*` variant for each.

### Container queries

-   Specify `@container` on parent. Use `@md` for children.
-   Specify `@container/name` on parent. Use `@md/name:flex` for children.
