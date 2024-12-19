# Model coherence in a-mir-formality

| Metadata         |               |
|------------------|---------------|
| Point of contact | @nikomatsakis |
| Teams            | [Types]       |
| Status           | Proposed      |

## Summary

We will model coherence (including negative impls) in a-mir-formality and compare its behavior against rustc.
This will require extending a-mir-formality with the ability to run Rust tests.

## Motivation

Over the next six months we will test [a-mir-formality][]'s model of coherence against the new trait solver.
To do this, we will extend it with the ability to run (small, simple) Rust files as tests and then build a tool to compare its behavior against rustc.
This is working towards our ultimate goal for [a-mir-formality] being an "executable spec and playground" for the Rust type system.
There are a number of things that need to happen for that goal to be truly realized in practice but the biggest of them is to be able to compare behavior against rustc.

[a-mir-formality]: https://github.com/rust-lang/a-mir-formality

### The status quo

a-mir-formality has a sketch of a model of the Rust type system but tests must be written in a "Rust-like" dialect.
This dialect is great for precisely controlling the input but makes it impossible to compare mir-formality's behavior to rustc in any systematic way.

### The next 6 months

Our goal for the next 6 months is to use a-mir-formality to document and explore Rust's coherence system.
Towards this end we will do two major initiatives:

* Preparing an explainer that documents a-mir-formality's rules and reading it with the [types] team;
    * this will also involve changing and improving those rules
* Extending a-mir-formality with the ability to run (small, simple) unmodified Rust tests.

We will use the ability to run tests to compare the behavior of a-mir-formality against rustc, looking for discrepancies between the model and rustc's actual behavior.

### The "shiny future" we are working towards

We are working towards a future where

* a-mir-formality is regularly tested against a subset of the compiler's test suite;
* new features that impact the type system are modeled in a-mir-formality prior to stabilization (and perhaps prior to RFC);
* a-mir-formality is widely maintained by all members of the types team.

## Design axioms

The primary "axiom" in choosing this goal is that **there's nothing like the magic of running code** -- in other words, the best way to make the shiny future come true is going to be making it easy to write tests and play with a-mir-formality. Right now the barrier to entry is still too high. 

[da]: ../about/design_axioms.md

## Ownership and team asks

| Task                         | Owner(s) or team(s) | Notes |
|------------------------------|---------------------|-------|
| Discussion and moral support | ![Team][] [types]   |       |

### Modeling and documenting coherence rules

| Task                                 | Owner(s) or team(s) | Notes |
|--------------------------------------|---------------------|-------|
| Author explainer for coherence model | @nikomatsakis       |       |

### Running Rust tests in a-mir-formality

| Task           | Owner(s) or team(s) | Notes |
|----------------|---------------------|-------|
| Mentorship     | @nikomatsakis       |       |
| Implementation | ![Help wanted][]    |       |

### Stretch goal: modeling Rust borrow checker

As a stretch goal, we can extend a-mir-formality to model the bodies of functions and try to model the Rust borrow checker.

| Task           | Owner(s) or team(s) | Notes |
|----------------|---------------------|-------|
| Mentorship     | @nikomatsakis       |       |
| Implementation | ![Help wanted][]    |       |

### Definitions

Definitions for terms used above:

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

*This is a good place to elaborate on your reasoning above -- for example, why did you put the design axioms in the order that you did? It's also a good place to put the answers to any questions that come up during discussion. The expectation is that this FAQ section will grow as the goal is discussed and eventually should contain a complete summary of the points raised along the way.*