---
title: Global Attributes
parent: HTML
grand_parent: Frontend
nav_order: 2
---

<!-- prettier-ignore-start -->
# Global Attributes
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

-   [link](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements) MDN HTML Elements

---

## accesskey

```html
<button accesskey="s">Button</button>
```

-   DO NOT USE.
-   Specify a single character as keyboard shortcut to focus/click elements. Button is clicked.
-   Chrome - Alt + key
-   Firefox - Alt + Shift + key
-   Safari - n/a
-   Bad for accessibility
    -   Shortcut keys can conflict with keyboard shortcut, assistive technology functionality.
    -   Difference in operating systems, keyboards.
    -   Internationalization concerns when supporting multiple languages.
    -   Need to educate users about the presence of shortcuts, so they do not accidentally activate it.

## autocapitalize (enumerated)

```html
<!-- Turned off -->
<input autocapitalize="none" />
<input autocapitalize="off" />

<!-- Turn on CAPS LOCK for the first character of each sentence -->

<textarea autocapitalize="sentences"></textarea>
<textarea autocapitalize="on"></textarea>

<!-- Turn on CAPS LOCK for the first character of each word -->
<div autocapitalize="words" contenteditable="true"></div>
<!-- Turn on CAPS LOCK for every character -->
<form autocapitalize="characters" />
```

-   Turn on the CAPS LOCK key when using virtual keyboards (mobile), voice input, to make data entry faster. Does not work with a physical keyboard.
-   Used on `<input>`, `<textarea>`, tags with `contenteditable=”true”`, `<form>` (overrides inner values).
-   Does not work with `input type=` url, email, password.
-   Chrome and Safari default to on/sentences, Firefox default to off/none, when no value is provided.

## autofocus (boolean)

```html
<input autofocus />

<dialog open>
    <button autofocus>Ok</button>
</dialog>
```

-   BEST NOT TO USE.
-   Focus the element on page load, or when `<dialog>` is shown.
-   If applied to multiple elements, only the first one will receive focus.
-   Accessibility concerns
    -   Autofocusing a form control can confuse visually-imparied people using screen-readers and people with cognitive impairments.
    -   Autofocus can cause scroll, and sudden appearance of dynamic keyboard. Or give the illusion of being teleported past important content.
    -   Label for the input will be announced by screen reader, but nothing before that.
    -   When not to use. If autofocus skips users past important information, then do not use.

## class

```html
<div class="class1 class2"></div>
```

-   Use class names that describe the semantic purpose of the element, rather than presentation of the element.

## contenteditable (enumerated)

```html
<!-- Enable editing (use Ctrl-b to bold, ...). Formatting preserved on paste. -->
<div contenteditable="true">Text1</div>
<div contenteditable="" style="caret-color: red;">Text2</div>

<!-- Disable editing -->
<div contenteditable="false">Text3</div>

<!-- Disable rich text formatting. Formatting removed on paste. -->
<div contenteditable="plaintext-only">Text4</div>
```

-   DO NOT USE.
-   Used originally to create text editors in Explorer days, with rich text formatting like bold, italics. Use a dedicated library instead.
-   Setting the attribute makes the element focusable, but not tabbable.
-   Value inherited from parent if not specified.

## data-\*

```html
<div data-id="some-name">...</div>

<!-- Do not use non-alphabetic characters followed by hyphen -->
<div data-id="some-name-1">...</div>
```

-   Data available via HTMLElement.dataset property.
-   Do not use names that start with xml, contain colon, contain capital letters, to preserve space for XML spec.
-   In JS use `ele.getAttribute('data-some-name')` or ele.dataset.someName.

## dir (enumerated)

```html
<p dir="ltr">Left To Right</p>
<p dir="rtl">Right To Left</p>
<p dir="auto">Use for external data or user input.</p>

<img alt="Will appear right-to-left" dir="rtl" />
<table dir="rtl">
    <!-- Col1 will appear last-->
    <tr>
        <th>Col1</th>
        <th>Col2</th>
    </tr>
</table>

<bdo dir="rtl">Mandatory for bdo element.</bdo>
<bdi dir="auto">Value not inherited from parent. And default is 'auto'.</bdi>
```

-   Do not use CSS direction and unicode-bidi to set these values, since the direction of text is semantically related to text and not to its presentation.
-   In `<input>` and `<textarea>` Chrome/Safari provide a directionality option in the contextual menu (right-click inside input field). In Firefox use Ctrl + Shift + X to toggle text direction.
-   Value inherited from parent if not specified.

## draggable (enumerated)

```html
<!-- Allowed values true, false, auto (set by browser) -->
<p draggable="true">Text</p>

<script>
    const draggable_ele = document.querySelector('p[draggable="true"]');

    draggable_ele.addEventListener("dragstart", (e) => {
        e.dataTransfer.setData("text/plain", "This appears when dropped.");
    });
</script>
```

