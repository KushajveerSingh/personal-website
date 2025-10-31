---
title: Projects
search_exclude: true
nav_exclude: true
---

<!-- prettier-ignore-start -->
# Projects
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

## Personal Values

-   Limit network access to core functionality (no automatic check for updates, ...).
    -   Support offline mode. Store local data in simple text formats without encryption so users can view it without the app. If encryption is needed, provide easy access to keys so users can decrypt data outside the app.
-   Make software open-source (optionally open-contribution).
    -   Provide detailed documentation so users can modify the source code for their needs. This includes writing software that is easy is maintain, and expand. This allows users to debloat the software per their needs. The documentation on customizing software can go as deep as the last if-condition.
        -   For example, if the application supports two payment methods (check and payment processor).
        -   Write each type in a different file, with common API structure.
        -   This allows user to delete one type of payment, or add a third type easily.
    -   Never force the user to read the source code to infer logic, valid config options.
    -   Provide detailed documentation on DevOps. This lets advanced users integrate and customize the app within their DevOps environment.
        -   Allow users to run software locally. If the application can benefit from peer-to-peer, add support for that as well.
        -   When applicable, create a centralized website to list local servers.
-   Meet accessibility standards.
-   No nuisances which includes
    -   Forced artificial intelligence.
    -   Recommendation algorithms which need user data, history, third-party info.
        -   Recommendations derived from a user’s follows and their extended network are permitted.
        -   This has downside of confining people to a bubble. Add random recommendations to counter this.
    -   Dark patterns.
    -   Forced advertisements.
    -   Changes to UI/UX should be opt-in, not forced.
        -   Made new UI available through an opt-in toggle.
        -   Provide documentation on how to move from old UI to new UI. Split the documentation as
            -   New functionality.
            -   Deprecated functionality. Provide reason.
            -   Moved functionality.
-   Optimize software, till limited by network, disk I/O.
    -   Prefer hardware acceleration.
-   Support greatest number of platforms.
    -   Ability to use the entire app through API.
        -   Allow users to create plugins.
    -   Command line support.
    -   Terminal user interface support. Use API to create simplified interface.
-   Remain neutral. Avoid influence from politics, religion, geopolitics, or other external agendas.
-   Respect right to repair. When making hardware do not employ dark patterns to make it harder for people to access, modify, and repair the machine as they wish to.
-   An exception to the rules can be created for organizations that don't respect these rules1.

---

## Microsoft Migrate

-   For all Microsoft products created Excel, Word, VB Script, Access, Azure create alternate tooling to migrate to. For each tool you will have to look at the various versions throughout the years as well.

## OpenKush

-   Create open source version for services that can be considered basic utilities in modern digital world.
-   Register main domain `openkush.com`.

### OpenPay

-   Use domain `usa.pay.openkush.com`.
-   Payment processsor to rival Visa, Mastercard, Stripe, Square.
-   The cost of the system will be distributed equally amonst people based on the usage.

### OpenBank

-   Use domain `usa.bank.openkush.com`.
-   Bank which is only used for debit account (checking account).
-   Can expand to savings account and take no cut from the investment.
-   The payment processor can now become independent from other banks.
-   The cost will be supported through OpenPay, since you don't want people to lose money sitting in their account.

### OpenTax

-   Use domain `usa.tax.openkush.com`.
-   Can be added as a feature of the bank or an independent product.
-   Convert the entire tax code to a software which makes paying taxes as simple as clicking a single button.

### OpenTLD

-   In addition create your down top-level domain `.kush` and get free from ICANN.
    -   Create own ip standard with maybe 5 parts similar to ipv4 and more bits for each part. Use the syntax 123-234.x.x.x to signify the ip ranges. In this way the ip string will always include the 3 dots and either have two numbers in each section separated by a dash.

### OpenISP

-   Create your own ISP.
-   Include phone number with additional digit if necessary.

### OpenAuth

-   Auth service to let people host locally or use the provided cloud server.
-   To replace Google login.

### OpenShop

-   Shop service similar to Amazon. Delegate the delivery to UPS, Fedex.
-   It is important for people to sell/buy products freely.

### OpenPoll

-   Poll/voting service to let people create polls and vote on features.

### OpenSocial

-   Social media that keeps the content only for certain timeframe.
-   The idea being social media is important to let people communicate with others in case of emergency.
-   Extend this to include chat messaging, live video (discord, slack).

### OpenPhone

-   Create a phone with open-source blueprint and modular approach.
-   Every component can be individually purchased and integrated into the system.

### OpenInsurance

### OpenBrowser

-   Create own internet specification, and club them into Internet v2.
-   Browser will not have cookies. auth/pay will be controlled by the browser. accessibility/design system will be controlled by browser.
-   In the frontend framework, use directory structure to define functionality
    -   keyboard shortcuts, event listeners (reactive programming).
    -   css modes like print, in separate file.
-   Add popular fonts to the browser.
-   Define a tree like data structure for defining the laod order of js files. Parallel branches can be fetched in parallel (similar to async).
-   For global account like in Chrome you sign in with Google, use OpenAuth or a local version.
-   After a working browser has been created. Create a clone of all the popular apps, if multiple can be consolidated into one do that. And make these part of the browser.

### OpenFoodTest

-   A better alternative to department of health and usda, and an unbiased testing facility for food products and recommendations.
-   Add toos like calorie tracker and thenutrient breakdown of all foods you eat. Include supplementation.

## SEO

