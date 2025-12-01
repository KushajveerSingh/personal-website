---
title: Svelte
parent: Reorganize
grand_parent: Frontend
nav_order: 3
---

<!-- prettier-ignore-start -->
# Svelte
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

## .svelte syntax

```svelte
<script module>
    // Only ran once, when the module first evaluates.
</script>

<script lang="ts">
    // Run when component instance is created.
</script>

<!-- HTML markup -->

<style>
    /* Apply to current component only */
</style>
```

### .svelte.ts

-   Behaves same as regular `.ts` files, except you can use runes.

## $state

-   Create reactive state i.e. UI changes when state changes.
-   You modify the actual state when using `$state`, rather than reassigning the whole object like in React.

```svelte
<script>
    let count: number = $state(0);

    // If not providing a default value, and you know a value would be present before first use
    let count = $state() as number;
</script>

<button onclick={() => count++}>
    clicks: {count}
</button>
```

### Object

-   In case state is an object, making changes to a property, will only trigger UI changes that depend on that property.
-   If you destructure a reactive value, the references are not reactive.

```ts
let todos = $state([
    {
        done: false,
        text: "add more todos",
    },
]);

let { done, text } = todos[0]; // `done`, `text` are not reactive, since you destructured

todos[0].done = !todos[0].done; // This will not affect the value of above destructured variable
```

### Classes

Internally the state variables are converted to get and set methods.

```ts
class Todo {
    done = $state(false);

    constructor(text) {
        this.text = $state(text);
    }

    reset = () => {
        this.text = "";
        this.done = false;
    };
}
```

### $state.raw

-   To reassign the entire object in the state, similar to React.
-   Better for performance when you have large arrays and objects that you don't want to mutate. Raw state can contain reactive state.

```ts
let person = $state.raw({
    name: "John",
    age: 10,
});

person.age += 1; // This will not change state

person = {
    // This will change state
    name: "John",
    age: 12,
};
```

### $state.snapshot

-   Will output the actual state rather than `Proxy`.
-   Useful for passing state to external API that does not work with Proxy, or `console.log`.

```ts
console.log($state.snapshot(counter));
```

### $state.eager

-   Svelte manages updating the UI. A consequence is sometimes state changes are not reflected in the UI immediately.
-   To update the UI as soon as the state changes use `$state.eager` (use it sparingly).

```svelte
<nav>
    <a href="/" aria-current={$state.eager(pathname) === '/' ? 'page' : null}>home</a>
</nav>
```

### Module sharing

State can be declared in `.svelte.ts` file, you can only be exported if the state is not being modified in the `.svelte.ts` file as well.

```ts
// This does not work, since `increment` is modifying the state
export let count = $state(0);

export function increment() {
    count += 1;
}
```

You can export the function instead

```ts
// This works
let count = $state(0);

export function increment() {
    count += 1;
}

export function getCount() {
    return count;
}
```

### Date, Map, Set, URL

Changing the values in _effect_ or _derived_ will cause the state to be re-evaluated.

```ts
import {
    SvelteDate,
    SvelteMap,
    SvelteSet,
    SvelteURL,
    SvelteURLSearchParams,
} from "svelte/reactivity";
```

## $derived

-   Derived state. When original state changes, the derived state is marked dirty and will be recalculated when it is next read.
-   State change not permitted inside derived expression. For example `$derived(count++)`, since it modifies `count`.
-   This is also shorthand for `$derived.by(() => expression)`.

```svelte
<script lang="ts">
    let count = $state(0);
    let doubled = $derived(count + 2);
</script>

<p>{count} doubled is {doubled}</p>
```

### $derived.by

-   Use function to calculate derived state.

```ts
let numbers = $state([1, 2, 3]);
let total = $derived.by(() => {
    let total = 0;
    for (const n of numbers) {
        total += n;
    }
    return total;
});
```

### untrack

-   To mark state not to be used as a dependency inside `$derived.by`.

```ts
// `derived_state` will not change when `state_variable` changes
let derived_state = $derived.by(() => {
    const some_state = untrack(() => state_variable);
});
```

### Temporary change

-   Derived values can be modified temporarily.
-   Useful in optimistic UI, where you want to show feedback to the user immediately.
-   The state will be recalculated when the dependency changes.

```ts
let likes = $derived(post.likes);

async function onclick() {
    likes += 1; // Optimistic UI update, to give feedback to user immediately

    try {
        await like(); // Server side code, which will eventually update the `likes`
    } catch {
        likes -= 1;
    }
}
```

### Destructuring

-   Unline `$state` destructured variables of `$derived.by` will be reactive.

```ts
let { a, b, c } = $derived(stuff());

// The above is same as
let _stuff = $derived(stuff());
let a = $derived(_stuff.a);
let b = $derived(_stuff.b);
let c = $derived(_stuff.c);
```

## $effect

-   Code that runs only in the browser, not during server-side rendering.
-   Use for things like analytics, direct DOM manipulation. Use sparingly.

    ```svelte
    <script>
        let size = $state(50);

        let canvas;

        $effect(() => {
            // Fill in the canvas
        });
    </script>

    <canvas bind:this={canvas}></canvas>
    ```

