---
title: URL
parent: SEO
nav_order: 2
---

<!-- prettier-ignore-start -->
# URL (Uniform Resource Locator)
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

## Type of URL

### Absolute URL string

`https://www.example.com:80/path/to/file.html?key1=value1&key2=value2#SomewhereInTheDocument`

-   Scheme `http` - Specify the protocol that the browser must use to request the resource.
-   Authority `www.example.com:80` - Separated from the schema using `://`. Includes both the domain `www.example.com` and the port `443` (ommitted if using standard ports 80 for HTTP and 443 for HTTPS).
-   Path to resource `/path/to/file.html` - Path to the resource on the web server. It can be an actual file or an abstraction handled by web servers without the presence of an actual file.
-   Parameters `?key1=value1&key2=value2` - The GET parameters.
-   Anchor `#SomewhereInTheDocument` - Location inside the resource itself.

### Relative URL string

For this absolute URL `https://developer.mozilla.org/en-US/docs/Learn_web_development`

-   Scheme-relative URL `//developer.mozilla.org/en-US/docs/Learn_web_development` - The browser will use the same protocol as the one used to load the document hosting that URL.
-   Domain-relative URL `/en-US/docs/Learn_web_development` - The protocol and domain name will be used from the host URL.
-   Sub-resource `HowTo/Web_mechanics/What_is_a_URL` - Browser will use path relative to `en-US/docs/Learn_web_development`. So the final URL becomes `https://developer.mozilla.org/en-US/docs/Learn_web_development/HowTo/Web_mechanics/What_is_a_URL`. We can use `..` to move up a directory.
-   Anchor-only `#semantic_urls` - To link to different parts of the current document, as the browser will use the entire URL.

### HTTP Authentication

`https://username:password@www.example.com:80`

-   To immediately sign in to a website and bypass the username/password dialog box.
-   Deprecated in modern browsers. And the username/password info is stripped from the request before it is sent.

## Best practices

-   Keep it short and memorable, in case someone has to manually write down the URL in the browser.
-   Only use lowercase, as it removes the ambiguity, and also the official URL spec says to use lowercase.
-   Use readable slug instead of ID. A readable slug makes it easier to glean the content of the page by just looking at the URL.
-   Do not use spaces. Spaces also make it harder to identify the end of the URL.
-   Use hyphens instead of underscore, use kebab case. Also, when links are underlined on a page, the underscores get hard to see.
-   Use period only for domains and indicating file extensions.
-   If possible avoid using `www` in the URL.
-   Do not have `.html` in the URL, as it adds unncessary length.
-   Avoid trailing slash where it does not make sense. `example.com/posts/` makes sense, since there is further content within `posts/`. But `example.com/posts/post-title/` is bad.
-   URL's structure should work as breadcrumb. And the user should be able to make their way back along the path without error at any stage.
-   Avoid date paths if you can `example.com/posts/2025/02/24/post-title`. It makes sense on news sites where there are multiple items releasing daily. Also, the date adds extra length to the url.
