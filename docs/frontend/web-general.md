---
title: Web General
parent: Frontend
nav_order: 1
---

<!-- prettier-ignore-start -->
# Web General
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

-   MDN Learn
    -   [link](https://developer.mozilla.org/en-US/docs/Learn_web_development/Getting_started/Environment_setup/Browsing_the_web) Browsing the web
    -   [link](https://developer.mozilla.org/en-US/docs/Learn_web_development/Getting_started/Environment_setup/Dealing_with_files) Dealing with files
    -   [link](https://developer.mozilla.org/en-US/docs/Learn_web_development/Getting_started/Web_standards/How_the_web_works) How the web works
    -   [link](https://developer.mozilla.org/en-US/docs/Learn_web_development/Getting_started/Web_standards/The_web_standards_model) The web standards model
    -   [link](https://developer.mozilla.org/en-US/docs/Learn_web_development/Getting_started/Web_standards/How_browsers_load_websites) How browsers load websites
    -   [link](https://developer.mozilla.org/en-US/docs/Learn_web_development/Extensions/Client-side_tools) Understanding client-side tools
    -   [link](https://developer.mozilla.org/en-US/docs/Learn_web_development/Extensions/Server-side) Server-side websites
    -   [link](https://developer.mozilla.org/en-US/docs/Learn_web_development/Howto/Web_mechanics) How to solve common problems (Web mechanics)
-   [link](https://wpc.guide/) Web Platform Contribution Guide
-   [link](https://resilientwebdesign.com/) Resilient web design by _Jeremy Keith_

---

## Terminology

-   **Web page** (or "page") - Document that can be displayed in a web browser, like a HTML document.
-   **Website** (or "site") - Collection of web pages grouped together into a single resource, with links connecting them together.
-   **Web server** - Computer that hosts a website on the internet.
-   **Web service** - Software that responds to requests over the Internet to perform a function (like handle user login) or provide data (like weather report).
-   **Search engine** - Web service that helps people find web pages, such as Google, Bing.
-   **Interoperable** - Different implementations behave exactly the same for a given case i.e. browsers should provide the same rendered output for a given input.
-   **URL** - Uniform Resource Locator is a web address (kushaj.com) plus a protocol (https).

## Domain name

-   IPv4 - `192.0.2.172`
-   IPv6 - `2001:db8:8b73:0000:0000:8a2e:0370:1337`
-   Domain name (separated by dots and read from right to left)
    -   Top-Level Domain (TLD) - `.com`. Tells the general purpose of the service behind the domain name. Some TLDs impose restrictions like `.gov` are only allowed to be used by government.
    -   Label (or component) - case-insensitive character sequence from 1 to 63 characters in length.
        -   Secondary Level Domain (SLD) - `mozilla.org` here `mozilla` is Label 1 also called SLD. The label located right before the TLD is called SLD.
        -   Subdomain - `developer.mozilla.org` here `developer` is Label 2 and also servers the purpose of subdomain.
-   You never own a domain name. Instead you pay to use domain name for a year or more. You can renew your right and your renewal has priority over other people's applications.
    -   Companies called registrars book keep domain name info and administrative stuff.
    -   Private companies can also maintain their own domains, like Amazon manages domains under `.fire`.
-   DNS Refreshing - DNS servers refer to "top-level DNS servers".
    -   Each DNS server that knows about a given domain stores the information for some time before it is automatically invalidated and then refreshed (the DNS server queries an authoritative server and fetches the updated information from it).

### How DNS request works

1. Type `mozilla.org` in browser search bar.
2. The browser first checks local DNS cache on your computer. If it is a hit, then make request to web server.
3. The browser then makes request to DNS server, which returns the IP address.

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

## How the web works

When a URL is typed into browser address bar, the following steps occur

1. The browser goes to the DNS server and finds the real address of the server that the website lives on.
2. The browser sends an HTTP request message to the server, asking it to send a copy of the website to the client (TCP/IP is used for sending the data).
3. If the server approves the client's request, the server sends the client a "200 OK" message, and then starts sending the website's files to the browser as a series of small chunks called data packets.
    - Reason for using small packets: Easier to replace dropped or corrupted packets. Also, the packets can be routed along different paths, making the exchange faster and allowing many users to download the same website at the same time (since the browser can start showing one packet at a time).
4. The browser assembles the small chunks into a complete web page and displays it to you.

## Web Platform

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

### JavaScript

_John Postel_ (worked on ARPANET, precursor to Internet) created the Robustness Principle, also known as Postel's Law

> Be conservative in what you send; be liberal in what you accept.

This philosophy is true for declarative languages like HTML, CSS. Declarative language is where you describe the desired outcome without providing step-by-step instructions on how to achieve it.

Imperative languages provide more power than declarative languages. The main downside being sytax needs to be perfect, hard to learn. In the browser, the only choice for imperative language is JavaScript.

In 1996, _Brendan Eich_ working at Netscape created the first imperative language for the web in 10 days. First it was called _Mocha_, then launched officially as _LiveScript_, then marketing department changed it to _JavaScript_, hoping to ride the Java wave hype.

Two main early uses were form validation (to get quicker feedback then getting response from server), and rollovers (swapping out images to mimic hover effect).

Over the history of JavaScript, features that became popular were moved to HTML, CSS (as it is easier to write declarative langauge).

_Ethan Zuckerman_ used JavaScript to spawn window with an advertisement in it.

In 2005 _Jesse James Garett_ published an article _Ajax: A New Approach to Web Applications_, which popularized a technique for JavaScript to send and receive data from a web server without refreshing the whole page, resulting in smoother user experience.

_Tim O'Reilly_ used the phrase _Web 2.0_ around the same time to describe new wave of web products and services. Not clear what exactly made Web 2.0 (rounded corners, gradients or JavaScript, Ajax or new business models). Launching the age of _web apps_ (hard to provide definition except use cases).

After the standardization of HTMLv4 in 1999, the _World Wide Web Consortium_ published XHTML 1.0. It was a stricter way of writing HTML (moving HTML from declarative to imperative side). XHTMLv2 was abandoned. Interseting how later everyone moved to JavaScript which is also imperative.

### Web Platform

Platform is an incrorect term, as platforms are controlled and predictable. WWW is the opposite.

iOS is a platform. If you build an ios app and someone has an ios device, you know they will get 100% of your software. On Android they will get 0%.

Web on the other hand is a _continuum_. You can't be sure how many web technologies will be supported. Some people will get 80% or 90%, others 20%.

### Progressive enhancement

What does designing a thing in layer looks lik

-   A chair in a room.
-   A room in a house.
-   A house in an environment.
-   An environment in a city plan.

In web the benefit of layering is separation of concerns, which allows enhancements to be applied according to the capabilities of the end user's device.

Tha layers of web

-   JavaScript and CSS build on top of HTML.
-   HTML requires a URL to be reachable.
-   URL reachability depends on HTTP protocol.
-   HTTP protocol depends on TCP/IP.

Progressive enhancement asks that designers start from the lowest common denominator (well marked-up document), and then add as many features on top as they want. If any feature is not supported, the browser will just ignore it. Websites do not need to look exactly the same in every browser. People using the old browsers should get the same content as the new browsers, but that dosen't mean they should get the same experience as well.

Progressive enhancement means providing core functionality to everyone. The enhancements can be browser specific.

Use this approach to implement progressive enhancement in web (this approach can help with technical debt, as you can throw the enhancements and start over)

1. Identify core functionality (most people come to your website to read, write, buy, sell, not to tap or scroll).
2. Make it available using the simplest possible technology.
3. Enhance, which can include adding interactions like swiping, tapping, clicking, scrolling, dragging, dropping.

Example news website

1. Core functionality is to show the article. So serve plain HTML at a URL (the url should be simple and easy to read and share).
2. Enhance, by adding your own stylesheet.

Example social network

1. Core functionality is sending and receiving messages. For displaying message, plain HTML is enough. For sending message, HTML with form and a button for submit is enough.
    - People can now send and receive messages no matter that device or browser they are using.
2. Improve the experience without breaking the fundamental activity.