-   Do not update state inside effects.
-   `effect` function is reran, when the state/derived dependencies value change.
-   `effect` runs after the component is mounted to the DOM. And re-runs are batched (and applied after DOM updates), so chnages to multiple state variables will not cause multiple re-reruns.
-   State that is read asynchronously (after an await or in setTimeout), won't be registered as dependencies.
    ```ts
    $effect(() => {
        setTimeout(() => {
            // State changes here won't trigger effect re-reun
        }, 9);
    });
    ```
-   Effect only reruns when the object it reads changes, not when a property inside it changes (use `$inspect` instead).

    ```ts
    let state = $state({ value: 0 });
    let derived = $derived({ value: state.value * 2 });

    // Will run only once. Since `state` is never reassigned.
    $effect(() => {
        state;
    });

    // Will run whenever `state.value` changes
    $effect(() => {
        state.value;
    });

    // Will run whenever derived changes, since `derived` is a new object each time
    $effect(() => {
        derived;
    });
    ```

-   Effect only depends on the values it read the last time it ran. This has a side-effect when using conditionals.

    ```ts
    let condition = $state(true);
    let color = $state("#ff3e00");

    $effect(() => {
        // If condition is true, then both condition and color will trigger a re-rerun
        if (condition) {
            confetti({ colors: [color] });
        } else {
            // If condition is false, only condition will trigger the re-run
            confetti();
        }
    });
    ```

### teardown function

```ts
let milliseconds = $state(1000);

$effect(() => {
    // This will be recreated whenever `milliseconds` changes
    const interval = setInterval(() => {
        count += 1;
    }, milliseconds);

    return () => {
        // if a teardown function is provided, it will run
        // a) immediately before the effect re-runs
        // b) when the component is destroyed (or parent's effect re-reruns)
        clearInterval(interval);
    };
});
```

### $effect.pre

-   For cases when you need to run code before DOM updates.
-   Everything else works the same as `$effect`.

```svelte
<script>
	import { tick } from 'svelte';

	let div = $state();
	let messages = $state([]);

	// ...

	$effect.pre(() => {
		if (!div) return; // not yet mounted

		// reference `messages` array length so that this code re-runs whenever it changes
		messages.length;

		// autoscroll when new messages are added
		if (div.offsetHeight + div.scrollTop > div.scrollHeight - 20) {
			tick().then(() => {
				div.scrollTo(0, div.scrollHeight);
			});
		}
	});
</script>

<div bind:this={div}>
	{#each messages as message}
		<p>{message}</p>
	{/each}
</div>
```

## $props

To pass inputs to components.

```svelte
<script lang="ts">
	import MyComponent from './MyComponent.svelte';
</script>

<MyComponent adjective="cool" />
```

In the component, use the prop

```svelte
<script lang="ts">
    interface Props {
        adjective: string;
    }

	let props: Props = $props();

    // Or
    let { adjective }: Props = $props();

    // Or with default value
    let { adjective = 'happy' } = $props();

    // Or rename prop name
    let { adjective: new_name = 'happy' } = $props();

    let { adjective, ...others } = $props();
</script>
```

To add types for HTML elements

```svelte
<script lang="ts">
    import type { HTMLButtonAttributes, SvelteHTMLElements } from 'svelte/elements';

    let { children, ...rest }: HTMLButtonAttributes = $props();
    let { children, ...rest }: SvelteHTMLElements['div'] = $props(); // In case of a div
</script>

<button {...rest}>
    {@render children?.()}
</button>
```

### Updating props

-   Child component re-renders when prop changes.
-   Child component can temporarily change value.
-   Avoid mutating props from the child. If mutation is necessary communicate using callback props, or use `$bindable`.

### $props.id

-   Used to get a consistent `id` between the server and client, for the component.

```svelte
<script>
	const uid = $props.id();
</script>

<form>
	<label for="{uid}-firstname">First Name: </label>
	<input id="{uid}-firstname" type="text" />

	<label for="{uid}-lastname">Last Name: </label>
	<input id="{uid}-lastname" type="text" />
</form>
```

## $bindable

-   `$props` - is for state going from parent to child.
-   `$bindable` - is for state going from child to parent (use sparingly).

Child component

```svelte
<script lang="ts">
	let { value = $bindable('fallback'), ...props } = $props();
</script>

<input bind:value={value} {...props} />
```

Parent component

```svelte
<script lang="ts">
	import FancyInput from './FancyInput.svelte';
	let message = $state('hello');
</script>

<FancyInput bind:value={message} />

<!-- For parents that are not interested in child's prop -->
<FancyInput />
```

## $inspect

-   Used only in development.
-   It will console.log whenever the variable changes

```js
let count = $state();
let message = $state();

$inspect(count, message);
```

### Context

-   Share state between parent and children without passing as props.
-   Solves the problem of having global state.

```ts
import { createContext } from "svelte";

export const [getUserContext, setUserContext] = createContext<User>();
```

Avoid global state, as there is a risk of mutating the state during server-side rendering.