-   Combine all SEO products from [link](https://github.com/serpapi/awesome-seo-tools) into a single app.
    -   Add these as well [link](https://kushaj.com/product-design/seo).
    -   Wordpress aioseo.
-   Reference SEO notes for more ideas.

## Better Tech Name

-   Use domain `bettertechname.com`.
-   Come up with alternative names for GPUs/CPUs/RAM/Monitors (desktop, enterprise, mobile).
-   General strategy: bigger the number better the product.
    -   Consult research papers to understand the architecture of each product.
    -   Each product begins with the year the architecture was released not the product was released.
-   Have a (autogenerated) section for each product to explain the rationale behind the name.

## Diagram

-   Reference [UI LIbrary-2D visualization](#2d-visualizations) before working on this.
-   Reference all the diagramming tools out there and create a single tool with all the features.
-   Include collaborative features as well using local server.
-   Make it work online completely client side, with option to sync to storage like google drive. Add option to share with people.
-   Create a programming language interface to draw the diagram through code.
-   Include simple diagrams, software architecture diagrams, devops diragrams, charts (look at charting libraries), graphs.

## Programming Langauge

-   Create transpiler for programming languages to Rust. For each language look at the top libraries and transpile them as a proof of concept. Also, check performance of the transpiled to the original.
    -   If a language has a specific tooling built around it, build that for the Rust version as well.

## Database

-   Go through postgres documentation and create an admin panel to configure all the settings, this includes options for compiling postgres from source.
    -   For options which can take multiple values like page size, offer a testing utility which users can use to get the best value.
-   Go through all the Postgre Weekly newsletters and add stuff from there.
-   Look at databases built on top of Postgres and merge their features.
-   Look at databases outside of Postgres and merge their features.
-   Create transpiler and tooling to migrate from other databases to postgres.
    -   Access

## HTTP

-   Tool for debugging HTTP requests, similar to Proxyman, Chrome Dev Tools.
-   Add `curl` integration for making requests from interface, and seeing request, response information in detail.

### CSP

-   Create a tool similar to [csper](https://csper.io/) to track/report CSP violations, and suggestions on resolving.
-   Option to add HTML head tags to resolve the issue.
-   Option to add HTTP headers to resolve the issue.

## Coding Live Documentation Viewer

-   When writing say Rust, depending on where cursor is, show the official documentation for that thing in a browser window. As you type, the page keeps getting updated.
-   For this to work, offline documentation would work best. And for navigation using URL (file path) would be best.
-   In addition show custom user documentation, which can include like performance characteristics of each thing, accessibility, security, usage examples, best practices.
-   Another pane can show the piece of code at different compiler stages.

## UI library

-   Create a UI library for Qwik js (or framework agnostic) with inspiration from shadcn.
-   Add design design from UI collective. Split it into files, with the base where you define the colors, and other values being in a separate file, that can be exported and imported easily.
-   Look up all the UI libraries out there and add their components.

### Storybook

-   Render the component at different resolutions, different browsers.
-   It integrates with the design system, and provides a page to adjust the design system and see the new rendering of the components. From the page, the design system can be exported to be used in your library (or just saved directly to the file).

### Accessibility

-   In the storybook app, add diffeerent accessibility renderings as well.
-   Integrate everything related to accessibility in the app. Things like stark app.

### 2D-visualization

-   Charts.
-   Graphs.
-   Diagrams. Refer [Diagrams project](#diagram) as well, as that will build on top of this.
-   Some diagrams where it is of video format, to show flow of data (make it interactive, have the ability to hover and see tooltip). This might be int he realm of 3Blue1Brown animation library.

### 3D-visualization

### Maps

### Math

-   Look at 3Blue1Brown animation library.

### Random

-   Look for niche topics, like electronic circuits.

---

## Reorganize

Write a paper on how doing multiple sports can prevent injuries. Look at different sports, and parts they normally use, and make a series of sports that cover all parts of the body, without overusing the same part over and over again.
Look at mentality, physicality aspect of the sports as well.
Give suggestions like playing with old people, young people, left/right hand, playing with woman.
Goal: If you are a kid use this protocol, and this might help you with reaching professional league, but this will defintiely hep you stay healthier and injury free in your 60s.

Lims system using quickjs

Usa militiary industry complex
Show the people who jokd positions and where they held before. And how these people are related to each other.
For the grants, see which people approved.
In a table show all the people in militiary industry complez, have a button to show that persons last positions, relationship to others in the table, grants accepted.
Show how much money is spent lobbying by each company and at which locations.

create a frontend function to check for caniuse. It checks the browser name (there is a js package for that), and then compares the version with caniuse, to return a boolean if the feature is supported or not. Create a similar function for backend.

when creating lims system for aesl with qwik js, be active in qwik discord and showcase what you are doing. Also, opensource all the work like authentication pages, and stuff like that so that others can use from the community.

create visualization like tigerbeetle simulator, for all distributed algorithms and database

## Astro

### Asset shortcut plugin

-   Make referencing asset folder easier. Currently to access the images, you have to use `../../assets`. But we can use `~` to reference `assets` by default.
-   This is also true, when using `baseUrl`. As all the relative links have to add `baseUrl`.
-   Using relative URLs should also be improved by this plugin. And this should take into account `baseUrl`, `trailingSlash` values as well.
-   Modify Kushajveer.github.io repo after the project is implemented.

## Starlight

### Documentation clone

-   There are a tone of features in Docusaurus (and other static site generators like hugo, eleventy, gatsby, jekyll, material for mkdocs, sphinx, read the docs, asciidoc, bookstack, confluence) that are missing in Starlight.
-   Clone starlight and create a new theme, with all these features included.
-   Include remarkplugins, rehypeplugins.
-   Add blog support (plus additional starlight plugins).
-   Add protected pages for internal docs.
-   Integrate all TODO from code in a single page.

### Sidebar utils plugin

**Motivation** \
Show "Internal" in the sidebar only for the _internal_ subroute. Also, create a table of contents like page for "kushajveersingh.com/internal" which has the same content as the "internal" in the sidebar. This has the benefit

-   people don't have to create the page manually.
-   people can go directly to "internal" and look for what they want, rather than going to "internal/something".

Other stuff

-   Add option to collapse sidebar like Docusaurus and look for other features provided by Docusaurus sidebar.
-   Look at other Starlight plugins related to sidebar.

## Prettier

### Plugin markdown codeblocks

-   Create a Prettier plugin, that disables formatting of codeblocks in Markdown.
-   Also, see if the logic can be extended to format the codeblock based upon the language used by the codeblock. And provide an option which can be provided in the backticks to disable formatting manually.
-   Look into Prettier markdown code, to see if various formatting options can be disabled manually.
-   Modify KushajveerSingh.github.io with relevant changes after finishing the project (remove prettier-ignore comments).

### Configurable Formatter

**Motivation** \
Unopinionated prettier alternative. In Markdown, prettier would currently format codeblocks and that would mess up JS code. And the way to avoid this is by adding prettier ignore comments, which is annoying.

Split the project into core module and language specific modules.

**Goal**

-   It should be drop in replacement.
-   Rewrite in C++.
-   Clear all [prettier tests](https://github.com/prettier/prettier/tree/main/tests/format) to ensure success.
-   All the 3rd party plugins/integrations should work automatically. For this you might have to force people to rename your package to "prettier" manually (or through a script, soft alias).

Other stuff

-   Make every formatting option of prettier available as a config option.
-   Prettier plugin to disable formatting of content inside codeblocks in markdown. Need to handle the case of using more than 3 backticks to specify a codeblock.
-   Additionally, code inside codeblocks should be formatted as per the provided language.
-   Disable formatting of content inside frontmatter i.e. inside `---`.
-   Publish on `npm`, `jsr`. Look for other alternatives.
-   Copy prettier repository/doc/npm structure.
-   By default, the new formatter should behave the same as prettier.
-   Provide a config file, which includes all the config options, that people can read top-to-bottom. This is one thing that I found annoying with current tools, as you have to check the docs for all possible options and sometimes they are not even documented properly.
-   C++23 is the target and the formatting should be parallelized.
-   Look at prettier alternatives like [Biome](https://biomejs.dev/) written in Rust and do speed comparisons.
-   For every possible formatting done by Prettier, expose a function letting people build their own logic on how to format that option. For example, if you have code that handle formatting ternaries, then expose a function that can override that behavior.
-   Use the Chrome JS parser to get the AST.
-   If AST is the way to fo formatting, then look into all the programming languages out there and integrate their AST into the project, and then provide an option to format each node. This would make the project a universal formatter.

## HTML

### Analyze DOM

-   There are chrome extensions that deal with analyzing DOM like number of nodes. Consolidate all the extensions into one script/extension.
-   Chrome console shows DOM after JD render. Ctrl-U shows HTML returned by the server. In the chrome DOM, spaces and newline nodes are not shows. In your project show these as well.
-   Add a graph showing the dom depth.

### Linter

-   In the spec, there are ton of recommendations, plus errors which browsers respect. And all this information can be converted to linting rules.
-   For each tag look into
    -   how many times they can appear in the DOM. Like `<title>` can only appear once and in the `<head>`.
    -   the possible attribute values, and no duplication of attributes.
-   Security best practices, should also be integrated. This includes how to get rid of all security attacks like cross-site, and more (as mentioned in the spec).
-   For SSR framework, get all the routes and then run the linter on the generated HTML. Since in SSR, all HTML is hidden in JS and linter would not be of much use.
-   Check HTML xmlns for XML, for stricter parsing.
-   `<p><p>Text</p></p>` - Linter can look into unnecessary tags as well. What is nested div's are used, but the code be replaced with just one div.
-   h1 tags should be in order, and there should be no skipping.
-   After reading best UI practices, see if these guidelines can be translated to a linter.
-   Ensure all the tags are closed. Every tag (should probably) have a certain number of valid tags that can be nested inside of it. This information might be helpful for guessing if the tag was closed or not. When using the HTML parser, you have to look into if the AST returned includes auto-closing nodes added by the parser, which is what we don't want. The reason why we don't want auto-closing tags, is it sometimes results in weird behavior.

    -   This website lists [most common errors](https://htmlparser.info/conformance-checkers/).

### Minifier

-   For each tag, use the spec to minify to the extreme.
-   Add an option to further compress the output using popular compression techniques and show the output size comparison using various all the comparison options.
-   Use DOM analyzer, to test the effectiveness of the minifier, in terms of `querySelector` performance, number of nodes, and other relevant stuff.
-   For removing optional closing tags, one situation that comes to my mind is people modifying HTML from JS. Using `.innerHTML` to access HTML, would produce HTML without closing tags in production, which would be different from development, since the closing tags would be enforced through the linter.
-   Run the browser parser to generate HTML for the minified HTML and the original HTML, and it should be the same.

### Formatter

Create HTML formatter module for Configurable formatter.

### Parser library

-   Create a utility library that exposes the HTML parser that you use in previous projects, and can work on both full-HTML string and streaming HTML.
-   For the streaming HTMl, build a visualization tool to show how the DOM looks over time. The streaming should be at 1 character increment levels. And then of course you can provide a batch size option.
-   Show where the parser got halted. Like script, link, style tags halt the HTML parser. How can this information be made available to the users.

## CSS

### Inline style sorter

-   Create specific file/directory structure for adding css/js features. - Like for CSS, allow custom functions that can overwrite the default for all pseudo classes. - Keyboard shortcuts in a different directory. - JS event listeners can be replaced with reactive programming. And these can be placed in a specific directory only. - Similarly for CSS you can different modes like print, accessible options. And these all would be in a separate file.
    Another idea for checking duplication of utility function

-   Create a list of all the utility functions in a project, and optionally a string that describes the intent of each utility function.
-   When a new developer creates a utility function, they can query this database, to see if a similar utility function exists. And this can avoid duplication.
-   This workflow can be triggered from GitHub comments as well (which is really helpful during a pull request).

Another idea for testing orthogonality

-   Check the commit messages with bug/feature and create reports on the number of files changed, which can can test for orthogonality.

### Language best practices

-   For every language, there are common guidelines, best practices, best design patterns.
-   This is an extension to Pragmatic Programmer project, for language specific stuff.

### Charts

-   When creating charts, use AI to analyze how people of different calibre would use and understand the info.
-   For the chart, show how info can be improved, what is redundant, and what extra info can be added to improve comprehension, and what supplementary/complementary charts can be used.

## Other

### Static Page encrypt/decrypt

**Motivation**\
When hosting documentation on CND (without server), it is not possible to hide internal pages from public. HTTP password, hiding routes exist

-   Encrypt HTML of the page during build process, and when user navigates to the URL, the encrypted HTML is shown.
-   User can provide a one-way password (decryption key). The program decrypts the HTML. One optimization you can do is, every decrypted HTML should contain doctype at the top, so if the decrypted html does not start with doctype, then say incorrect password.
-   This encryption strategy can be extended to page components as well.
-   Store the password in cache and revalidate after some time.

### Spellcheck program

-   When using VSCode [Code Spell Checker](https://marketplace.visualstudio.com/items?itemName=streetsidesoftware.code-spell-checker) extension, there are so many false positives.
-   Make your own spell checker, and provide utilities to add their own content.
-   For example, in case of HTML, the HTML spec contains all the valid words. So just add every word of HTML spec into the spell check dictionary, and this should include HTML specific words. You can look into creating separate dictionaries for each language.
-   An alternative, is to look into the grammar of each language and add all the keywords and stuff like that. Also, for each language there are rules like, the function name can be anything, so spell checker should not be enforced there.

### Project version and info tracker

**Motivation**\
Make it easier to track the dependency of packages used across multiple projects, thus helping with version upgrades. And the same reasoning for personal info, like the tagline I use across all social media.

Other stuff

-   For every dependency, For every dependency, show the location and version number that the dependency is used in, in tabular form. Current version, latest version, location. Set the background of the table cell based on how far the current version is from the latest version.
-   Create a C++ script that can automatically get the version number of all the dependencies used by a project. So if you have 10 projects, you provide it 10 urls/locations and 10 methods to get all the required version numbers. And then you run the script whenever you want to update the page. It also fetches the latest package version.
-   The information can be outputted in some standard format (read on which format to choose).
-   Also, add personal information tracking. Like you are using a tagline across multiple social media accounts and pages. So you list things like tagline, name and then list of all locations those things are used.
-   For MDN HTML docs, you have to check the modified files between the last and current commit.

### NBA Game App

-   How would a NBA game have played if there was a 4 point line, or the 3 point line was molded into a custom line, with all the related stats.
-   Create a leaderboard of who can make the losing team the winner the fastest by modifying the 3 point line.
-   Do the marketing and creating a viral hashtag like `myteamwon`.

## Company Ideas

### Basketball shooting machine

-   Set up basketball shooting machines in gym and create a website for people to book 1 hour sessions.
-   Split the charges/maintenance with the gym and yourself.
-   Invest money equivalent to the cost of the machine, before expansion.

### Ground marking machine- Create specific file/directory structure for adding css/js features.

    -   Like for CSS, allow custom functions that can overwrite the default for all pseudo classes.
    -   Keyboard shortcuts in a different directory.
    -   JS event listeners can be replaced with reactive programming. And these can be placed in a specific directory only.
    -   Similarly for CSS you can different modes like print, accessible options. And these all would be in a separate file.ormation about healthy lifestyle (exercise, recovery, nutrition, sleep, ...).

-   Completely free and proof-checked with experts. Donations are welcome but not expected.
-   Have a forum section, where people can post suggestion for new/missing topics.
-   How to take care of each body part (ear, nose, eyes, ...).
-   For supplementation, show how you can get raw ingredients and make this stuff at home, optimized for your blood work/goals.

Companion app

-   To track stuff in a single place. Like for sleep, there are various metrics that can be tracked (all provided by a watch), or simply user logs.
-   Make this app available on a cloud provider that people can setup themselves, so as to ensure all the sensitive data stays with them.
-   Users can enter the stuff they eat daily (through a barcode scan) and the portion used, and the app will display the micro-nutrients consumed daily.

### Pharmacy company

-   Focus on one health domain, like eyes. What are all the possible disorders, and the best medicine for each.
-   For some medicines, companies charge ridiculous amount of money (due to patents and copyright).
-   Screw the patents, and create the cheapest possible option.
-   Make the final price of the medicine completely transparent. If the medicine cost $10, then break it down to the smallest factor and show how you got $10 as the final price.

### Origami machine

-   It takes paper and diagram and does all the pre-creasing.
-   Diagrams composed of grids, and creases are folds in the grid.
-   The machine first makes the grid, and then square by square makes the creases.
-   Software that helps making the creases easier.
    Create HTML formatter module for Configurable formatter.

### Stapler

-   Stapler that folds paper instead of using physical stable.

## Job roles explores

-   GIS
-   Licensed land surveyor field
-   Emergency management
-   Project associate recovery specialist
-   Mineral rights for oil and gas companies
-   Check if company is meeting compliance for something

## (Template) - Template repo

-   Create a template repo on github that you will use for all future side projects.
-   Create a script that walks through all the setup, and can read the config from a JSON file for example. So you should be able to recreate the entire repo using the JSON file and the script.
-   Also, if people want to make changes to the existing repo, see if the script can be adjusted for that as well.
-   Use GoLang, HTMX, Tailwind as the tech stack.
-   Along side the interactive tutorial script, have detailed documentation as well.
-   If you just want documentation, then the repo can be used to generate that as well. If you just want to write HTML files, then it can be used as well.

## Fortran transpiler

-   Start from 66 standard and work your way up (look into various compiler options also)
-   [fortran-lang.org](https://fortran-lang.org/)
-   [fortranwiki.org](https://fortranwiki.org/fortran/show/HomePage)
-   Wiki has list of libraries using Fortran. Transpile them to C++, make test cases pass, test for performance to act as a proof of concept.
-   Use the official compilers for this task. A compiler has the following components - Lexical analysis - source c- Create specific file/directory structure for adding css/js features. - Like for CSS, allow custom functions that can overwrite the default for all pseudo classes. - Keyboard shortcuts in a different directory. - JS event listeners can be replaced with reactive programming. And these can be placed in a specific directory only. - Similarly for CSS you can different modes like print, accessible options. And these all would be in a separate file.your cpu (the most powerful one like m4). And then throttle the cpu, till you get the performance of a weaker cpu. Do this for all the cpus you target and same for gpus.
    Then in your testing you can throttle the same cpu, and get website fps scores for all the cpus you are targetting.
    Cpu/gpu/ram can be throttled at hardware and browser level

When making your css component library, have three levels of documentation.

-   usage docs, showing how to use all the variations of the component
-   intermediate docs, showing how the component addresses all the accesibility, performance, ui issues with the component
-   advanced docs, showing how the component is built internally

To track changes in document pages, use an automation tool to go to each page, and compare the info with the previous value.
You will need to manually, add links to all the pages.
Also, you can track sidebar elements, to ensure no new item is added.

Extension to hide sponsored from google search, ai and allthe other stuff and show legacy stuff first. Also, show random results from page 2, 3 of results.

Create a linux distro with windows 10 ui, and all the similar programs, settings ui, windows store.

Add kinetic sculptures to home

Cable television simulator, where you have episodes and commercials, and then the tv runs 24/7 on a set schedule and commercials are inserted in between episodes.

For data attributes in html, define a enum with all possible values somewhere and it can be used in html, css, js to verify if someone misspelled it, or see available options

Create a code snippet module in native css, js. Check all the documentation generators and copy their features.
More importantly, add compiler outputs, advanced stuff like assembly output, or whatever is relevant to that language.
Integrate this into all the popular documentation generators as a plugin.
See typescript, swift docs for inspiration.
Add optional feature for formatting the code using offical formatters of each language.
Comments/Code can be highlighted for warming, danger, info, ...
All the code in the codeblock should be generated at build time, including line numbers

Qwik ui library, with options to first define the design system and then the components.
Qwik in mistosis, to convert the code to all other frameworks and test using ui components.
Qwik seo, with all the required components like creating head, schema.org components, tools that help with seo.
Qwik owasp security helper.
Qwik pwa helper.
Qwik accessibility helper.

Create a visualization tool of parser for each library you use.
For html, given the html, show step by step the graph being built.
For browsers, show how this is translated to graphics layers and all the intermediary.

In qwik create, all the libraries to reach feature parity as other frameworks
Qwik- design-system build the design system based on the youtube video, create figma and css code, test cases, multi language(left to right, right to left, top to bottom), headless components, allow users to modufy the style while still using the existing edge cases and add new ones
Qwik-seo schema.org, other validators

In database, create logic to check how different is your query from sql standard, and provide suggestions on the differences, like this function does not exist, or this is how the function differs from sql standard.

Create application to track item by item costs of a company, liek salary then go deeper salary per employee, then deeper how salary is distributed for a si gle employee like tax and actual comp.
For a plane, break it down to how many screws are in the plane and what is the cost of each screw

For battleship game, ceeate a ui to input all your ships coordinates, and then compute the number of moves it will take to figure out all the ships using the optimal strategy of exploring in a checkerbox pattern, also add different exploration techniques.
Add option to add a second player, and find odds of winning.
Start checkbox pattern from each positin and go in both forward and backward directions to get average probabilities.
Also, add option to input custom size of the ships and board size and adjust the algorithm according to that.

When working on html, css tools create something similar to "tldr" on linux. When you hover over html element, it shows the semantic meaning and performance impact.

Create a ui for vimms lair, with all the emulators made till this day. Create text files with urls (steps that can be fed to playwright) to download all the roms. After downloading, find a way to compress as well, and the ui can show a list of decompressed files which can be played.
Run a test every month, to see if all the downloads are working.
The ui tool takes care of downloading, updating emulators.
It also shows where to see for help on the wiki before playing a game, to fix any bugs.
Support for all the co trollers should also be seamless.

Can jasn file be preprocessed to optimize it for later. Check kiki's bytes youtube video on how nvidia reduced json processing. Looke how chrome/google does it.
Create c++ parser to parse the file in parallel chunks like apache spark.
Create vilkan/gpu versions of the algorithm.
Can you minify the json variable names, and then in the end have the mappings if needed?
The mappings can be extended to have another object that specifies the type.

In the c++ template, how to compile a binary for each linux distribution.
This includes testing infrastructure, to run alll the source test cases on the compiled binary, and show the info in a table and github action.
This includes detailed automatically created documentation on how to install, upgrade, delete.
This includes creating a ui that will look in a folder (similar to macos) for the binaries. Create desktop icons and other stuff for each distribution. When user runs the ui, show option to check for update, delete, roll back to previous version.
In the tool, provide a syntax where the developers can define where and how to check if an update is released.

Look at all the compliance related stuff and create a detailed blog on how to achieve it.

In the css library, add animations like nocode tools, charts, diagram tools like plantuml, maps, and anything else you can find.

After creating css components, create a simple tool for qwik with drag and drop to mimic nocode tools, with the same options as the arguments of the components. Limit the customization to have a better mapping to code.
Create cms for people to easily modify the site.

For devtools, can you build your own. Like ladybird hooked into firefox devtools protocol to get all its benefits.
In this way we can show lot more information from the chromium likerender tree and so on.

Create ai to translate all the ui principles to your website

Old indian political cartoons,can be brought to american audience, like mojahed fudaleist in ufc.

Website with all the card games, and their optimal solutions.

Url best practices. Integrate with js framework and automatically check all urls.

Make a drone vr thing, where you can play liek star wars ships.

Sync clipboard for copy/paste between computer and phone. Useful when you want to paste something from computer to mobile.

Take popular movie series like resident evil and reorganize all the video clips in chronological order. Create a youtube channel for this.

Take books like goosebumps and make animates series for kids6

When crossing rivers, mountains, attractions you wonder what that is. And app that can just show the attraction around you as travelling can solve this problem.

As sleep time approches the lights need to get dimmer. All lights, monitors can be controlled through a single app.
Plus, in case you arrive jome late the algorithm can adapt and adjust sleep time or how fast the lights get dimmer.
Other mood things can be explored like getting ready for an event, meeting, waking up. Bryan johnson as reference.

How to unfragment linux. Universal directory structure, shortcuts, ...

On trail take pictures abd report tge location for trash pickupvand maintenance

Weite your own we standards abd integrate with chromium, and provide thevbuild for download. No html, css, js only rust

Convert linux kernel development email into online form thing, where all the updates from email are visible in a searchable form.
Maybe allow people to post from form to email chain.

In rust, create a special debug run, where we convert all the numbers to string and do the relevant arithematic, as the application is running, and check if there is ever an overflow or underflow.
Extend the program to include precison errors, like when adding two floats. In this case, tell the user to check for if numbers are close to each other, rather than strict equality.
Look at whitebox for more debugger ideas [link](https://whitebox.systems/)

C trnspiler to rust. Look at tractor by federal. Convert major c codebases like linux, git, gnome, ... as a proof of concept.
Write a transpiler for c++, go, ...
Start with low-level and even ancient languages and work your way up.

App to track skincare routine. Set timer for multiple things in a row. And track.

Using a series of mobile cameras, record live basketball game and get stats of all players

Ticketmaster has a ton of problems. Create a clone.

Phone app with option to block people similar to alarm clock. Block people in a time range, day range. When blocked, people's call would just keep on ri ging so that they don't know that they are blocked.

Build ui for all steps of compiler architecture

Create a book that shows how India can improve as a society. Make a constitution from scratch, and what current laws need to be removed.
Death penanlty for corrupt people.
Reduce population by taxing on second child.
Prison like El Salvador to get free labor.

All medical records are online, and there is a risk these might get altered without you knowing, or you might lose access to them.
Create a versioned version of these documents, plus any other document that is relevant like insurance, bank statement.
Help autoparse the document for searchability.
Add records of family.

There is a standard on how to create li rary books. Convert this into an interactive web page, and maybe work with an actual bookbinder and create 3d visuals of the process.

Learn magic. Matt mcgurk start here. Asi wind tricks next.

Learn road bike and miuntain bike maintennace. Zinn and the art of road/mountain bike maintenance books.

In project where you analyze code that can be paralleized automatically, do it for Js where you convert all other code to promises and do an await before it is first used.
Check the performance on popular libraries.
Create a simple UI to show what changes were made.
Combine this with a profiler where you get time taken by each line, or function. Then you can say the profiler has these lines with the worst performance, reference it with parallel and only optimize these.
Using the profiler we can also automatically optimize sections of the code.

Add all pc parts to bettertech project like motherboard, powersupply

## Company ideas

In games you can have servers and bots running to make monry. Watch asmongolfdtv $3,000,000 WoW mafia that blizzard is scared of.
Explore all these options an dhave your own setup making money.

Vanilla ice cream with chocholste outside. Have multiple layers vanilla, choco, vanilla choco

Clash royale clone with the original gameplay and cards. Have a random mechanism where cards get buffed, nerfed every two weeks

Competitor to hello fresh but instead connect chefs to customers like uber and let the chefs handle groceries delivery.
Provide nutritional content of all meals to users, vet the cooks

Drone show for weddings, events
Micro robots in fish tanks for another variation

Make framework for character making in video games and similar stuff like having console, pc input mappings in games

Solution to help with scalping products. This can be sold to companies/retialers, and at launch time the solution would manually verify the customer before approving the sale.

Create 3d models of the home for sales, adding another option for buyers to help make their decision.

App to purchase traditional infian dresses, and expand it to other countries

Leaves are used to teplace plastic plates. What if we were to form a sheet using the leaves, that people can use to cover the plastic containers in their home.
The selling idea would be to help reduce intake of microplastics from the containers

Vr to teach biology. Vr is used in car manufacture industry, when making 3d designs. The 3d designes can be extended to human/other animal.
How different diseases problems propagate or evolve in human body. Like how dry eyes evolve, smoking affects the lungs.

Drone to 3d map water cave. Put sensors to get various laters of data from wlthe walls.
Use AI to figure out what might be under the walls.

Payment processor like stripe but with fixed cost per transaction.
Start with usa, and then keep adding countires. Can have different payment page for each country.

Modify anti-cheat to handle all the hacks. Sell it for like 10 cents per user base.

Create unpickable electronic lock

Create a tool like your own version of storybook for for desktop apps built with qt. It will show screenshots of how the app looks in different linux distros.
And also take care of all inconsistencies.

For every ai tool they add metadata which can be checked if the inage/video by created by it.
Make a tool to automatically check all this stuff. Google released SynthID detector to check for their products.
Make it an app, desktop where users can check validity of a video, audio, image on their screen with the click of a button.

App similar to nutrition insights of kroger. Include all the nutrient breakdown. I tegrate with shopping sites and get price of food per day or user can enter manyally. I tegrate all the health related stuff.
Merge robert idea of showing deals on food prices from different stores.

Delete tool for all os's that actually clears out the bits on disk. And gives an option to fill with random bits.
Nvme drives have a default password, since data is encrypted on hardware level. Look at this level of detail when creating the software.
For each hardware, what is the best way to deal with the erasing process.
Check if encryption is enabled, so that user does not have to worry about it.
Mental outlaw video: How to securely erase data on May 27, 2025

Make a data recovery tool.

Password geenrator app. "How strong should your password by by mental outlaw"
In the password tool, have it where it automatically helps you resets your password on the websites you choose.
Maybe a monthly popup where is opens the necessary wi dows back and forth, and you copy paste the passwords with ease.

In apps liek garmin for tracking runs, bikes, hikes, sometimes you forget to turn off after finishing the activity. Create extensions to these apps for modifying the activities, and quickly set the stopping point to where you actually stopped.

Cybersecurity. Take wifi and for all versions create an easy to run exploit collection.
Do for bluetooth.
Find a cve list and provide 1 click solutions for all severe (or the highest ranked vunerabilities)

Mental outlaw video "Make any messaging service E2E Encrypted with PGP". Create a client for this.

NBA reffing is messed up. Create data visualizations to see how refs, odds, vegas affect the outcome of the games. https://www.youtube.com/watch?v=wmFCwweYy_E
Create a graph of how refrees might be connected outside (like a social engineering graph, who refs follow on social and stuff like that)

Use incline, decline elevation and length of trail to score the hardness of a trail

In js linter, create a tree showing event delegation. So you can look at the tree and show how much can you save by using event delegation.

create rust standard library, with all the data structures. and optimize them as much as possible and have graphs showing improvements. Also, document the data structure in detail with all the optimizations. Also, look into optimizing the data structures with cpu specific stuff.

Problem: with accessibility with math equations (can maybe can extend to chemistry)

Create an api similar to stripe and webpage similar to stripe/square. Have all the payment options that can be added, without requiring to be a payment processor like stripe. For example, stripe takes a cut of google pay, but if you set it by yourself, you don't have to pay that cut.
Add invoicing options to it next. Look at popular invoicing (invoices created) by enterprise erp software.
Add taxes (including international) as much as you can.
Charge 50 bucks for 6 months for payment code, and additional money for invoice, tax.
Create dashboard with graphs to shoiw demographics/money and other stuff.

Combine all enterprise software solutions into 1 like erp, sas, sap, github, ci/cd, issue tracking.
Start with a central account management system similar to microsft. Now you have accounts and organizations.
Add github clone to it, without the bloat and also the privacy issues.
Add issue tracker and link github to refer to this.
Add ci/cd.
And keep adding other enterprise solutions.
Host it locally, cloud.
Add scripts to help companies export data from other services to this.

In youtube video dark reader project, add an option to blur/black a section of the video. Sometimes you are watching a video with chat in one corner that you want to hide, to avoid spoilers.

Biochemistry, bioinformatics, gene related fields. All conferences, software

Build a UI for all the steps of the compiler architecture

in css project, meant to replace storybook, also show how figma rendering. The design would havbe to manually map the components from figma to the tool, but this tool can act as an intemediate between designers and developers.
get clips of wnba players shooting ball and do a question on the outcome (miss, fail, foul, fight). Mkae it like a game, every score gets +1 dollar and incorrect gets -1$. The target money is the debt wnba is in. The program's entire goal is to make wnba profitable for first time.

https://optimeist.com/ add to aws ui project

simulation testing for websites. provide an array of all actions that a user can take, and the simulator will run those randomly. this includes, everything like you are on a page, the user has the options to close the tab, open new tab, change url of current tab and lot more, and also click the buy button on the page.

struct in c# have memory alignment issues. create tool to get size of all sturcts in your code, and give suggestions using data-oriented design on how to optimize if for your programming language of choice.

js performance. create an Emmet like extension for vscode that takes the syntax `p_{name}_{args}`. For example to get code for optimized for loop it will be `p_for_i_arrName`. Go through all JS features, look at how each feature is implemented at assembly or some lower level to get idea of max performance. And create shorthands for that. You can also, look at creating a transpiler to convert js into performance js. In vscode create snippets starting with `k_`. And make it for every js feature like loop, reduce, ...

Create tool similar to sessionbox, multilogin chrome extension. When opening outlook, you can only have one account open in the entire chrome window across multiple tabs. Create an extension that allows you to limit a website cookies, cache to a single tab. Also, provide full control over the user-agent, in the sense you can modify the user-agent per tab.

In access control, with a lot of booleans for read/write permissions, you can put all the booleans into a single byte and have code abstracted that deals with the underlying. This is part of the auth project that repicates better-auth. Build a UI for the access control as well, like drupal.

extension to npm, where it automatically choses the next avaialble port, in case the provided port is busy.
Think how you can extend this to any project, since knowing if a port is open or not easy, and then you create an env variable to run the script with that port, assuming the script can take port from env variable.

People are reluctant to read texts that use unfamiliar words. Have a feature when reading online where the meaning of words from dict or from llm would be useful. Definition of a word from dict is not that useful, as we need to know what the meaning of the word in the current context is. Reference the reading mind book on what other things can be added, like sound of word.
Create a tool that can be used by teachers to help improve reading in schools and by parents.
Add all the grammer stuff liek subject, noun, verb, adjective, adverb. In each sentence we can classify the words into what grammer they belong to.

Track health insurance benefits. For example, my insurance has 1 free physical and 1 free blood test. Have an app which shows stuff like this and helps you set reminders to use all the benefits.

Rust web development
Instead of creating tools for vanilla js, html, css. Create rust equivalent tools. Create a framework that converts rust into raw html. This can use limited html syntax. Add seo, image optimization, and all your other plans.
In the end create a website for the new framework, where you compare writing in rust vs old stack. This includes, better html support and various features that are supported like seo, accessbility. For js, not needed in new as we using rust. For DOM, here is the library to use. For broser apis, here is the api to use.

China has a drone show. Use drones to create a wedding story.

calorie tracker. plus exercise tracker. Use the exercise info to somehow infer the calorie requirements for the day.

In rust htmlrewrite, add a templating engine.

create a payments page template using info from https://mitchellh.com/writing/advice-for-tech-nonprofits

cancer related software. something that can take all the research and make it available to the people (by extension the hospitals). If people have the instrument, they can run it themselves (thinking in this direction).

Problem with math books, they are too abstract. Textbooks pull abstractions out of thin air. Yet mathematicians don't randomly set up axioms and start working out theorems. Every axiomatic system had a history. Had a motivation. Worked through loads of examples and counterexamples to motivate and finally settle on a specific definition that's both general enough to capture a sufficiently wide variety of interesting problems, but also specific enough so that we can actually prove interesting results about them. https://www.youtube.com/watch?v=D0ZVe9AtgUg Create your own math book for subfields of math you are interested in learning, and do motivation, history, examples, use cases, arguments against and then do the usual stuff that current books do.

Windows has a central account management on domain level, active directory. Linux probably has this in paid ubuntu, red hat. Create an easy to use setup for free. Manage linux fistro from central server.

On linked add software engineer 2. Used rust to migrate js to rust, transpiler to rust, internal report generator to rust.
Excel files to web apps, outlook to an open source emeail server. Microost auth to linux auth, and stuff like that chroot.

Have stuff in cart a week before black friday on amazon to order and get prime credit card

Make a detailed note on how internet works i.e. how do you make request for a webpage and get it served. Check full stack for front end engineers v3 video on how the i ternet works on fro tend masters for reference.

Use vpn to check your site performance across counties. Part of seo project.

Simulate small world problem https://www.youtube.com/watch?v=CYlon2tvywA. Get population of each country, and further every city and zip code. Now you know exactly how poeple are distributed across the world. Next add probabilities of interaction based on geography and reduce as you go back further. Make these numbers available as controls.
Have additional controls like railway connections, airport connections, visa free entry, gpd.

Create an automation tool similar to zapier, https://n8n.io/. Look at all the tools used by companies like gmail, ... and create a comprehensive automation using the official apisl.

In the documentation theme, create one based on https://fumadocs.dev/

personal website. take inspiration from

-   https://vale.rocks/posts/the-design-of-this-site#footnote-ref-1
-   https://vale.rocks/posts/the-implementation-of-this-site
-   https://vale.rocks/posts/writing-style-and-mannerisms
-   https://singhkays.com/blog/netflix-av1-decode/ - Use of > and >> in headers

live airflight tracker. map the flights in an app.
include future flights from the database and then choose the locations on map and show only the flights from/to the location.
add trips like from to location pairs and show them on the app.
extend this to railway, bus, taxi, tourist spots and adventure spots (look at adventure.md for things you can add) (anything that has public data)
include weather app in this as well.
This app becomes one stop shop for everything travel, weather, adventure. Include your adventure.
Include all the local emergency numbers and locations for hospitals, and where to go for different kind of emergencies like eyes, accident, urgent care.

make a program for all of nvidia ecosystem

obfuscation and hardening, used to prevent information leak from source code to machine code to make the compiled code extremely difficult to reverse engineer and understand.
