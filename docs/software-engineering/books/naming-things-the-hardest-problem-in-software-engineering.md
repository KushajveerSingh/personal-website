---
title: "Naming Things: The Hardest Problem In Software Engineering"
parent: Books
grand_parent: Software Engineering
---

<!-- prettier-ignore-start -->
# Naming Things: The Hardest Problem In Software Engineering
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

- [link](https://www.amazon.com/Pragmatic-Programmer-journey-mastery-Anniversary/dp/0135957052) Naming Things: The Hardest Problem In Software Engineering by _Tom Benner_ (106 pages) (Feb 1, 2023)

---

## Why is naming important?

- Code is for a human first and a computer second.
- Professional engineers pay more attention to the name of the identifiers than source code comments.
- Good naming -> Understandable code -> Efficient maintenance
- Helps new team members understand the codebase.
- To write good names, an engineer must understand the domain and associated terminology. Domain expertise leads to leadership roles and career growth.

Good naming helps with

- _Comprehension_ - The ability of software engineer to **understand** the logic, structure, and purpose of the code.
- _Recall_ - The ability of a software engineer to **remember** the logic, structure, and purpose of the code.
- _Defect resolution time_ - The duration between when a software engineer starts working on a defect and when they resolve the defect.

## Why is naming hard?

- Dynamic, subjective requirements
    - The concept being named is dynamic, and often evolves over time. Also, the concept and its name is dependent on the domain, which can also change over time.
    - The audience reading the name is dyanmic. We can guess the audience at the time of naming, but the audience may change over time.
    - The way name's correctness is judged is subjective. Understandability and conciseness are determined by the audience.
- Insufficient best practices and tooling
    - There is no industry-wide standard.
    - Lack of tooling for enforcing high-quality naming.
- Short-term costs and long-term value
    - Costs of good naming are mostly immediate and its value is mostly long-term.

## Principle 1: Understandability

A name should describe the concept it represents.

How to

- If the reader is familiar with the concept, the name should be familiar to them.
- If the reader is unfamiliar with the concept, then the concept's name should teach them the correct terminology for the concept within the domain.

### Describe the concept the name represents

```
Bad - foo, thing, scooter
Good - bicycle
```

### Use common dictionary terms

```
Bad - o, org, people_group
Good - organization
```

### Use problem-domain terms instead of solution-domain terms

- Problem domain - Information that defines a problem (requirements, goals). These are more stable.
- Solution domain - Solution's design and implementation

```
Problem - Schedule meethings
Bad - schedule_events(events)
Good - schedule_meetings(meetings)
```

### Use standard terms for domain-agnostic concepts

Standardized terminology exists for

- Universal concepts - time, physical distance.
- Technical concepts - networking, operating systems, database.

```
Bad - Database.remove_record(table, primary_key)
Good - Datbase.delete_row(table, primary _key)

Bad - Process.stop(process_id, signal_name)
Good - Process.kill(pid, signal_name)
```

### Use correct pluralization

```
Bad - users = User.where(id=user_id)[0]
Good - user = User.where(id=user_id)[0]

Bad - validate_user(users)
Good - validate_users(users)
```

### Use correct parts of speech (noun, verb, adjective)

- Classes - Noun or noun phrase (User, PaymentMethod)
- Variables - Noun, noun phrase, linking verb, subject cmplement (is_valid)
- Method - Verbs, subject complement (validate, delete_all, is_valid)
- Interfaces - Noun, adjectives (Parser, DataInput, Runnable)

Linking verb - use for booleans like "is" (is_valid). "valid" is the subject complement.

### Include units in measurement

The only exception to this rule is when the codebase consitently uses seconds everywhere or USD for currency.

```
Bad - elapsed_duration
Good - elapsed_duration_in_days

Bad - remaining_distance
Good - remaining_distance_in_meters
```

### Avoid unconventional single-letter named

The exception to this rule is for known conventions like i/j for loop indexes, p/q for pointers.

```
Bad - u = users[0]
Good - user = users[0]

Bad - l = users.length
Good - user_count = users.length
```

### Avoid abbrevations

The exception to this rule are widely used abbreviations like URL, DNS, ID.

```
Bad - ap, acts_payable
Good - accounts_payable

Bad - org
Good - organization
```

### Avoid non-standard symbolic names (math symbols, emojis)

```
Bad - def ->(&block)
Good - def map(&block)
```

### Avoid cleverness, humor

```
Problem - Write function to remove bullets from list
Bad - apply_kevlar(text)
Good - remove_bullets(text)
```

### Avoid usage of temporary or irrelevant concepts like movie references

```
Bad - kell_em_all(processes)
Good - kill_processes(processes)
```

### Consider the audience's familiarity with the name

If the audience of the code is not familiar with the domain, then avoid industry-specific terms. Do consider the fact that using industry-specific terms in this scenario can actually help the audience learn about the domain.

## Principle 2: Conciseness

A name should use only the words necessary to communicate the concept it represents. Long names require more time to consider, process and understand. The recall decreases as the number of syllables increase due to over-crowding of short-term memory.

### Use the appropriate level of abstraction (do not use implementation details)

You can introduce yourself as

- Hi, I'm a human (relevant if you are meeting an alien)
- Hi, I'm Jane (relevant if you are meeting friends)
- Hi, I'm Jane Smith, daughter os Sarah Smith (relevant if you are meeting relatives)

Do not use a name that is so specific that you are providing irrelevant information. Do not use a name that's so generic that it provides no relevant information.

Use a name that describes the intent of the concept, what it is meant to be used for, and not how it is implemented.

### Use words with rich meaning (use problem-domain terms)

```
Bad - SongCollection
Good - Album
```

### Omit metadata (type, scope)

```
Bad - first_name_string
Good - first_name

Bad - person_list, people_array
Good - people
```

The exception to the rule is when you need the type to specify the context.

```
location_text = 'Chicago'
user.location = Location.geocode(location_text)
```

### Omit implementation details, use arguments when needed

This mostly occurs in method implementation.

```
Bad - csv_processor.process_in_parallel(rows)
Good - csv_processor.process(rows)
```

If you need to support both serial and parallel processing, then use an argument to provide the configuration.

```
csv_processor.process(rows, in_parallel=True)
csv_processor.process(rows, in_parallel=False)
```

### Omit unnecessary words

A normal person would assume deletion would happen instantly, so no need to specify "now".

```
Bad - user.delete_now()
Good - user.delete()
```

```
Bad - User.delete_user(user)
Good - User.delete(user)
```

### Booleans should default to true

Do not use "not" (not_prepared) or a negative prefix (invalid) when defining booleans.

```
Bad - if !user.is_invalid() {}
Good - if user.is_valid() {}
```

## Principle 3: Consitency

Names should be used and formatted uniformly.

## Principle 4: Distinguishability