```ts
import { myGlobalState } from "./state.svelte.js";

let { data } = $props();

if (data.user) {
    myGlobalState.user = data.user; // This can cause issue, as the data might
    // be accessible by the next user
}
```

## Special elements

### \<svelte:boundary>

-   Provide UI that should be shown when `await` expressions are first resolving.
-   Provide UI to show in case of errors.

```svelte
<svelte:boundary>
    <p>{await delayed('hello!')}</p>

    {#snippet pending()}
        <p>loading...</p>
    {/snippet}

    {#snippet failed(error, reset)}
        <button onclick={reset}>oops! try again</button>
    {/snippet}
</svelte:boundary>
```

### \<svelte:window>

-   To add window event listeners.

```svelte
<script>
    function handleKeydown(event) {
        alert(`pressed the ${event.key} key`);
    }
</script>

<svelte:window onkeydown={handleKeydown} />
```

You can bind to the following properties

-   innnerWidth
-   innerHeight
-   outerWidth
-   outerHeight
-   scrollX (readonly)
-   scrollY (readonly)
-   online
-   devicePixelRatio

```svelte
<svelte:window bind:scrollX={x} />
```

### \<svelte:document>

-   To add listeners to the `document`.

```svelte
<svelte:document onevent={handler} />
<svelte:document bind:prop={value} />
<svelte:document onvisibilitychange={handleVisibilityChange} use:someAction />
```

### \<svelte:body>

-   To add listeners to `document.body`.

### \<svelte:head>

-   To add elements to `<head>`.

```svelte
<svelte:head>
    <title>...</title>
</svelte:head>
```

## HTML markup

### Comment tag

The comment will appear when you hover over the component.

````
<!--
@component
- Use markdown.
- Usage:
    ```html
    <Main name=...>
    ```
-->
<script>
    let { name } = $props();
</script>

<main>
    <h1>
        Hello, {name}
    </h1>
</main>
````

### {#if}

```svelte
{#if expression}
    ...
{:else if expression}
    ...
{:else}
    ...
{/if}
```

### {#each}

-   Can use anything that can be used with `Array.from`.

```svelte
{#each items as item, index}
    ...
{/each}
```

Keyed each blocks, allow Svelte to intelligently remove, insert items.

```svelte
{#each items as item, index, (item.id)}
    <!-- You don't have to use item.id here -->
{/each}
```

Destructuring is supported

```svelte
{#each items as { id, name, qty }, i (id)}

{/each}
```

Else block can be added, for when the list is empty

```svelte
{#each items as item}
    ...
{:else}
    ...
{/each}
```

### {#key}

-   Key blocks destroy and recreate their contents when the value of an expression changes.

```svelte
{#key value}
    <Component />
{/key}
```

### {#await}

```svelte
{#await promise}
    <p>Waiting for promise to resolve</p>
{:then value}
    <p>The value is {value}</p>
{:catch error}
    <p>Something went wrong</p>
{/await}

<!-- Don't care about pending -->
{#await promise then value}
{/await}

<!-- Only care about error state -->
{#await promise catch error}
{/await}
```

This allows you to lazy load component

```svelte
{#await import('./Component.svelte') then { default: Component }}
    <Component />
{/await}
```

### {#snippet}

-   Cimilar to component, but you can define it inside existing `.svelte` file.
-   Can pass as prop to components as well.
-   For snippets defined inside component, they are automatically added to the prop.
-   Use `import type { Snippet } from 'svelte'`.

```svelte
{#snippet name(param1, param2, paramN)}...{/snippet}

{@render name(param1, param2, paramN)}
```

### {#html}

-   To inject raw HTMl into the component.
-   Scoped styles do not work on these. Instead use global styles.

```svelte
{@html content}
```

### {#const}

-   To define a local constant.

```svelte
{@const area = box.width + box.height}
```

### bind:

`<input bind:value={name}>`. Essentially what it does is, instead of having an event handler `oninput` to store the input value to a variable, `bind:value` will automatically store the value in `name` variable.

```svelte
<input type="number" bind:value={a} />
<input type="range" bind:value={a} />

<input type="checkbox" bind:checked={yes}>
<!-- In case of {# each } you can bind to properties of the item returns by each as well -->

<select bind:value={selected}>
<select multiple bind:value={flavours}>

<input type="radio" bind:group={scoops} />
<input type="checkbox" bind:group={flavours} />

<textarea bind:value={value}></textarea>

<div bind:innerHTML={html} contenteditable></div>
```

The following bind properties can be added to any element

-   `clientWidth`
-   `clientHeight`
-   `offsetWidth`
-   `offsetHeight`

### class

-   To conditionally set classes on elements.

```svelte
class="card {flipped ? 'flipped' : ''}"

<!-- Add 'flipped' only when 'flipped' is truthy -->
class={["card", { flipped }]}
```

## Event handlers

-   Add `capture` at the end to runt he handler during the capture phase of the bubbling process.

```
<div onkeydowncapture={(e) => alert(`<div> ${e.key}`)} role="presentation">
    <input onkeydowncapture={(e) => alert(`<input> ${e.key}`)} />
</div>
```
