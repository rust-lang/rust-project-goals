# Prepare const traits for stabilization

| Metadata         |                                    |
|:-----------------|------------------------------------|
| Point of contact | @oli-obk                           |
| Teams            | <!-- TEAMS WITH ASKS -->           |
| Task owners      | <!-- TASK OWNERS -->               |
| Status           | Proposed                           |
| Tracking issue   | [rust-lang/rust-project-goals#106] |
| Zulip channel    | N/A                                |


## Summary

Prepare `const Trait` bounds for stabilization.

## Motivation

`const fn` on stable are unable to invoke trait methods, limiting their usefulness. After years of experimentation, the compiler now has a promising implementation of `const traits` and key parts of the stdlib have been updated to use it. However, the feature is still firmly in experimental territory: there has never been an RFC describing its syntax.

The goal for the next 6 months is to author an RFC proposing an end-user syntax for const bounds and a specific semantics. Our believe is that the core implementation work is done and that it should be possible to merge the RFC, implement the syntactic choices, issue a public call for experimentation, and otherwise pave the ground for stabilization (stabilization itself is a stretch goal and seems likely to occur in 2025H2).

### The status quo

People write a lot of code that will be run in compile time. They include procedural macros, build scripts ([42.8k hits][build scripts] on GitHub for `build.rs`), and const functions/consts ([108k hits][const fns] on GitHub for `const fn`). Not being able to write const functions with generic behavior is often cited as a pain point of Rust's compile time capabilities. Because of the limited expressiveness of `const fn`, people may decide to move some compile time logic to a build script, which could increase build times, or simply choose not to do it in compile time (even though it would have helped runtime performance).

There are also language features that require the use of traits, such as iterating with `for` and handling errors with `?`. Because the `Iterator` and `Try` traits currently cannot be used in constant contexts, people are unable to use `?` to handle results, nor use iterators e.g. `for x in 0..5`.

[build scripts]: https://github.com/search?q=path%3A**%2Fbuild.rs+NOT+is%3Afork&type=code
[const fns]: https://github.com/search?q=%22const+fn%22+language%3Arust+NOT+is%3Afork&type=code

### The next 6 months

The primary goal is to do "Everything but" stabilization of this feature over the next 6 months

* Author an RFC for `const traits` and get it accepted by the language design team
* Implement the syntax and user experience described in the RFC
* Issue a call for experimentation and resolve any issues found
* Author a stabilization report

As a secondary goal, we will use this as an experiment to drive forward a-mir-formality, with @nikomatsakis and @oli-obk mentoring @tiif to extend a-mir-formality with support for const traits. 

### The "shiny future" we are working towards

We're working towards enabling developers to do more things in general within a `const` context. Const traits is a blocker for many future possibilities (see also the const eval [feature skill tree]) including heap operations in const contexts. 

[feature skill tree]: https://rust-lang.github.io/const-eval/skill_tree.html

## Design axioms

None.

## Ownership and team asks

Steps towards the primary goal of doing everything towards stabilization apart from approving stabilization itself:

| Task                               | Owner(s) or team(s)            | Notes                                                           |
|------------------------------------|--------------------------------|-----------------------------------------------------------------|
| Lang-team experiment               | ![Team][] [lang]               | ![Complete][]                                                   |
| Experimental implementation        | @fee1-dead, @compiler-errors   | ![Complete][]                                                   |
| Author RFC                         | @oli-obk                       | ![Complete][]                                                   |
| Adjust implementation to match RFC | rust-lang/project-const-traits |                                                                 |
| Call for testing                   | @oli-obk                       |                                                                 |
| Standard reviews                   | ![Team][] [compiler]           |                                                                 |
| Design meeting                     | ![Team][] [lang]               | first meeting scheduled for Jan; second meeting may be required |
| RFC decision                       | ![Team][] [lang]               |                                                                 |
| RFC secondary review                   | ![Team][] [types]              | Types team needs to validate the approach                       |
| Author stabilization report        | @oli-obk                       | stretch goal                                                    |

### Formalize const-traits in a-mir-formality

Steps towards formalization of the approach in a-mir-formality:

| Task                         | Owner(s) or team(s)     | Notes                                                                       |
|------------------------------|-------------------------|-----------------------------------------------------------------------------|
| Discussion and moral support | ![Team][] [types]       | During types team office hours, we'll share information about our progress. |
| Implementation               | @tiif                   |                                                                             |
| Mentoring and kibbitzing     | @nikomatsakis, @oli-obk |                                                                             |

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

### Will we be stabilizing the syntax found on libstd today?

Most likely not. The current syntax includes some controversial notation, such as `T: ~Trait`. The point of the RFC is to determine what syntax will be used. What we hope will not change is the *semantics*.

