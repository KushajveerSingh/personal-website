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

- [link](https://developer.mozilla.org/en-US/docs/Web/URI/Guides/Choosing_between_www_and_non-www_URLs) MDN URI: Choosing between www and non-www URLs
- [link](https://www.netlify.com/blog/2020/03/26/how-to-set-up-netlify-dns-custom-domains-cname-and-a-records/#options-for-bare-domains) How to set up Netlify DNS - Custom Domains, CNAME, & A Records
- [link](https://www.wpbeginner.com/beginners-guide/www-vs-non-www-which-is-better-for-wordpress-seo/) WWW vs non-WWW - Which is Better for WordPress SEO?
- [link](https://alfy.blog/2025/10/16/hidden-cost-of-url-design.html) The Hidden Cost of URL Design
- [link](https://alfy.blog/2025/10/31/your-url-is-your-state.html) Your URL Is Your State
- [link](https://stackoverflow.com/questions/417142/what-is-the-maximum-length-of-a-url-in-different-browsers/417184#417184) What is the maximum length of a URL in different browsers?

---

## www vs non-www URL

`www` serves as the hostname. Domains without `www` are called naked domains or root domain or apex domain.

- Choose one as the default (canonical). I use non-www as the default.
- Namecheap can handle redirect of www to non-www. Alternatively, use HTTP 301 redirect.
- In case you are serving both www and non-www domains with the same content, use `<link rel="canonical" href="" />` to tell search engine the canonical domain. The search engine will treat both the domains as different, and penalize for duplicate content/spam and lower the page rank.
- Do not use CNAME for the redirect. According to DNS specification, any domain that has a CNAME record set cannot have other DNS records associated with it. Instead use the A record to point to a server and from the server do HTTP 301 redirect.
- It is more important to mantain consistency than deciding which url to use.

Benefits of www

- At high traffic volumes (millions of daily users) `www` makes more sense.
- Easier to manage DNS settings when using `www`.
- Handling cookies is easier. A cookie set on `example.com` would apply to all subdomains as well.
- CNAME record cannot be added to root domain. CDNs rely on CNAME records and using a non-www domain makes it harder to redirect traffic when servers experience issues. `www.example.com` can be redirected to `mycdn.provider.net` at the CDN level and `mycdn.provider.net` target can be changed whenever traffic needs to be rerouted.

## Type of URL

### Absolute URL string

`https://www.example.com:80/path/to/file.html?key1=value1&key2=value2#SomewhereInTheDocument`

- Scheme `http` - Specify the protocol that the browser must use to request the resource.
- Authority `www.example.com:80` - Separated from the schema using `://`. Includes both the domain `www.example.com` and the port `443` (ommitted if using standard ports 80 for HTTP and 443 for HTTPS).
- Path to resource `/path/to/file.html` - Path to the resource on the web server. It can be an actual file or an abstraction handled by web servers without the presence of an actual file.
- Parameters `?key1=value1&key2=value2` - The GET parameters.
- Anchor `#SomewhereInTheDocument` - Location inside the resource itself.

### Relative URL string

For this absolute URL `https://developer.mozilla.org/en-US/docs/Learn_web_development`

- Scheme-relative URL `//developer.mozilla.org/en-US/docs/Learn_web_development` - The browser will use the same protocol as the one used to load the document hosting that URL.
- Domain-relative URL `/en-US/docs/Learn_web_development` - The protocol and domain name will be used from the host URL.
- Sub-resource `HowTo/Web_mechanics/What_is_a_URL` - Browser will use path relative to `en-US/docs/Learn_web_development`. So the final URL becomes `https://developer.mozilla.org/en-US/docs/Learn_web_development/HowTo/Web_mechanics/What_is_a_URL`. We can use `..` to move up a directory.
- Anchor-only `#semantic_urls` - To link to different parts of the current document, as the browser will use the entire URL.

### HTTP Authentication

`https://username:password@www.example.com:80`

- To immediately sign in to a website and bypass the username/password dialog box.
- Deprecated in modern browsers. And the username/password info is stripped from the request before it is sent.

## Best practices

- Keep it short and memorable, in case someone has to manually write down the URL in the browser.
- Only use lowercase, as it removes the ambiguity, and also the official URL spec says to use lowercase.
- Use readable slug instead of ID. A readable slug makes it easier to glean the content of the page by just looking at the URL.
- Do not use spaces. Spaces also make it harder to identify the end of the URL.
- Use hyphens instead of underscore, use kebab case. Also, when links are underlined on a page, the underscores get hard to see.
- Use period only for domains and indicating file extensions.
- If possible avoid using `www` in the URL.
- Do not have `.html` in the URL, as it adds unncessary length.
- Avoid trailing slash where it does not make sense. `example.com/posts/` makes sense, since there is further content within `posts/`. But `example.com/posts/post-title/` is bad.
- URL's structure should work as breadcrumb. And the user should be able to make their way back along the path without error at any stage.
- Avoid date paths if you can `example.com/posts/2025/02/24/post-title`. It makes sense on news sites where there are multiple items releasing daily. Also, the date adds extra length to the url.

## URL architecture

- Flat URLs trade determinism for aesthetics. For example, the url `/leather-jacket`can imply
    - A product named "Leather Jacket".
    - A category named "Leather Jacket".
    - A blog post named "Leather Jacket".
    - A landing page for a "Leather Jacket" campaign.
    - Nothing at all (a 404).
- A solution to above is making a database call for URL Resolution Table (map of clean URL to their corresponding entity types and IDs).
- In structured URL system `/product/leather-jacket` the routing decision is instant. In flat URL, you need to make a backend call to check if leather jacket is product or category.

### Migrate to structured URL

- To migrate your website from clean URLs to structured URL (close enough)
    - Create a cache of things that don't change frequently (like categories).
    - When a URL is hit, check if it is a category.
    - If not, check if it is a product, otherwise show 404.

### Suggestion

- Use deterministic URLs
    - `/product/leather-jacket`
    - `/category/outwear`
    - `page/about-us`
- Later if you want to remove structure, use 301 redirects.

## URL as state management

- Store all the relevant info about a page in the URL like every checkbox, dropdown to match the user configuration. The entire page reconstructed from a single URL.
    - The URL in this case is storing state, encoding intent, and making the entire setup shareable and recoverable. No database, no cookies, no localstorage.
    - URL is state management tool in this case.
- URLs provide the following for free
    - Shareability. Send someone a link, and they see exactly what you see.
    - Bookmarkability. Save a URL, and you've saved a moment in time.
    - Browser history. The back button just works.
    - Deep linking. Jump directly into a specific application state.
- Ways to provide query information in the URL (you can provide information in anchor as well)
    - Multiple values with delimiters like `+`, or `,`.
        ```
        ?languages=javascript+typescript+python
        ?tags=frontend,react,hooks
        ```
    - Nested data by using key-value pairs separated by comma. Or serialize JSON or base64-encode for safety.
        ```
        ?filters=status:active,owner:me,priority:high
        ?config=eyHNrwislsCj9==
        ```
    - Boolean flags.
        ```
        ?debug=true&analytics=false
        ?mobile
        ```
    - Arrays
        ```
        ?tags[]=frontend&tags[]=react&tags[]=hooks
        ?ids[0]=42&ids[1]=73
        ```

### What to include in URL

- Search queries and filters.
- Pagination and sorting.
- View modes (list/grid, dark/light).
- Date ranges and time periods.
- Selected items or active tabs.
- UI configuration that affects content.
- Feature flags and A/B test variants.

What not to include

- Sensitive information (passwords, tokens, personally identifiable information).
- Temporary UI states (modal open/closed, dropdown expanded).
- Form input in progress (unsaved changes).
- Extremely large or complex nested data.
- High frequency transient states (mouse position, scroll position).

### Implementation

```js
const params = new URLSearchParams(window.location.search);
const view = params.get("view") ?? "grid";

// Update URL info
params.set("sort", filters.sort);
const newUrl = `${window.location.pathname}?${params.toString()}`;
window.history.pushState({}, "", newUrl);
renderContent(filters);

// Handle back/forward buttons (mostly handled by the framework)
window.addEventListener("popstate", () => {
    const params = new URLSearchParams(window.location.search);
    const filters = {
        status: params.get("status") || "all",
    };
    renderContent(filters);
});
```

### Best practices

- Do not include default values. Use defaults in code when reading parameters.
- Use _debouce_ for high-frequency updates (like search).

    ```js
    const updateSearchParam = debounce((value) => {
        const params = new URLSearchParams(window.location.search);
        if (value) {
            params.set("q", value);
        } else {
            params.delele("q");
        }

        window.history.replaceState({}, "", `?${params.toString()}`);
    }, 300);
    ```

- Use `popState` when you want to create a new history entry (for distinct navigation actions like changing filters, pagination, navigating to new view). Users can use _Back_ button to return to the previous state.
- Use `replaceState` for making refinements such as search-as-you-type or minor UI adjustments where you don't want to flood the history with every keystroke.

### Other use cases

- Good URLs let users understand what they are looking at.
- URLs are cache keys. Query parameters define cache variations. And CDNs can cache intelligently based on URL patterns.
- User's journey on your website can be visualized without any extra tracking code. Just track the URL history.

## URL length

As of Sep 2023

- 2000 characters limit works virtually everywhere.
- 8000 characters works well outside search engines.
- Internet explorer 8's maximum URL length is 2083 characters. Limit is same for IE9. For IE10 and IE11 the search bar only shows 2083 characters, but you can click on links with longer length.
    - IE won't bookmark URLs longer than 260 characters.
- Sitemap has a URL limit of 2048 characters.
- Research on search engines show a limit of around 2047 characters, aligning with sitemap limit.
- Google Search Engine Results Page (page that shows after you search for a query), can't handle links longer than 1855 characters.
- CDNs have limits as well
    - Fastly - 8Kb (8192 characters)
    - CloudFront - 8Kb
    - Cloudfare - 32Kb
