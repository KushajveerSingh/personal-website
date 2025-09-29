---
title: URI
parent: SEO
nav_order: 1
---

<!-- prettier-ignore-start -->
# Uniform Resource Identifier (URI)
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

URI begins with `protocol:` where _protocol_ is the scheme and is used to indicate which protocol the browser must use to fetch the resource. It should only contain alphanumeric characters, `+`, `-`, `.`.

-   `blob:<origin>/<uuid>` - Pointer to a large in-memory binary object which can be `Blob`.

    -   Can be used to trigger downloads of locally generated data using `URL.createObjectURL()` and `URL.revokeObjectURL()`.
    -   Only free the object URL when you remove the element from the DOM. Since doing early would prevent the user from right-clicking image to save or opening in new tab.
    -   Browsers can enforce `noopener` when navigating to blob URL.

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

    Other examples

    ```js
    // Example of origin
    const textBlob = new Blob(["Hello, World!"], { type: "text/plain" });
    const blobUri = URL.createObjectURL(textBlob);
    // blob:https://example.com/c40a5f22-38d7-463d-82d8-e7c65c29219e

    // Example of UUID
    const imageBlob = new Blob(
        [
            /* binary data */
        ],
        { type: "image/jpeg" }
    );
    const opaqueBlobUri = URL.createObjectURL(imageBlob);
    // blob:uuid:34778130-14e3-4f18-97a6-f308a0d92295
    ```

-   `data:` - Data directly embedded in the URL.
-   `file:` - Host specific file name.
-   `ftp:` - For file transfer protocol.
-   `http:` / `https:` - For HTTP protocol.
-   `javascript:` - Embed JavaScript in the URL.
-   `mailto:` - Email address.
-   `ssh:` - For SSH protocol.
-   `tel:` - Phone number.
-   `urn:` - Uniform resource names.
-   `view-source:` - Source code of the resource.
-   `ws:` / `wss:` - Websocket connection.

A complete list of all schemes can be found at [IANA](https://www.iana.org/assignments/uri-schemes/uri-schemes.xhtml) website.

continue form data urls https://developer.mozilla.org/en-US/docs/Web/URI/Reference/Schemes/data
