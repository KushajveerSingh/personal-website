---
title: Zod
parent: Reorganize
grand_parent: Frontend
nav_order: 1
---

<!-- prettier-ignore-start -->
# Zod
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

## Overview

```ts
import * as z from "zod";

const Player = z.object({
    username: z.string(),
    xp: z.number(),
});

const result = Player.safeParse({ username: 42, xp: "100" });
if (!result.success) {
    result.error;
    // result.error.issues is the array of all issues
} else {
    result.data;
}

// Use "safeParseAsync" in case of async APIs

// Generate TypeScript Type
type Player = z.infer<typeof Player>;
const player: Player = { username: "john", xp: 100 };

// To get error for each property easily (use for deeper nested schema)
const tree = z.treeifyError(result.error);
tree.properties?.username?.errors; // To get error associated with username
tree.properties?.xp?.errors;

// Use for one-level deep schemas
const flattened = z.flattenError(result.error);
flattened.fieldErrors.username;

// Get a pretty string output of all errors
const pretty = z.prettifyError(result.error);
```

## Primitive

```
z.string()
z.number() // Does not include NaN and Infinity
z.bigint()
z.boolean()
z.symbol()
z.undefined()
z.null()
z.void() // same as z.undefined()
z.optional(z...) // To allow undefined
    z.string().optional() // Same as above
z.nullable(z...) // To allow null
    z.string().nullable() // Same as above
z.nullish(z...) // Allow both undefined and null
z.any() // From TypeScript
z.unknown() // From TypeScript
z.never() // No valuw will pass validation
```

## Convert to type

```
z.coerce.string(); // String(value)
z.coerce.number(); // Number (value)
z.coerce.boolean(); // Boolean(value)
z.coerce.bigint(); // BigInt (value)
z.coerce.date(); // new Date(value)
```

## Literal, exact value

```
z.literal("string");
z.literal(12);
z.literal(2n);
z.literal(true);
z.literal(["red", "gree"]); // To extract valid values use schema.values
```

## String

### Validation

```
z.string().max(5);
z.string().min(5);
z.string().length(5);
z.string().regex(/^[a-z]+$/);
z.string().startsWith("aaa");
z.string().endsWith("aaa");
z.string().includes("---");
z.string().uppercase();
z.string().lowercase();
z.stringbool();
// Default (case-insensitive match)
z.stringbool({
    truthy: ["true", "1", "yes", "on", "y", "enabled"],
    falsy: ["false", "0", "no", "off", "n", "disabled"],
});
```

### Transform methods

```
z.string().trim();
z.string().toLowerCase();
z.string().toUpperCase();
z.string().normalize(); // normalize unicode characters
```

### Common formats

```
z.email();
z.email({ pattern: z.regexes.email });
z.email({ pattern: z.regexes.html5Email }); // Used by browser
z.uuid();
z.uuid({ version: "v4" }); // v1, v2, v3, v4, v5, v6, v7, v8
z.guid();
z.url(); // Uses "new URL()"
z.url({ hostname: /^example\.com$/ });
z.url({ protocol: /^https$/ });
z.httpUrl(); // http or https only
z.hostname();
z.emoji();
z.base64();
z.base64url();
z.hex();
z.jwt();
z.jwt({ alg: "HS256" });
z.nanoid();
z.cuid();
z.cuid2();
z.ulid();
z.ipv4();
z.ipv6();
z.mac();
z.mac({ delimiter: "-" });
z.cidrv4(); // 192.168.0.0/24
z.cidrv6();
z.hash("sha256");
z.hash("md5");
z.hash("sha1");
z.hash("sha256");
z.hash("sha384");
z.hash("sha512");
z.hash("...", { enc: "hex" }); // Default
z.hash("...", { enc: "base64" });
z.hash("...", { enc: "base64url" });
z.iso.date();
z.iso.time();
z.iso.time({ precision: -1 }); // HH:MM
z.iso.time({ precision: 0 }); // HH:MM:SS
z.iso.time({ precision: 1 }); // HH:MM:SS.s
z.iso.time({ precision: 2 }); // HH:MM:SS.ss
z.iso.time({ precision: 3 }); // HH:MM:SS.sss
z.iso.datetime();
z.iso.datetime({ offset: true }); // To allow timezone offset
z.iso.datetime({ local: true }); // To allow local time
z.iso.datetime({ precision: -1 }); // Minute precision only (cannot pass second)
z.iso.datetime({ precision: 0 }); // Second precision only (cannot pass millisecond)
z.iso.datetime({ precision: 3 }); // Millisecond precision only
z.iso.duration();

// Create custom string format
z.stringFormat;
```

## Number formats

```
z.number().gt(5);
z.number().gte(5);
z.number().min(5);
z.number().lt(5);
z.number().lte(5);
z.number().max(5);
z.number().positive();
z.number().nonnegative();
z.number().negative();
z.number().nonpositive();
z.number().multipleOf(5);
z.number().step(5); // Same as multipleOf(5)
z.nan();
z.int();
z.int32();
```

## Date

```
z.date().min(new Date("1900-01-01"));
z.date().max(new Date());
```

## Enum

```
// Enum - fixed set of allowable string values
z.enum(['red', 'green', 'blue'])

const var = ['red', 'green', 'blue'] as const;
z.enum(var)

// Can pass object as well the values being string | number
cosnt Fish = {
Salmon: 0,
Tuna: 1
} as const;
z.enum(Fish) // Valid values Fish.Salmon, Fish.Tuna. 0 will not work
```

## Object

```
z.object({ ... }) // Unrecognized keys will be stripped
z.strictObject({ ... }) // An error is thrown with unknown keys
z.looseObject({ ... }) // Unrecognized keys allowed to pass through
z.object({ ... }).catchall(z.string()) // Schema for unrecognized keys
```

## Collections

```
z.array(z.string()) // Same as z.string().array()
z.array(z.string()).min(5)
z.array(z.string()).max(5)
z.array(z.string()).length(5)
z.tuple([ // Fixed length array
z.string(),
z.number(),
z.boolean(),
])
z.union([z.string(), z.number()])
z.discriminatorUnion('status', [ // Narrow type signature using TypeScript
z.object({ status: z.literal('success'), data: z.string() }),
z.object({ status: z.literal('failed'), error: z.string() }),
])
z.intersection(a, b) // Usefule for intersecting two object types
z.map(z.string(), z.number())
z.set(z.number())
z.set(z.number()).min(5)
z.set(z.number()).max(5)
z.set(z.number()).size(5)
```
