---
title: URI
parent: SEO
nav_order: 1
---

<!-- prettier-ignore-start -->
# URI (Uniform Resource Identifier)
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

-   [link](https://developer.mozilla.org/en-US/docs/Web/URI) MDN Reference: URIs

---

Uniform Resource Identifier (URI) used to identify "resources" on the web.

## Scheme

URI begins with `protocol:` where _protocol_ is the scheme and is used to indicate which protocol the browser must use to fetch the resource.

-   `blob:<origin>/<uuid>` - Pointer to a large in-memory binary object which can be `Blob`.
    -   Can be used to trigger downloads of locally generated data using `URL.createObjectURL()` and `URL.revokeObjectURL()`.
    -   Only free the object URL when you remove the element from the DOM. Since doing early would prevent the user from right-clicking image to save or opening in new tab.
        ```js
        const canvas = document.querySelector("canvas");
        canvas.toBlob((blob) => {
            const img = document.createElement("img");
            img.src = URL.createObjectURL(blob);
            document.body.appendChild(img);
        });
        // ...
        const img = document.querySelector("img");
        img.remove();
        URL.revokeObjectURL(url);
        ```
    -   Browsers can enforce `noopener` when navigating to blob URL.
    -   Blob URLs can be fetched with `Range` HTTP header to specify the bytes you want.
        ```js
        const url = URL.createObjectURL(blob);
        fetch(url, {
            headers: {
                Range: "bytes=7-11",
            },
        }).then((res) => res.text());
        ```
-   `data:[<media-type>][;base64],<data>` - Embed small files inline in documents. Useful performance optimization in web development as you can embed small assets like fonts, images in the source code itself, rather than making HTTP request.
    -   `[<media-type>]` - Specify MIME type.
    -   `;base64` - Indicate data should be base64 decoded.
    -   `<data>` - Data with percent-encoded for spaces, and other stuff.
        ```
        // In linux
        > base64 a.text > b.txt
        > echo -n hello|base64
        ```
-   `file:` - Host specific file name.
-   `http:` / `https:` - For HTTP protocol.
-   `javascript:` - Embed JavaScript in the URL. (do not use)
-   `mailto:` - Email address.
-   `ssh:` - For SSH protocol.
-   `tel:` - Phone number.
-   `urn:` - Uniform resource name, to identify resources in a permanent way. This is useful in a library, where the identifier of the book (like isbn) will always remain the same and can be used to identify the book, regardless of the location of the book.
-   `view-source:` - Source code of the resource.
    ```txt
    urn:isbn:1234567890
    urn:ietf:rfc:2648
    ```
-   `ws:` / `wss:` - Websocket connection.

A complete list of all schemes can be found at [IANA](https://www.iana.org/assignments/uri-schemes/uri-schemes.xhtml) website.

## Authority

Comes after scheme to provide username, host, port information.

```
host
host:port
user@host
user@host:port
user:password@host:port
```

-   `host` - domain name or IP address of the server hosting the resource.
-   `port` - the port on which the server is listening for requests. It defaults to 80 for HTTP and 443 for HTTPS.

Example

```
http://www.example.com:80/path/to/file.html
postgresql://postgres:admin123@db:5432
    - postgres - user
    - admin123 - password
    - db - host
    - p5432 - port
```

## Path

Comes after authority and continues till `?` or `#`. Every URI has a path, so when you see a URL `https://example.com` a trailing slash is added to it `https://example.com/` to specify `/` as the path of the URI.

## Query

Comes after path. Starts with `?` and ends at `#` (if specified). Syntax `?query`.

## Fragment

Last part of URI, used to identify a specific part of the resource. It is not sent to the server when the URI is requested, instead it is processed by the client after the resource is retrieved.

This means in the backend the origin URL will not contain fragment.

### Text fragment

-   Used to link to text contents in a page, without having to use ID. The browser will scroll to the text and highlight with color.
-   This solves the problem where you want to link to a particular section in a website, that you don't have control over (the content does not have ID).
-   Use `noopener` when linking to a cross-origin page.
-   Text is not searched in `<iframe>`.
-   Matches are case-insensitive.
-   Use `Document-Policy: force-load-at-top` to opt-out of text fragments (chromium only).
    Syntax

```
https://example.com#:~:text=[prefix-,]textStart[,textEnd][,-suffix]

# Multiple fragments
https://example.com#:~:text=fragment1&text=fragment2
```

-   `:~:` - Fragment directive to mark the start of user-agent instructions (directives). These are stripped form the URL during loading so that author scripts cannot directly interact with them.

Example

-   [https://kushaj.com/seo/uri.html#:~:text=to-,trigger,generated,-data](https://kushaj.com/seo/uri.html#:~:text=to-,trigger,generated,-data) - Get text between _trigger_ and _generated_.

Style the selection

```css
::target-text {
    background-color: rebeccapurple;
    color: white;
}
```
