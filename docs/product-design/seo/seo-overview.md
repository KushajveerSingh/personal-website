---
title: SEO Overview
parent: SEO
nav_order: 3
---

<!-- prettier-ignore-start -->
# SEO Overview
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

-   [link](https://frontendmasters.com/courses/modern-seo/) Frontend Masters: Modern Search Engine Optimization (SEO) by _Mike North_ (4 hours 4 minutes) (Jun 15, 2017)

---

## Traditional SEO

-   Focus on _content quality_ and _crawler hints_.
-   Frequent updates, link building.
-   Keyword stuffing and using long tail keywords (multi-word phrases) to find very specific information, as site visitors are likely to use these keywords when they are closer to a point-of-purchase.
-   PageRank algorithm assumed "quality" of a site is proportional to the number of links to it. And the value of a link on a "quality site" is worth more.

### Example

-   Create a complete [Google Business Profile](https://business.google.com/us/business-profile/).
-   Ask for reviews for your business, and display on your website.
-   Submit your business to local directories.
-   Correct errors/outdated information in local citations.
-   Acquire local backlinks - vendors, partners, local NGOs.
-   In Google Search Console, specify your site is only for a specific region.

### Optimize PageRank

-   Create _key pages_ for popular product and services on your website.
-   Focus inbound links to key pages to build higher ranking pages.
-   Guest post on trustworthy industry/authroity sites.
-   Analyze competitors link profile and identify link opportunities.
-   Eliminate bad links
    -   Use Google Search Console to review search traffic and links.
    -   Remove link reference from spammy websites.
    -   Eliminate mutual _link exchange_ with other sites using `<rel="nofollow">`.
    -   Remove dead links from your website.
-   Do Google Ad Campaign.

## Social metadata

-   [link](https://developers.google.com/search/docs/appearance/structured-data) Google Structured Data Testing Tool
-   [link](https://seo-extension.com/) SEO Meta chrome extension
-   [link](https://developers.facebook.com/tools/debug/) Facebook OpenGraph debugger
    -   Images are cached, so change URL to update the cache.
-   [link](https://cards-dev.x.com/validator) Twitter card validator
-   [link](https://developers.pinterest.com/docs/web-features/rich-pins-overview/) Pinterest rich card debugger

## Structured data

-   Provide consistent structure, across websites.
-   Use JSON-LD syntax over HTML. Get the syntax from schema.org.
-   Adding this information helps us control the way search results look.

## Mobile optimization

-   Google uses a different index for mobile and desktop.
-   Provide information in manifest.json.
