---
title: Uniform Resource Locator (URL)
parent: SEO
nav_order: 2
---

<!-- prettier-ignore-start -->
# Uniform Resource Locator (URL)
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

## Rersources

-   [link](https://developer.mozilla.org/en-US/docs/Web/URI/Guides/Choosing_between_www_and_non-www_URLs) MDN URI: Choosing between www and non-www URLs
-   [link](https://www.netlify.com/blog/2020/03/26/how-to-set-up-netlify-dns-custom-domains-cname-and-a-records/#options-for-bare-domains) How to set up Netlify DNS - Custom Domains, CNAME, & A Records
-   [link](https://www.wpbeginner.com/beginners-guide/www-vs-non-www-which-is-better-for-wordpress-seo/) WWW vs non-WWW - Which is Better for WordPress SEO?

---

## www vs non-www URL

`www` serves as the hostname. Domains without `www` are called naked domains or root domain or apex domain.

-   Choose one as the default (canonical). I use non-www as the default.
-   Namecheap can handle redirect of www to non-www. Alternatively, use HTTP 301 redirect.
-   In case you are serving both www and non-www domains with the same content, use `<link rel="canonical" href="" />` to tell search engine the canonical domain. The search engine will treat both the domains as different, and penalize for duplicate content/spam and lower the page rank.
-   Do not use CNAME for the redirect. According to DNS specification, any domain that has a CNAME record set cannot have other DNS records associated with it. Instead use the A record to point to a server and from the server do HTTP 301 redirect.
-   It is more important to mantain consistency than deciding which url to use.

Benefits of www

-   At high traffic volumes (millions of daily users) `www` makes more sense.
-   Easier to manage DNS settings when using `www`.
-   Handling cookies is easier. A cookie set on `example.com` would apply to all subdomains as well.
-   CNAME record cannot be added to root domain. CDNs rely on CNAME records and using a non-www domain makes it harder to redirect traffic when servers experience issues. `www.example.com` can be redirected to `mycdn.provider.net` at the CDN level and `mycdn.provider.net` target can be changed whenever traffic needs to be rerouted.
