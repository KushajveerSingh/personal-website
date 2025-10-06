---
title: Google Search
parent: SEO
nav_order:
---

<!-- prettier-ignore-start -->
# Google Search
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

-   [link](https://developers.google.com/search/docs) Google Search Central docs

---

## Tools

-   [Crawl Stats report](https://support.google.com/webmasters/answer/9679690) - stats about Google's crawling history on your website.
-   [Page Indexing report](https://support.google.com/webmasters/answer/7440203) - to check pages that are indexed by Google for your domain.
-   [URL Inspection tool](https://support.google.com/webmasters/answer/9012289) - check if a page is indexed.

## Minimum requirements

For Googlebot to index your site, the following things should be met

1. Googlebot should not be blocked. If a page is private i.e. requires to log-in to view, Googlebot will not crawl it.
    - Pages in `robots.txt` are unlikely to show in Google Search results.
    - Use [Page Indexing Report](https://support.google.com/webmasters/answer/7440203) and [Crawl Stats report](https://support.google.com/webmasters/answer/9679690) to see list of pages that are inacesible to Google, but you would like to see in Search results. [URL Inspection tool](https://support.google.com/webmasters/answer/9012289) can be used to test a specific page.
2. Only pages that return HTTP 200 status code are indexed.
3. The content on the page should be of valid type and not violate spam policies, for it to be indexed.

### URL Inspection tool

## URL best practices

[RFC 3986](https://www.rfc-editor.org/rfc/rfc3986) defines the URL standard.

-   Use simple, descriptive words, and the words should be in the audience's language.
    Example of German URL
    ```
    https://example.com/lebensmittel/pfefferminz
    ```
-   Use UTF-8 encoding as necessary.

    Example of Arabic characters with UTF-8

    ```
    https://example.com/%D9%86%D8%B9%D9%86%D8%A7%D8%B9/%D8%A8%D9%82%D8%A7%D9%84%D8%A9
    ```

-   Do not use non-ASCII characters.
-   Do not add unreadable, long ID numbers in URL (tihs includes session ID).
    Do not do this
    ```
    https://example.com/index.php?id_sezione=360&sid=3a5ebc944f41daa6f849f730f1
    ```
-   Do not use URI fragments to change content of page, as URL fragments are not supported.

    Do not do this

    ```
    https://example.com/#/potatoes
    ```

-   For multi-regional websites, use locale-specific URLs.
    ```
    https://example.de
    https://example.com/de/
    ```
-   Use hypens to separate words in URLs and help users to identify concepts in the URL more easily.
-   Do not use underscores.
-   Do not combine words in URL, instead separate them using hyphen.
-   To specify query parameters use `=` to separate key-value pairs and add additional parameters with `&`. To specify multiple values for a key use `,`.
    ```
    https://example.com/category?category=dresses&color=purple,pink,salmon&sort=low-to-high&sid=789
    ```

### Issues that lead to high number of URLs

The following can cause high number of URLs to be generated, which can cause Googlebot to not completely index all the content on your site.

-   Additive filtering of a set of items. For example on sites you can provide latitude, longitude as search parameters to get more refined results.
-   Dynamic generation of documents, which can cause small changes in the document like timestamps, or advertisements.
-   Problematic parameters in the URL, like Session IDs.
-   Sorting parameters in the URL.
-   Irrelevant parameters in the URL, such as referral parameters.
-   Infinite calendar, will have links to future and previous dates with no restrictions on start and end dates.
-   Broken relative links can often cause infinte spaces (like inifinte calendar).

Steps to resolve

-   Use `robots.txt` to ignore these URLs.
-   Shorten URLs by trimming unnecessary parameters.
-   For infinite calendar, add `nofollow` attribute to links to dynamically created future calendar pages.