-   Images, links, selections are draggable by default. Do not modify the behavior of these, as this is controlled by browsers and provides consistency across websites.
-   For other elements, set draggable=”true” and add dragstart listener with drag data (when image/link are dragged, the URL is set as the drag data).
-   The content in drag data is pasted/inserted when the mouse button is released, which may happen in a completely separate window.

## enterkeyhint (enumerated)

```html
<!-- Insert new line. (return, ↵) -->
<input enterkeyhint="enter" />

<!-- Nothing more to input and close the insert editor. (done, ✅) -->
<input enterkeyhint="done" />

<!-- Take user to the target of the text they typed. (go, 🡢) -->
<input enterkeyhint="go" />

<!-- Take user to the next field that will accept text. (next, ⇥) -->
<input enterkeyhint="next" />

<!-- Take user to the previous field that will accept text. (return, ⇤) -->
<input enterkeyhint="previous" />

<!-- Take user to the results of searching for the text they typed. (search, 🔍) -->
<input enterkeyhint="search" />

<!-- Typically delivering the text to its target. (send) -->
<input enterkeyhint="send" />
```

-   Provide what action label or icon to show on virtual keyboards for the enter key.
-   If no value is provided, information from inputmode, type, pattern used to make a guess.

## hidden (enumerated)

```html
<span hidden="hidden">Hidden</span>
<span hidden="">Hidden as incorrect val</span>
<span hidden>Hidden as incorrect val</span>

<a href="#hidden-content">Go to hidden content</a>
<div id="hidden-content" hidden="until-found">I'm hidden until found</div>
```

-   Use `ele.removeAttribute('hidden')` to remove.
-   Used to hide content that is currently not relevant to the page. Or that is being used to declare content for reuse by other parts of the page and should not be directly presented to the user. For example, content that should be shown only after the user is logged in.
-   Hidden from screen readers as well. Use aria-describedby to refer to hidden descriptions, to provide additional context.
-   `<canvas>` can use hidden for an off-screen buffer. `<form>` can use it to hide CSRF tokens.
-   Setting display using CSS override's hidden attribute.
-   Browsers implement `hidden="hidden"` using `display: none`.
-   Browsers implement `hidden="until-found"` using `content-visibility: hidden`. Element is shown when Ctrl-F leads to it or href directs to it. beforematch event is fired before removing the hidden attribute.
-   Does not work with inline-elements, as it requires elements to be affected by layout containment.

## id

```html
<div id="someid"></div>
```

-   Define an identifier that is unique within the entire document.
-   Do not use `window.someid` and `window['someid']` to access elements, as it can cause conflicts with future or existing APIs in the browser.
-   Use `document.getElementById()` or `document.querySelector()`.

## inert (boolean)

```html
<style>
    [inert],
    [inert] * {
        opacity: 0.5;
        pointer-events: none;
        cursor: default;
        user-select: none;
    }
</style>
<div inert>Something</div>
```

Inert means element cannot be clicked, focused, prevents the content from showing in Ctrl + F, prevents users from selecting/editing text within the element. Hides the element and its content from the accessibility tree.
When a <dialog> is opened, the background (or <body>) becomes inert, and can only interacted again after closing the dialog.
For accessibility, useful for focus trapping. inert lets you control the discoverability and interactivity of elements.
When element is offscreen or hidden, like a drawer. Use inert when drawer is hidden to ensure keyboard user cannot accidentally interact with it.
When element is non-interactive, like during page load, while a form is submitting. Carousel with non-active items, non-applicable form content.
Accessibility concern for visual viewers.
Need to manually indicate the content parts that are active and those that are inert (<dialog> does this by dimming the background).
Users may be zoomed in a small section of content. This can cause issues if entire screen gets covered by inert content, and they don't know what to do.

inputmode (enumerated)
<input inputmode="none" /> No keyboard. In cases where input should not show keybaord.
<input inputmode="text" /> Default.
<input inputmode="decimal" /> Digits 0-9, decimal (full-stop or comma),maybe minus key.
<input inputmode="numeric" /> Digits 0-9, maybe minus key.
<input inputmode="tel" /> Digits 0-9, asterisk, pound key. Use <input type="tel"> instead.
<input inputmode="search" /> Use <input type="search"> instead.
<input inputmode="email" /> Use <input type="email"> instead.
<input inputmode="url" /> Use <input type="url"> instead.

Specify the virtual keyboard to use with <input> or contenteditable.

Microdata
itemid
itemprop
itemref
itemscope
itemtype

<div itemscope itemtype="https://schema.org/SoftwareApplication">
  <span itemprop="name">Angry Birds</span> - REQUIRES
</div>

CAN BE SKIPPED.
Alternative to using JSON-LD for specifying schema.org. Prefer JSON-LD, since it is easier to maintain, and parse by tools.
itemtype - URL to schem.org vocab. https://schema.org/SoftwareApplication.
itemscope - Specify alongside itemtype.
itemprop - Add properties to item defined by itemtype.
itemref - Use alongside itemscope when adding a property that is not part of type defined by itemtype.
itemid - Unique, global identifier of an item.

