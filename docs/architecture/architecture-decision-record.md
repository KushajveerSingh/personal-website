---
title: Architecture Decision Records (ADR)
parent: Architecture
nav_order: 1
---

<!-- prettier-ignore-start -->
# Architecture Decision Records (ADR)
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

-   [link](https://alfy.blog/2021/01/01/adrs.html) Architecture Decision Records by Ahmad Alfy
-   [link](https://github.com/joelparkerhenderson/architecture-decision-record) Architecture Decision Record GitHub
-   [link](https://github.com/npryce/adr-tools) adr-tools GitHub

---

## Architecture Knowledge Management

-   **Architecture decision (AD)** - Software design choice that addresses a significant requirement.
-   **Architecturally-Significant Requirement (ASR)** - Requirement that has a measurable effect on a software system's architecture.
-   **Architecture Decision Record (ADR)** - Document that captures an important architecture decision made along with its context and consequences.
-   **Architecture Decision Log (ADL)** - Collection of all ADRs created and maintained for a particular project or organization.
-   **Architecture Knowledge Management (AKM)** - Everything above falls under AKM.

## ADR

Markdown file for every _architecturally significant_ decision. Include the following

-   **The decision** we need to take.
-   **Status** of that decision whether it's a proposal, accepted decision, rejected, or superseded.
-   **Context**, which is an explanantion of the problem and all the circumstances around it.
-   **Considered options**. Listing all the considered options with the pros, cons, and impact of each.
-   **The decision** that will be taken and all the reasons favoring that decision.
-   **The consequences** of taking that decision whether it's positive or not.

## Example

```
# Choosing Pre/Post processor for project

## Status

Accepted

## Context

## Considerd options

## Decision

## Consequences

### Positive consequences

### Nesgative consequences
```

## Organizing ADRs

-   Store all the records under `doc/adr` folder.
-   Number the ADRs like `0001-record-architecture-decisions.md`.
    -   Use present tense imperative verb phrase like choose-database, format-timestamps, manage-passwords, handle-exceptions.
-   Identify when each item in the ADR is written using timestamps.
-   Don't alter existing information in an ADR. Instead amend the ADR by adding new information, or supersede the ADR by creating a new ADR.
-   Make this as the first ADR.

    ```
    # 1. Record architecture decisions

    Date: 2016-02-12

    ## Status

    Accepted

    ## Context

    We need to record the architectural decisions made on this project.

    ## Decision

    We will use Architecture Decision Records, as described by Michael Nygard in this article: http://thinkrelevance.com/blog/2011/11/15/documenting-architecture-decisions

    ## Consequences

    See Michael Nygard's article, linked above.
    ```

-   For ADR that supercede's previous ADR, adjust the status accordingly.

    For old ADR

    ```
    ## Status

    Accepted

    Amended by [9. Help scripts](0009-help-scripts.md)
    ```

    For new ADR

    ```
    ## Status

    Accepted

    Amends [5. Help comments](0005-help-comments.md)
    ```
