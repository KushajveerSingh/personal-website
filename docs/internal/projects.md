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

### OpenTicket

-   Have a ticketing system similar to Ticketmaster.
-   Only people with OpenPay/OpenBank accounts, will be able to use the service.

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

### PWA

-   Create all things needed for PWA.
-   Checker which shows the rendering of all the options.

### HTML Head Generator

-   To generate a template for all the tags that you can possibly enter in HTML head.
-   This can be `<meta>` tags or HTTP relevant or a function that returns the HTML.

## VPN Performance Check

-   Use vpn to check your site performance across counties.
-   Can use aws vps as well.

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

### Fortran

-   Start from 66 standard and work your way up (look into various compiler options also)
-   [fortran-lang.org](https://fortran-lang.org/)
-   [fortranwiki.org](https://fortranwiki.org/fortran/show/HomePage)
-   Wiki has list of libraries using Fortran. Transpile them to C++, make test cases pass, test for performance to act as a proof of concept.
-   Use the official compilers for this task.

## Database

-   Go through postgres documentation and create an admin panel to configure all the settings, this includes options for compiling postgres from source.
    -   For options which can take multiple values like page size, offer a testing utility which users can use to get the best value.
    -   Provide guide on meaning of all values and how to choose the appropriate one.
    -   [link](https://postgresqlco.nf/) Page listing configuration options and tuning guide.
-   Go through all the Postgre Weekly newsletters and add stuff from there.
-   Look at databases built on top of Postgres and merge their features.
-   Look at databases outside of Postgres and merge their features.
-   CLI to create josn schema of all tables, typescript format, format for other languages.
-   Some queries integrate data from previous years, and the output rarely changes. The output from previous years can be cached. Create interface where you keep track of such queries and update the cache. If intergation can be made on when was the table last modified, the update of query output can be automated.
-   Development database setup and easier way to run queries in dev mode before pushing to production. In dev mode, add methods to easily add worst case inputs for the queries. And also test for performance.
-   For migrations provide two functions for each function. How to upgrade and how to downgrade. And then it is to the user to handle the logic. The software handles the execution of these functions and can be used to roll back 100 migrations if the logic allows it.
    -   During migration we might have to look into load balancing and how to migrate when the database is really huge.
-   High availability
    -   patroni
-   Add backup testing, backup restore. Disaster recovery.

### Access

-   Create transpiler and tooling to migrate from Access database system.

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

## UI Library

-   Create a UI library for Qwik js (or framework agnostic) with inspiration from shadcn.
-   Add design design from UI collective. Split it into files, with the base where you define the colors, and other values being in a separate file, that can be exported and imported easily.
-   Look up all the UI libraries out there and add their components.
-   For each component, have a separate file/layer for print, accessibilty options, keyboard shortcuts, event listeners.
-   Add option to add changes to inputs, text fields, dropdowns to the URL. This is relevant to use URL as state management. Check [/product-design/seo/url.html#url-as-state-management](/product-design/seo/url.html#url-as-state-management).
-   Add performance optimization for things like toggle. Even though toggle is a boolean, it takes 1 byte in memory. Multiple toggles can be packed together using bitwise operators. [video](https://www.youtube.com/watch?v=z7wVUfnm7M0).
-   Components
    -   Header - Similar to amazon mobile app, the header background fades if you have not scrolled for some time, and then the text fades as well.

### Markdown Editor

-   Markdown -> HTML -> PDF
-   Show live preview of PDF export.

### CSS Style Sorter

-   Sort CSS style properties similar to how Tailwind sort's the class names.
-   Add sorting logic for both inline, and stylesheet.

### Media Optimizer

-   Image/audio/video/svg best practices in HTML.
-   For images, generate responsive img and picture tags.
-   Look at sharp, Nextjs img, cloudfare cloudinary.

### Code Snippet

-   Create a code snippet module. Check all the documentation generators and copy their features.
-   Add compiler outputs, print output, advanced stuff like assembly output, or whatever is relevant to that language.
-   See typescript, swift docs for inspiration.
-   Add optional feature for formatting the code using offical formatters of each language.
-   Comments/Code can be highlighted for warming, danger, info, ...
-   All the code in the codeblock should be generated at build time, including line numbers.

### Code Transition

-   Code transition animation as seen in youtube videos.
-   Component to show code and syntax highlighting like Shiki. Look at astro for all the things it can do.

### Storybook

-   Render the component at different resolutions, different browsers.
-   It integrates with the design system, and provides a page to adjust the design system and see the new rendering of the components. From the page, the design system can be exported to be used in your library (or just saved directly to the file).
-   Integrate testing to show the components for all test cases.
-   Add documentation on how to use the component.
-   Take screenshots of the components from all browsers. Look into using OBS (or native Linux APIs to capture screenshot).

### Accessibility

-   In the storybook app, add diffeerent accessibility renderings as well.
-   Integrate everything related to accessibility in the app. Things like stark app.

### 2D-Visualization

-   Charts.
    -   When creating charts, look into how people of different calibre would use and understand the info.
    -   For the chart, show how info can be improved, what is redundant, and what extra info can be added to improve comprehension, and what supplementary/complementary charts can be used.
-   Graphs.
-   Diagrams. Refer [Diagrams project](#diagram) as well, as that will build on top of this.
-   Some diagrams where it is of video format, to show flow of data (make it interactive, have the ability to hover and see tooltip). This might be int he realm of 3Blue1Brown animation library.
-   Icepanel, Lucidchart.

### 3D-Visualization

### Maps

### Math

-   Look at 3Blue1Brown animation library.

### Network/Distributed/Database

-   Create visualization like tigerbeetle simulator, for all distributed algorithms and database.

### Documentation Theme

-   Combine features from all documentation themes (fumadocs, ...) into a single theme.
-   Implement remark and rehype plugins.
-   Have internal pages viewed through authentication as well.
-   Live code snippets that sync with codebase.
-   Add Content Management System.
-   Support private/internal pages without server/authentication.
    -   Encrypt HTML of the page during the build process, and when user navigates to the URL, the encrypted HTML is shown.
    -   User provides one-way password/decryption key, and the webpage decrypts the HTML.

### NoCode Tool

-   Create NoCode tool with drag and drop.

### Website Templates

-   Add website templates like shopping, documentation, cms, forum site, ticketmaster, erp, sas, sap, github, issue tracking, ...
-   Payments page template using info from https://mitchellh.com/writing/advice-for-tech-nonprofits.
-   Check youtubes for project ideas
    -   Josh tried coding
    -   Code with antonio
    -   Javascript mastery
-   Check popular saas services and homelab solutions.

### Math Equations

### Chemistry

### Random

-   Look for niche topics, like electronic circuits.

## Code To UML Generator

-   Create program that reads file and removes all code between start and end comment block.
-   Use the code in between to generate UML diagram, and generate image from that.
-   This can be used to visualize if/else other loop logic regardless of programming langauge, as we are still relying on the user to define the diagram and doing any programming language code parsing.
-   The user will have to define an identifier which links the current code to some global object (identifying the function for which we are creating the diagram) and then define the logic for if/else and other things.
-   Also, try generating an automatic diagram for the entire code. Start with main function and then all the functions that it calls. Show sequential and parallel function calls.

### Parallel Code Finder

-   Find parts of functions that can be parallelized or made async. Do it automatically and compare the performance differences.
-   Create a simple UI to show what changes were made.
-   Combine this with a profiler where you get time taken by each line, or function. Then you can say the profiler has these lines with the worst performance, reference it with parallel and only optimize these.

## Wayland/X11

-   Create a Wayland/X11 alternative.
-   Some ultrawide monitors splitting single monitor into 2. Can this be implemented at software level, and let users define custom regions on their monitors, to act as sub monitors similar to ultrawide.
-   Dell monitors have Dell Display Manager, which uses DDC/CI protocol to change monitor settings. When creating the alternative, also create a GUI thing, and in the GUI add features from the Dell Display Manager as well (change brightness, color mode, ...).
-   Global shortcut for window movement to left, right, top, bottom. Use Win + UpArrow to move window up. Press up arrow twice to expand the window to entire top 50% width. Then downArrow would move it to smaller top region. For multi monitors, do Win + 2 + UpArrow, where 2 is for the second monitor. This will move the window to the number 2 monitor. Also, provide option to define these regions like top-left, top-right, .... And then use Win + arrow keys to move around.
-   Use CSS names for all the things. Like for borders, provide option to control border for each window separately. On the programming side, this can be achieved by using a unique dictionary key corresponding to the window and then defining the boder options, including the monitor to show on and the placement within the monitor as well.
    -   Include all the unique features from tiling window managers as well.
-   Programming interface allows any application to open a new window, with all the config options. As long as the dictionary key name is unique, applications can control exactly how to display their window (this does include things like excluding the top bar with minimize, close buttons, and changing position to top-bar, buttons).
-   Explore the idea of unfragmenting Linux, like having universal settings menu.

## Basketball Stats

-   For every stat in basketball, find its meaning and how to actually use it (without stat padding).
    -   For example, number of rebounds is a false metric. Because a reboud can be contested and uncontested, your teammate might box out making it easier for you to rebound. A better look at this metric is what is the rebound number of the team with/without you.
    -   For example, number of blocks is a false metric. A better metric is opponent at-rim field goal percentage (on-off). The reason being having blocks does not give complete picture of defense. You may still be weak and get pushed around and in general cannot keep up with defensive assignment. Also, use opponent at-rim shot frequency (on-off).
    -   [Video](https://www.youtube.com/watch?v=nhLIh4XQQjo) for reference.
-   Do the above for every stat.
-   Make it easy to make charts from these, like you see on youtube videos. Let people choose a player and time period and add it to the graph.
-   Look at popular youtube basketball videos and add the graphs they use in the program as well.

## Basketball Commentator

-   Whil playing 1on1 or 5's capture the footage and make up commentary mimiching NBA or 2k game.
-   Have commentary from different personas.

## NBA Game App

-   How would a NBA game have played if there was a 4 point line, or the 3 point line was molded into a custom line, with all the related stats.
-   Create a leaderboard of who can make the losing team the winner the fastest by modifying the 3 point line.

## Basketball Game Stats

-   Use a series of mobile cameras, record live basketball game and get stats of all players.

## WNBA Shot Predictor

-   Get clips of WNBA players shooting ball and do a question on the outcome (miss, fail, foul, fight).
-   Make it like a game, every score gets +1 dollar and incorrect gets -1$.
-   The target money is the debt wnba is in.
-   The program ends when WNBA becomes profitable.

## Linux GUI

-   Create Graphical UI to better view and navigate man pages.
-   Add all the special linux related files and settings here.
-   All the special command line commands that give info about the os and linux, have their output already present here.

## Email HTML Creator

-   Create tool to make HTML email campaigns.
-   Plain HTML/CSS (using Vite might work better).
    -   Have a template folder, and have something similar to JINJA syntax.
    -   Another folder for emails, which will contain HTML files. Folder structure can used for versioning.
    -   Script that creates `out` folder containing the final HTML files, with the templates filled in.
-   Reference [HTML Email Development](https://frontendmasters.com/courses/html-email-v2) course.
    -   Add tooling for linting, formatting based on the course.
    -   If stuff can be added from [Email on Acid](https://www.emailonacid.com/) and [Litmus](https://www.litmus.com/) add that as well.
        -   Leave the final checking of the email against email clients to these tool though.
-   Tooling to check the email server. Like what protocol is being used, is the email encrypted, ...
-   Add design system based on UI collective and simpler component library.
-   [mjml](https://mjml.io/) framework for making responsive emails.
-   Create GUI similar to nocode tools, for creating HTML email. Example [postcard](https://designmodo.com/postcards/).
-   Copy SAAS providers like Litmus.
-   Other tools
    -   [link](https://www.litmus.com/) litmus
    -   [link](https://www.emailonacid.com/) Email on Acid
    -   [link](https://designmodo.com/postcards/) Postcards
    -   [link](https://www.campaignmonitor.com/) Campaign Monitor

## Rust Dynamic Library

-   When installing library through Cargo, all the dependencies have to recompiled. Since rust has no stable ABI, there are differences in the compiled binary across different compiler versions.
-   Create a wrapper around cargo installer, which will install/get libraries from a shared folder.
-   When the compiler is updated, recompile all the libraries as well.
-   Features from security area can be added, like creating dependency graph, checking for outdated dependencies.

## Exercise Research

-   Write a paper on how doing multiple sports can prevent injuries.
-   Goal: If you are a kid use this protocol, and this might help you with reaching professional league, but this will defintiely help you stay healthier and injury free in your 60s.
-   Look at different sports, and parts they normally use, and make a series of sports that cover all parts of the body, without overusing the same part over and over again.
-   Look at mentality, physicality aspect of the sports as well.
-   Give suggestions like playing with old people, young people, left/right hand, playing with woman if you are a man. This covers playing the same sport slowly (against old people), and faster (against younger people).

## Laboratory Inventory Management System (LIMS)

-   Lims system using quickjs.

## Agriculture SAAS

-   What are the different types of things where agriculture related stuff is relevant (like water, soil, ...).
-   Now dig deeper into each type.
-   For water, where are all the places it is used (tap water, water filtration system, desalination systems, agriculture, acquarium, ships, ...).
-   For each use case, build a simpler 3d printed model, to understand how these systems are constructed. Like how a tap water is built from the tap, to the pipes that carry the water. These will help identify issues that are impossible to think about just mentally.
-   For each use case, identify the analysis or test that is relevant.
-   In addition to the 3d model, make an animation that can be used to teach people how the system they are using is constructed.
-   Consolidate all agriculture related software companies, precision agriculture, research papers.
-   Show how much people are saving by using this project. For every usecase, anlysis show the price from commercial companies.

## Bike Car Cover

-   When carrying bike behind a car, make a cover for it.
-   The cover can expand and contract using velcros to support 1 to 4 bikes.
-   The cover can shrink to cover smaller size bikes as well.
-   Add a bottom cover, so that water does not seep in.
-   Add reflective light things on the cover for night time riding.
-   Pitch: You need a single cover for a car. The number of bikes don't matter.

## UI Layout

-   Build UI layout engine with all CSS properties. Reference [Clay](https://www.youtube.com/watch?v=DYWTw19_8r4).
-   Add HTML like frontend with full HTML parsing and streaming (as browsers do it).
-   Look at the applications where Clay is used and modify the applications to use your HTML like syntax.

## Militiary Industry Complex

-   Show the people who joined private positions and what they held before. And how these people are related to each other.
-   For the grants, see which people approved.
-   In a table show all the people in militiary industry complez, have a button to show that persons last positions, relationship to others in the table, grants accepted.
-   Show how much money is spent lobbying by each company and at which locations.

## Cloud Helper

-   For every AWS service create GUI/API to configure all settings.
-   Check your current AWS implementation of that service for any anti-patterns and improvements.
-   In general provide a better documentation experience.
-   Do it for other clouds and then create utility methods allowing you to switch cloud service with a button click.
-   Look at [optimesit](https://optimeist.com/).

## Project Dependency Tracker

-   For all your personal projects (external projects as well), create a UI that shows all the dependency versions being used.
-   Show how the current version compares to the latest.
-   Show link to the release notes, and a script that can automatically submit pull request for updating the dependency.

### Personal Info

-   In addition to tracking project version, track smaller bits of info that can change over time like name, address.
    -   Some text that you put in the README.
    -   Some link that you shared. Places where you shared the link.
-   Include locations of all your social media, or places where you have created an account, provided payment info, provided address info. And a link to modify/delete the relevant info.

## Configurable Formatter

-   Unopinionated prettier alternative. In Markdown, prettier would currently format codeblocks and that would mess up JS code. And the way to avoid this is by adding prettier ignore comments, which is annoying.
-   Split the project into core module and language specific modules.
-   Clear all [prettier tests](https://github.com/prettier/prettier/tree/main/tests/format) to ensure success.
-   All the 3rd party plugins/integrations should work automatically. For this you might have to force people to rename your package to "prettier" manually (or through a script, soft alias).
-   Make every formatting option of prettier available as a config option.
-   Additionally, code inside codeblocks should be formatted as per the provided language.
-   Provide a config file, which includes all the config options, that people can read top-to-bottom. This is one thing that I found annoying with current tools, as you have to check the docs for all possible options and sometimes they are not even documented properly.
-   Look at prettier alternatives like [Biome](https://biomejs.dev/) written in Rust and do speed comparisons and OxcFormat.
-   For every possible formatting done by Prettier, expose a function letting people build their own logic on how to format that option. For example, if you have code that handle formatting ternaries, then expose a function that can override that behavior.
-   Look into all the programming languages out there and integrate their AST into the project, and then provide an option to format each node. This would make the project a universal formatter.

## HTML DOM Analyzer

-   There are chrome extensions that deal with analyzing DOM like number of nodes. Consolidate all the extensions into one script/extension.
-   Chrome console shows DOM after JS render. Ctrl-U shows HTML returned by the server. In the chrome DOM, spaces and newline nodes are not shows. In your project show these as well.
-   Add a graph showing the dom depth.
-   Given the html, show step by step the graph being built.
-   For browsers, show how this is translated to graphics layers and all the intermediary.

### Parser library

-   Create a utility library that exposes the HTML parser, and can work on both full-HTML string and streaming HTML.
-   For the streaming HTMl, build a visualization tool to show how the DOM looks over time.
-   Show where the parser got halted. Like script, link, style tags halt the HTML parser. How can this information be made available to the users.

## UI Helper

-   Take all the advise from UI courses and books and convert it into automatic checker.
-   The checker can maybe take the screenshot of the component as input and then run all the checks. Or it can take the source code and work from there.

## HTML Linter

-   In the spec, there are ton of recommendations, plus errors which browsers respect. And all this information can be converted to linting rules.
-   For each tag look into
    -   how many times they can appear in the DOM. Like `<title>` can only appear once and in the `<head>`.
    -   the possible attribute values, and no duplication of attributes.
-   Security best practices, should also be integrated. This includes how to get rid of all security attacks like cross-site, and more (as mentioned in the spec).
-   For SSR framework, get all the routes and then run the linter on the generated HTML. Since in SSR, all HTML is hidden in JS and linter would not be of much use.
-   `<p><p>Text</p></p>` - Linter can look into unnecessary tags as well. What is nested div's are used, but the code be replaced with just one div.
-   h1 tags should be in order, and there should be no skipping.
-   After reading best UI practices, see if these guidelines can be translated to a linter.
-   Ensure all the tags are closed. Every tag (should probably) have a certain number of valid tags that can be nested inside of it. This information might be helpful for guessing if the tag was closed or not. When using the HTML parser, you have to look into if the AST returned includes auto-closing nodes added by the parser, which is what we don't want. The reason why we don't want auto-closing tags, is it sometimes results in weird behavior.
    -   This website lists [most common errors](https://htmlparser.info/conformance-checkers/).

## Eye Care

-   Program to turn off the monitor for 5 minutes after 20 mins.
-   Intergrate other eye care program features and best practices.
-   Exercises that can help improve eye health.

### Sleep Care

-   As sleep time approches the lights need to get dimmer. All lights, monitors can be controlled through a single app.
-   Plus, in case you arrive home late the algorithm can adapt and adjust sleep time or how fast the lights get dimmer.
-   1Other mood things can be explored like getting ready for an event, meeting, waking up.

### Audio Care

-   Similar to above but for hearing health.
-   Look at various health conditions around hearing and how we can help train to prevent those.

### Brain Care

-   Similar to above but for brain health.
-   How can we train to counter conditions that lead to loss of memory, and other brain problems as you get older.

### Skin Care

-   App to track skincare routine. Set timer for multiple things in a row. And track.

## Spellcheck Program

-   Create spell check program.
-   Look at VSCode [Code Spell Checker](https://marketplace.visualstudio.com/items?itemName=streetsidesoftware.code-spell-checker) extension.
-   For example, in case of HTML, the HTML spec contains all the valid words. So just add every word of HTML spec into the spell check dictionary, and this should include HTML specific words. You can look into creating separate dictionaries for each language.
-   An alternative, is to look into the grammar of each language and add all the keywords and stuff like that. Also, for each language there are rules like, the function name can be anything, so spell checker should not be enforced there.

## Basketball Shooting Machine

-   Set up basketball shooting machines in gym and create a website for people to book 1 hour sessions.
-   Split the charges/maintenance with the gym and yourself.
-   Invest money equivalent to the cost of the machine, before expansion.

## Ground Marking Machine

-   Purchase robot that can mark the fields.

## Origami Machine

-   It takes paper and diagram and does all the pre-creasing.
-   Diagrams composed of grids, and creases are folds in the grid.
-   The machine first makes the grid, and then square by square makes the creases.
-   Software that helps making the creases easier.

### 3D Origami Machine

-   Machine that can make 3D origami pieces.

## Niche Jobs

-   GIS
-   Licensed land surveyor field
-   Emergency management
-   Project associate recovery specialist
-   Mineral rights for oil and gas companies
-   Check if company is meeting compliance for something
-   Dog walker
-   Photographer
-   Wedding related niche like butterfly farmer

## Compliance

-   Look at all the compliance related stuff and create a detailed blog on how to achieve it.

## Health Website

-   Information about healthy lifestyle (exercise, recovery, nutrition, sleep, ...).
-   Completely free and proof-checked with experts.
-   Have a forum section, where people can post suggestion for new/missing topics.
-   How to take care of each body part (ear, nose, eyes, ...).
-   For supplementation, show how you can get raw ingredients and make this stuff at home, optimized for your blood work/goals.
-   Document all disorders for every body part. Diagnosis and treatment options.

### Companion app

-   To track stuff in a single place. Like for sleep, there are various metrics that can be tracked (all provided by a watch), or simply user logs.
-   Users can enter the stuff they eat daily (through a barcode scan) and the portion used, and the app will display the micro-nutrients consumed daily.
-   Integrate with popular watches that track sleep, and other metrics.

## Website FPS Predictor

-   Benchmark the site using your cpu (the most powerful one like m4).
-   And then throttle the cpu, till you get the performance of a weaker cpu. Do this for all the cpus you target and same for gpus.
-   Then in your testing you can throttle the same cpu, and get website fps scores for all the cpus you are targetting.
-   Cpu/gpu/ram can be throttled at hardware and browser level
-   Mkae this part of the build system, and it can used as part of performance.

## Website Change Tracker

-   To track changes in document pages, use an automation tool to go to each page, and compare the info with the previous value.
-   You will need to manually add links to all the pages.
-   Also, track sidebar elements, to ensure no new item is added.

## Cable Television Simulator

-   Where you have episodes and commercials, and then the tv runs 24/7 on a set schedule and commercials are inserted in between episodes.

## OWASP Security Checker

-   Tool that checks all guidelines defined in OWASP.
-   Do it for all relevant OWASP guides like API, Web, ...

## Board/Card Game Optimizer

-   For all board games create UI to find the optimal strategy.

### Battleship

-   Create UI to input all your ships coordinates, and then compute the number of moves it will take to figure out all the ships using the optimal strategy of exploring in a checkerbox pattern, also add different exploration techniques.
-   Add option to add a second player, and find odds of winning.
-   Start checkbox pattern from each positin and go in both forward and backward directions to get average probabilities.
-   Also, add option to input custom size of the ships and board size and adjust the algorithm according to that.

## Emulator Game UI

-   Create a ui for vimms lair, with all the emulators made till this day. Create text files with urls (steps that can be fed to playwright) to download all the roms. After downloading, find a way to compress as well, and the ui can show a list of decompressed files which can be played.
-   Run a test every month, to see if all the downloads are working.
-   The ui tool takes care of downloading, updating emulators.
-   It also shows where to see for help on the wiki before playing a game, to fix any bugs.
-   All controllers should work without extra work.
-   [link](https://www.reddit.com/r/opendirectories/comments/6haqbq/every_ps2_game_iso_ever/) r/opendirectories every ps2 game (iso) ever

### Local Cloud Gaming

-   Run cloud gaming from your home server or VPS.

## DevTools

-   Fork devtools and create your own version.
-   Allow DevTools to be open in multiple windows.
-   Show more info like render tree, ...

## Clipboard Sync

-   Sync clipboard for copy/paste between computer and phone. Useful when you want to paste something from computer to mobile.
-   Also, keep track of clipboard history.

## Movie Series Youtube

-   Take popular movie series like resident evil and reorganize all the video clips in chronological order. Create a youtube channel for this.

## SAAS Builder

-   Tool similar to [saaspegasus](https://www.saaspegasus.com).
-   Do it for Rust and Qwik frontend. Include everything from zero to production rust book.

## Core Utils

-   GNU core utils can be made more consitent to have the same syntaxes and redundant ones removed.
-   There are faster alernatives to multiple and also some have better highlighting support like for `ls`.

## Dark Reader Video

-   Similar to Dark Reader extension, but for videos like Youtube.
-   Add an option to blur/black a section of the video. Sometimes you are watching a video with chat in one corner that you want to hide, to avoid spoilers.

## Development Extensions

-   Merge all the web-development related extensions into one big extension.
-   For each step from ui/ux, html, css, js, look at how an extension can be used to replace tools like color picker, ...

## Productivity Extensions

-   Merge all the productivity related extensions into one big extension.

## Automatic Marketing Images/Videos

-   Every project needs a feature/marketing page, whch will have screenshots, videos.
-   These can get outdated as the app updates (also in documentation).
-   Create a tool to automatically create the images and videos, which the marketing/documentation page can link to. When the app updates, the images/videos are updated.

## JavaScript Performance Subset

-   Create an Emmet like extension for vscode that takes the syntax `p_{name}_{args}`. For example to get code for optimized for loop it will be `p_for_i_arrName`.
-   Go through all JS features, look at how each feature is implemented at assembly or some lower level to get idea of max performance. And create shorthands for that.
-   In vscode create snippets starting with `k_`. And make it for every js feature like loop, reduce, ...
-   Document a subset of JS that should be used for performance.

### Rewrite Browser APIs

-   JS built in APIs have weird exceptions, like URL function returns empty string for certain ports.
-   Rewrite all these functions without the weird quirks.
-   For APIs that are hard to use, provide a wrapper for easier use.
-   Avoid having multiple smaller functions that are a wrapper over some other function.

## Nvidia GUI

-   Make a GUI for all of nvidia ecosystem.
-   Script that includes setting up the libraries and further installing multiple versions.
-   Documentation that includes project setup on all possible options, like personal computer, distributed computer, cloud, nvidia hardware.
-   How to choose the hardware for your application and run the desired application.

## Physical Calendar Template

-   Create a template for a physical calendar.
-   Customize it per your needs (without any branding) adn then print it on different paper sizes.
-   Start week can be Sunday/Monday. Add sections for notes.
-   Add national holidays for your country/manually.
-   Add your own yearly plans (like dentist visit, car maintinance, recurring events).

## Attraction Identifier

-   When crossing rivers, mountains, attractions you wonder what that is.
-   Create app that can just show the attraction around you as travelling.
-   The app can include attraction categories like town, rivers, mountains, ...

## Linux Email Forum

-   Convert linux kernel development email into online form thing, where all the updates from email are visible in a searchable form.
-   Maybe allow people to post from form to email chain.
-   Look at other popular open-source tools that still use emails.

## Automation Tool

-   Create an automation tool similar to zapier, n8n.
-   Look at all the tools used by companies like gmail, ... and create a comprehensive automation using the official apis.

## Phone Contact Block

-   Phone app with option to block people.
-   Block people in a time range, day range. When blocked, people's call would just keep on ringing so that they don't know that they are blocked.
-   Extend this to apps that let you do call/video calls as well.

## Medical Record Aggregator

-   All medical records are online, and there is a risk these might get altered without you knowing, or you might lose access to them.
-   Create a version of these documents, plus any other document that is relevant like insurance, bank statement.
-   Help autoparse the document for searchability.
-   Add records of family.

## Library Book Creator

-   There is a standard on how to create library books. Convert this into an interactive web page, and maybe work with an actual bookbinder and create 3d visuals of the process.

## Clash Royale Clone

-   Clash royale clone with the original gameplay and cards. Have a random mechanism where cards get buffed, nerfed every two weeks

## Drone Wedding Show

-   Drone show for weddings, events.
-   For weddings, it can be a story that the bride/groom want to convey. On like how they met.
-   Cupid aroow at end with the bridge, groom initals.

## Home Photographer

-   Services include home photography, 3d model, videos.
-   Check sites like Zillow for other things that are relevant to home buyers.

## VR Biology

-   Similar to BRanch Education on youtube and VR being used in car manufacturer industry.
-   Create 3D designs for human/animal.
-   How different diseases/problems propagate or evolve in human body. Like how dry eyes evolve, smoking affects the lungs.

## 3D Map Caves

-   Drone to 3D map water caves. Put sensors to get various layers of data from the walls.

## Linux Desktop Storybook

-   Show how the desktop apps render on different Linux distributions.

## Password Generator/Reset

-   Password geenrator app. "How strong should your password by by mental outlaw".
-   In the password tool, have it where it automatically helps you resets your password on the websites you choose.
    -   Maybe a monthly popup where is opens the necessary windows back and forth, and you copy paste the passwords with ease.
-   Integrate "have i been pawned" and other security websites.

### Email Unsubscriber

-   Go through all emails and find the subscriptions and show them in a list.
-   Integrate have i been pawned and other security related services to email.

## PGP Chat Client

-   Mental outlaw video "Make any messaging service E2E Encrypted with PGP". Create a client for this.
-   Merge all features from popular chat services like slack, discord, getstream, uwebsocket.

## Trail Hardness Score

-   Use incline, decline elevation and length of trail to score the hardness of a trail.

## Rust Standard Library

-   Create rust standard library, with all the data structures. and optimize them as much as possible and have graphs showing improvements.
-   Document the data structure in detail with all the optimizations.
-   Look into optimizing the data structures with cpu specific stuff.

## Stripe Clone

-   Create an api similar to stripe and webpage similar to stripe/square.
-   Have all the payment options that can be added, without requiring to be a payment processor like stripe.
-   For example, stripe takes a cut of google pay, but if you set it by yourself, you don't have to pay that cut.
-   Add invoicing options. Look at invoices generated by ERP systems used by enterprise.
-   Add taxes (including international) as much as you can.
-   Create dashboard with graphs to show demographics/money and other stuff.
-   Regional pricing support.

## Biology SAAS

-   Combine all software related products from fields such as biochemistry, gene related, anti aging, ....
-   Check conferences as well.

## Agriculture SAAS

-   Combine all software related products from agriculture related fields.

## Unit Test Builder

-   For each parameter to a function, make a list of all possible options. Like for an integer it would be min, max, 0. And there can be user defined constraints like if conditions.
    -   Additional constraints can be network loss, program crash, interrupts.
-   Use this to create an exhaustive list of unit tests.

### Website Simulation Testing

-   Provide an array of all actions that a user can take, and the simulator will run those randomly.
-   This includes everything like you are on a page, the user has the options to close the tab, open new tab, change url of current tab and lot more, and also click the buy button on the page.

## Struct Memory Visualizer

-   Struct in languages like c#, rust have memory alignment issues.
-   Create tool to get size of all sturcts in your code, and give suggestions using data-oriented design on how to optimize.

## Chrome MultiLogin

-   Create tool similar to sessionbox, multilogin chrome extension.
-   When opening outlook, you can only have one account open in the entire chrome window across multiple tabs.
-   Create an extension that allows you to limit a website cookies, cache to a single tab. Also, provide full control over the user-agent, in the sense you can modify the user-agent per tab.

## Book Reading Helper

-   People are reluctant to read texts that use unfamiliar words.
-   Have a feature when reading online where the meaning of words from dict or from llm would be useful.
-   Definition of a word from dict is not that useful, as we need to know what the meaning of the word in the current context is.
-   Reference _the reading mind_ book on what other things can be added, like sound of word.
-   Create a tool that can be used by teachers to help improve reading in schools and by parents.
-   Add all the grammer stuff like subject, noun, verb, adjective, adverb. In each sentence we can classify the words into what grammer they belong to.

## Health Insurance Benefit Tracker

-   Track health insurance benefits.
-   Have an app which shows stuff like this and helps you set reminders to use all the benefits.
-   It also shows the amount of benefits consumed.

## Exercise App

-   App for exercise tracking.
-   Database with all exercises and how to perform and what body parts they target.
-   Include calorie and nutrition tracker.

## Math Book

-   Problem with math books, they are too abstract. Textbooks pull abstractions out of thin air. Yet mathematicians don't randomly set up axioms and start working out theorems.
-   Every axiomatic system had a history. Had a motivation. Worked through loads of examples and counterexamples to motivate and finally settle on a specific definition that's both general enough to capture a sufficiently wide variety of interesting problems, but also specific enough so that we can actually prove interesting results about them.
-   https://www.youtube.com/watch?v=D0ZVe9AtgUg Create your own math book for subfields of math, and do motivation, history, examples, use cases, arguments against and then do the usual stuff that current books do.
-   The book progresses in chronological order, and you develop theory and the applications. Interactive diagrams as done by Brilliant.
-   Other topics cryptography, networking, theory of computing, linear algebra, differential/intergral calculus, information theory, matrices, digital electornics, analog electronics, computer architecture, operating system, quantum computing.

## How Internet Works

-   Make a detailed note on how internet works i.e. how do you make request for a webpage and get it served.
-   For every block go deeper like all the network layers, encryption, bits, assembly. DNS cables getting routed across the world.

## Small World Problem Simulator

-   Simulate small world problem https://www.youtube.com/watch?v=CYlon2tvywA.
-   Get population of each country, and further every city and zip code. Now you know exactly how poeple are distributed across the world.
-   Next add probabilities of interaction based on geography and reduce as you go back further. Make these numbers available as controls.
-   Have additional controls like railway connections, airport connections, visa free entry, gdp.

## Homelab Builder

-   Create detailed notes on how to create homelab/homedatacenter.
-   From setting up custom power solution and other electricity related stuff, to rack, to ups, to hardware, networking, software.

## Weather App

-   Google weather app has forecast for activities like running, camping. It gets the data from the weather channel.
-   Create a better user interface using google weather/maps api and the weather channel.
-   AccuWeather is bad, do not use it.
-   Look at all the weather apps and combine their features into a single app.
-   Look at how the weather channel gets data from sources and make your own model, so as to not rely on the weather channel.
-   National weather service better option than the weather channel (as the weather channel has some commerical restlrictions around their API).

## Media Viewer

-   Single media player, that can show images, videos, audio, pdf.
-   Include all the formats, hdr, display supports.
-   Add common editing features, and also applying the same editing instructions to batch of inputs.

## Anti DRM

-   Modify chrome browser, display driver, gpu driver to get anti DRM to work.
-   Test for all popular services and see if you can download the raw video, audio, images.

## Wayback Timemachine

-   Timemachine app that lets you backup a website from a URL. Includes manual backup and automatic interval backups.

## Rust JSF Standard

-   [youtube video](https://www.youtube.com/watch?v=Gv4sDL9Ljww) Why Fighter Jets Ban 90% of C++ Features
-   Create Rust equivalent of JSF standard.
    -   Implement static code analyzer to enforce the standard.
    -   Make a subset of Rust documentation that that shows only the allowed features. Go through the entire Rust API to include functions (and example usage).
    -   Add [Unit Test Builder project](#unit-test-builder) and code test coverage.
    -   Add flight simulation demo.
-   Use [Xplane](https://www.x-plane.com/desktop/buy-it/) simulator to prepare a flight demo ([example code](https://github.com/LaurieWired/XplaneFlightData)).
    -   Look at how to make the most comprehensive aerospace demo.
    -   But [F35B](https://store.x-plane.org/AOA-Simulations_bymfg_71-0-1.html) jet by AOA simulations
-   Standards to consider
    -   [JSF](https://www.stroustrup.com/JSF-AV-rules.pdf)
    -   [F Prime](https://fprime.jpl.nasa.gov/)
    -   [MISRA](https://misra.org.uk/shop/)
    -   [AUTOSAR](https://www.autosar.org/fileadmin/standards/R17-10_R1.2.0/AP/AUTOSAR_RS_CPP14Guidelines.pdf)
    -   [C++ Core Guidelines](https://github.com/isocpp/CppCoreGuidelines)

## Delete Consumer Reporting Data

-   [List of consumer reporting companies](https://www.consumerfinance.gov/consumer-tools/credit-reports-and-scores/consumer-reporting-companies/companies-list/).
-   Make a guide/automation tool on deleting all the info from the listed companies.
-   Query to view free report.
-   Data brokerages might be doing the same thing.

## Phone Rotation By 180

-   Phones can't be flipped to have header on the charge side, which is annoying in some cases when you need to use charger.
-   Create app to enable this flip. Check this tool as [reference](https://play.google.com/store/apps/details?id=com.pranavpandey.rotation).

## How To Use Website

-   Have a tutorial button on website to show how to use it.
-   It opens popovers with button for next/previous and close. Including table of contents to jump to certain sections.
-   The tutorial spans multiple urls.
-   The tutorial further includes transition periods that require users to press buttons, use the website.

## Chromium Pipeline Accessibility

-   Modify chromium pipeline, to include all things related to accessibility.
-   For images/videos, render them using dark mode, color blindness modes. Look at other tags and see what you can add.
-   Look at [Accessibility project](#accessibility) as well.