lang

<html lang="en-US">

Define language in which non-editable elements are written in, or the language that the editable elements should be written in by the user.
Specify the language using BCP 47 language tags.
Inherited from parent. Default is unknown.

nonce
Used by Content-Security-Policy to specify nonce (number used once), to avoid unsafe-inline directive with inline script or style elements.
Alternative, is to compute the hash of the contents inside <script> tag and set that in the CSP header. nonce is better since the inline-content can change independent from the hash value.
Steps to use nonce
Generate a unique nonce server-side on each page load.
import crypto from 'node:crypto';
crypto.randomBytes(16).toString('base64');

Add the nonce to the inline-script tag.

<script nonce="8IBTHwOdqNKAWeKl==">...</script>

Send the nonce value in the CSP header or in <meta> tag.

<meta http-equiv="Content-Security-Policy" content="script-src 'nonce-8IBTHwOdqNKAWeKl==';">

In JS access the value using script.nonce.

popover (enumerated)

<!-- Same as using popover, popover="", popover="auto" -->

<button popovertarget="auto">Open auto popover</button>

<div popover="auto" id="auto">
  Can be light dismissed. Will close other 'auto' popovers, except nested.
</div>

<button popovertarget="hint">Open hint popover</button>

<div popover="hint" id="hint">
  Same as 'auto' but will only close other 'hint' popovers. If inside 'auto', then uses 'auto' rules.
</div>

<button popovertarget="manual">Open manual popover</button>

<div popover="manual" id="manual">
  Close popover is by clicking 'Close' button or clicking the triggering button again.
  <button popovertarget="manual" popovertargetaction="hide">Close</button>
</div>

Appear in the top layer, so no need to use z-index.
Clicking outside the popover area will close the popover and return focus. Esc can be pressed to do the same, as well as double toggling the button used to open the popover.
Popover elements hidden via display: none until opened.
ele.showPopover() and ele.togglePopover().
Use auto for persistent UI like menus and dialogs.
Use hint for ephemeral UI like hovercards and tooltips. A common UX pattern for tooltips is to be hover triggered using mouseenter and mouseleave events. In future, you can use interesttarget.

spellcheck (enumerated)

<form spellcheck="false">
  <textarea>
    This exampull will nut be checkd fur spellung when you try to edit it.
  </textarea>
</form>

Hint for browsers, which can be ignored. Browser default is spell check enabled.
In Chrome, there is a local spell check and an enhanced spell-check which first sends the data in the field to the cloud. For password fields the data is sent when you click reveal password. Need to manually turn it on from the Chrome settings.
To avoid spell-jacking, set spellcheck=”false” for sensitive data, or set it on the <form>.

style

<div style="">...</div>

Should primarily be used for testing purposes.
Do not use it to hide irrelevant information, use hidden instead.

tabindex

<div role="group" id="errorSummary" aria-labelledby="errorSummaryHeading" tabindex="-1">
  <h2 id="errorSummaryHeading">Your information contains three errors</h2>
  <ul>
  ...
  </ul>
</div>

Make elements focusable, allow or prevent them from being sequentially focusable, and determine their relative ordering for sequential focus navigation.
Prefer to write the HTML document with the elements in a logical sequence rather than relying on tabindex.
tabindex=”-1” can be useful for the errors received after submitting a form. It helps draw attention to the errors, and at the same time avoids people tabbing into the error message.
tabindex=”0” not that useful, since you should be using semantic tags in the first place.
Do not use values greater than 1, as that breaks the normal flow of the page.

title

<iframe
  title="Convey content of iframe to screen reader"
  src="">
</iframe>

BEST NOT TO USE.
Main use is to label <iframe>. With <abbr> use it to provide the full abbreviation.
Value inherited from parent.
Accessibility concerns for people
Using touch-only devices, navigating with keyboard, navigating with screen readers or magnifiers.
Experiencing fine motor control impairment.
With cognitive concerns.

translate (enumerated)

<!-- Do not translate company name -->

<small>(c) 2020 <span translate="no">BrandName</span></small>

To mark content that should not be translated, by automatic systems like Google Translate.

virtualkeyboardpolicy (enumerated)

<div contenteditable virtualkeyboardpolicy="manual"></div>

To control on-screen keyboard behavior. Can be auto or manual.
In auto the browser controls the keyboard. For example, an input field a user is about to type into might be obscured by the virtual keyboard, so the browser has to scroll it into view.
For multi-screen devices, this can result in wasted screen real estate, as we would prefer to show the keyboard on only one side, and leave the other side unmodified.
Details on controlling the keyboard are in VirtualKeyboard API.

writingsuggestions (enumerated)
<input writingsuggestions="false" />

Browsers provided typing suggestions can be disabled using this. These suggestions appear as greyed-out text positioned after the text cursor. These can be turned off when providing site-specific writing suggestions.
