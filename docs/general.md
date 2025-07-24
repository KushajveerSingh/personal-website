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

> [link](https://wpc.guide/) Web Platform Contribution Guide

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

## History of web

> [link](https://resilientwebdesign.com/) Resilient web design by _Jeremy Keith_

### HTML

Tim-Berners-Lee working at CERN in 1980s, created a personal project _ENQUIRE_ to manage information. To expand the program to large amount of data being created at CERN, he pitched the idea in _Information Management: A Proposal_, which became the World Wide Web. CERN already had a network of networks, connected via telephone wires, since 1960s and the early adopters were universities and scientific institutions.

Initial research was funded by _DARPA_, but the engineers designed the network to withstand censorship, not a nuclear attack. This is reflected in the protocoals designed as well, like TCP/IP only cares about how packets of data should be moved around, and not about the content of the packets.

Hypertext was coined by Ted Nelson, who was originally working on his own hypertext system called Xanadu. Both Ted Nelson and Tim Berners-Lee were influenced by Vannevar Bush, who was in turn influenced by Paul Otlet. Tim though, people would hypertext to link to non-HTML resources, like word, excel, other computer files. But people began to create fully-fledged documents directly in HTML.

CERN was using SGML internally, and Tim Berners-Lee used SGML as a starting point for HTML. And the first version came out with 21 elements, with a proprietary element _NEXTID_, which only worked on Tim's machine running _NeXTSTEP_ operating system. Tim created the first web browser, called _WorldWideWeb_, which only worked on _NeXT_ machines.

Nicola Pellow, an intern at CERN, created the first cross-platform browser called _Line Mode Browser_, making the web accessible to everyone.

To solve issues with interoperability i.e. what should browser do when it encounters something that it doesn't understand, browsers will ignore tags they don't understand. This allows to add new tags, and knowing exactly how old browsers will treat it; they will ignore the tag and display the content.

### CSS

New tags kept being added to HTML, but most of these tags were focused on visual than the meaning of content. Hakon Wium Lie, also working at CERN at the same time as Tim Berners-Lee, proposed a Cascading Style Sheets to describe the presentation of HTML documents. And together with Bert Bos, they created the syntax of CSS.

In 1996 David Siegel published _Creating Killer Websites_ book, outlining series of ingenious techniques for creating eye-catching designs out of the raw HTML tags. Like using a transparent GIF as an IMG element to control amount of whitespace in the design. Or using TABLE element to recreate any desired layout.

Web designers didn't use CSS because of the browser war between Microsoft Internet Explorer and Netscape Navigator, who would invent their own separate HTML elements, and designers were forced to write for both browsers. A group of web designers formed Web Standards Project, and began lobbying Microsoft and Netscape to abandon their proprietary ways and adopt standards.

Dave Shea created CSS Zen Garden to showcase what could be done with CSS. And more importantly separate the concern between HTML and CSS.

### Designing

In 20th century, Swiss style was created to provide guidelines for designing for print pages. This included using grid systems and typographic scales based on the preceding centuries of design. Knowing the ratio of the dimensions of a page, designers could position elements with maximum effort (page is constraint, and grid system is used to impose order).

When transitioning to web, David Siegel's _Creating Killer Websites_, showed everyone how TABLE and GIF hacks can be used to create any websites. So designers took the same approach from print media (where page ratio is fixed) to the web, even though the browser window if not of fixed size.

It started with designing for monitors that are 640 pixels wide, then 800 pixels, 1024 pixels, and ultimately settling oni the magic number 960 pixels.

One reason why this happened, is because there were no tools created specifically for visualizing layouts on the web. Instead designers had to use the existing tools, which were focused on print media.

Photoshop was one tool utilized by graphic designers, which started with fixed canvas size. Then came _Macromedia's Dreamweaver_ for web design, which operated on WYSIWYG (What You See If What You Get), which would not always work on the web. Some assumptions that were prominent in first decade of twenty-first century

-   everyone was browsing with a screen wider than 960px.
-   everyone had broadband internet access, so no need to optimize the number and file size of images.
-   everyone was using a modern web browser with the latest plug-ins installed.

In 2007, Steve Jobs released iPhone which can be used to browse the Web. Before iPhone, mobile devices could only display a specialized mobile-friendly file format called WML.

Designers started with segmenting desktop (m.example.com) and mobile (mobile.example.come) into separate domains. This approach was termed _the mobile web_. User-agent of the browser was used to identify what subdomina to use. But this became cumbersome as the list of browsers increased. And with the introduction of iPad, the distinction between mobile and desktop became less clear.

Over time the illusion of one-size-fits-all approach to web design began to evaporate, and was gradually replaced with the acceptance of the ever-changing fluid nature of the web.

In April 2010 _Ethan Marcotte_ gave a talk at _An Event Apart_ in Seattle, about reponsive design in architecture. The idea that buildings could change and adapt according to the needs of the people using the building. One month later he expanded the idea in an article called _Responsive Web Design_, published on _A List Apart_ (referencing _A Dao Of Web Design_ article by _John Allsopp_). Article by John by originally rejected by the community since they did not want to change their ideaology from print media.

_Luke Wroblewski_ cointed the term _Mobile First_.

Orignally, web design was dictated by the designer, with users having no choice but to accomodate the site's demand, like screen size, or network speed. Now web design can if focused on the universal nature of the World Wide Web i.e. everyone should be able to use the web.

https://resilientwebdesign.com/chapter4/
