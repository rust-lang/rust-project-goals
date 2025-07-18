# Continue Experimentation with Pin Ergonomics

| Metadata         |                                                                                  |
|:-----------------|----------------------------------------------------------------------------------|
| Point of contact | @frank-king                                                                      |
| Teams            | &lt;!-- TEAMS WITH ASKS --&gt;                                                   |
| Teams            | <!-- TEAMS WITH ASKS -->                                                         |
| Status           | Proposed                                                                         |
| Tracking issue   |      |
| Zulip channel    | N/A (an existing stream can be re-used or new streams can be created on request) |

## Summary

Continue experimenting with and fleshing out the design and semantics for the pin ergonomics experiment.

## Motivation

Several flagship Rust features, such as `async` and generators, depend on the ability to ensure data will not move in memory.
The way Rust achieves this is through the `Pin` wrapper, but pinning has notoriously poor ergonomics.
Exploring how the ergonomics can be improved by integrating pinning into the language better could help unblock advancements in other Rust features.

### The status quo

Pinning exists but few people like it.
We have an experiment for improving the ergonomics around pinning and some initial PRs have landed, but we would like to build more sustained momentum on it.

### The next 6 months

- introduce `&pin mut|const place` borrowing syntax (parsing #135731 ready to merge, lowering and  borrowck not started yet)
- pattern matching of `&pin mut|const T` types (#139751 under review)
- introduce `&pin pat` pattern syntax
- support `&pin mut|const T` -> `&|&mut T` coercion (requires `T: Unpin` of `&pin mut T` -> `&mut T`)
- support auto borrowing of `&pin mut|const place` in method calls with `&pin mut|const self` receivers

### The "shiny future" we are working towards


## Design axioms


## Ownership and team asks


| Task                         | Owner(s) or team(s) | Notes |
|------------------------------|---------------------|-------|
| Discussion and moral support | ![Team][] [lang]    |       |
| Do the work                  | @frank-king         |       |

### Design language feature to solve problem X

> *Some goals propose to design a feature to solve a problem. Typically the outcome from this goal is an draft or accepted RFC. If you would like to work on an experimental implementation in-tree before an RFC is accepted, you can create a [lang team experiment](https://lang-team.rust-lang.org/how_to/experiment.html), but note that a trusted contributor is required.*

| Task                 | Owner(s) or team(s)                | Notes                                                               |
|----------------------|------------------------------------|---------------------------------------------------------------------|
| Lang-team experiment | ![Team][] [lang]                   | allows coding pre-RFC; only for trusted contributors                |
| Author RFC           | *Goal point of contact, typically* |                                                                     |
| Lang-team champion   | ![Team][] [lang]                   | Username here |
| RFC decision         | ![Team][] [lang]                   |                                                                     |
| RFC secondary review | ![Team][] [types]                  | request bandwidth from a second team, most features don't need this |

### Implement language feature X

> *If there is an accepted RFC, or you are doing a [lang-team experiment](https://lang-team.rust-lang.org/how_to/experiment.html), you commonly need someone to write the code, support from the compiler to review your PRs, and possibly lang-team design meetings to review interesting design questions. Once implementation completes we recommend a call for testing blog post.*

| Task                              | Owner(s) or team(s)                | Notes |
|-----------------------------------|------------------------------------|-------|
| Implementation                    | *Goal point of contact, typically* |       |
| Standard reviews                  | ![Team][] [compiler]               |       |
| Lang-team champion                | ![Team][] [lang]                   |       |
| Design meeting                    | ![Team][] [lang]                   |       |
| Author call for testing blog post | *Goal point of contact, typically* |       |

### Stabilize language feature X

> *If the feature has been RFC'd and implemented and experiences are positive, [stabilization](https://rustc-dev-guide.rust-lang.org/stabilization_guide.html) may be the right next step. In this case, you will need to author a first draft of text for the Rust reference and make a Team Ask to request someone from the the spec team to adapt that text for final inclusion. You will also need to author a stabilization report.

| Task                           | Owner(s) or team(s)                | Notes |
|--------------------------------|------------------------------------|-------|
| Author specification 1st draft | *Goal point of contact, typically* |       |
| Finalize specification text    | ![Team][] [spec]                   |       |
| Lang-team champion             | ![Team][] [lang]                   |       |
| Author stabilization report    | *Goal point of contact, typically* |       |
| Author stabilization PR        | *Goal point of contact, typically* |       |
| Stabilization decision         | ![Team][] [lang]                   |       |

### Stabilize library feature

> *Standard library features follow the [libs-api stabilization process](https://rustc-dev-guide.rust-lang.org/stability.html#stabilizing-a-library-feature). 

| Task                           | Owner(s) or team(s)                | Notes |
|--------------------------------|------------------------------------|-------|
| Author stabilization PR        | *Goal point of contact, typically* |       |
| Stabilization decision         | ![Team][] [libs-api]               |       |

### Definitions

For definitions for terms used above, see the [About > Team Asks](https://rust-lang.github.io/rust-project-goals/about/team_asks.html) page.

* *Discussion and moral support* is the lowest level offering, basically committing the team to nothing but good vibes and general support for this endeavor.
* *Author RFC* and *Implementation* means actually writing the code, document, whatever.
* *Design meeting* means holding a synchronous meeting to review a proposal and provide feedback (no decision expected).
* *RFC decisions* means reviewing an RFC and deciding whether to accept.
* *Org decisions* means reaching a decision on an organizational or policy matter.
* *Secondary review* of an RFC means that the team is "tangentially" involved in the RFC and should be expected to briefly review.
* *Stabilizations* means reviewing a stabilization and report and deciding whether to stabilize.
* *Standard reviews* refers to reviews for PRs against the repository; these PRs are not expected to be unduly large or complicated.
* *Prioritized nominations* refers to prioritized lang-team response to nominated issues, with the expectation that there will be *some* response from the next weekly triage meeting.
* *Dedicated review* means identifying an individual (or group of individuals) who will review the changes, as they're expected to require significant context.
* Other kinds of decisions:
    * [Lang team experiments](https://lang-team.rust-lang.org/how_to/experiment.html) are used to add nightly features that do not yet have an RFC. They are limited to trusted contributors and are used to resolve design details such that an RFC can be written.
    * Compiler [Major Change Proposal (MCP)](https://forge.rust-lang.org/compiler/mcp.html) is used to propose a 'larger than average' change and get feedback from the compiler team.
    * Library [API Change Proposal (ACP)](https://std-dev-guide.rust-lang.org/development/feature-lifecycle.html) describes a change to the standard library.

## Frequently asked questions

### What do I do with this space?
