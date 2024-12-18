# Declarative (`macro_rules!`) macro improvements

| Metadata |               |
|----------|---------------|
| Owner(s) | @joshtriplett |
| Teams    | [lang], [wg-macros]        |
| Status   | Proposed      |

## Summary

In this project goal, I'll propose and shepherd Rust language RFCs to make
`macro_rules!` macros just as capable as proc macros, and to make such macros
easier to write. I'll also start prototyping extensions to the declarative
macro system to make macros easier to write, with the aim of discussing and
reaching consensus on those additional proposals during RustWeek (May 2025) at
the latest. Finally, I'll write a series of Inside Rust blog posts on these
features, to encourage crate authors to try them and provide feedback, and to
plan transitions within the ecosystem.

The scope of this goal is an arc of many related RFCs that tell a complete
story, as well as the implementation of the first few steps.

## Motivation

This project goal will make it possible, and straightforward, to write any type
of macro using the declarative `macro_rules!` system. This will make many Rust
projects build substantially faster, make macros simpler to write and
understand, and reduce the dependency supply chain of most crates.

### The status quo

There are currently several capabilities that you can *only* get with a proc
macro: defining an attribute macro that you can invoke with `#[mymacro]`, or
defining a derive macro that you can invoke with `#[derive(MyTrait)]`. In
addition, even without the requirement to do so (e.g. using workarounds such as
the [`macro_rules_attribute`](https://crates.io/crates/macro_rules_attribute)
crate), macro authors often reach for proc macros anyway, in order to write
simpler procedural code rather than refactoring it into a declarative form.

Proc macros are complex to build, have to be built as a separate crate that
needs to be kept in sync with your main crate, add a heavy dependency chain
(`syn`/`quote`/`proc-macro2`) to projects using them, add to build time, and
lack some features of declarative (`macro_rules!`) macros such as `$crate`.

As a result, proc macros contribute to the perceptions that Rust is complex,
has large dependency supply chains, and takes a long time to build. Crate
authors sometimes push back on (or feature-gate) capabilities that require proc
macros if their crate doesn't yet have a dependency on any, to avoid increasing
their dependencies.

### The next 6 months

Over the next 6 months, I'll propose RFCs to improve the current state of
declarative (`macro_rules!`) macros, and work with @eholk and @vincenzopalazzo
to get those RFCs implemented. Those RFCs together will enable:

- Using `macro_rules!` to define attribute macros (`#[attr]`)
- Using `macro_rules!` to define derive macros (`#[derive(Trait)]`)
- Using `macro_rules!` to define unsafe attributes and unsafe derive macros.

I also have an RFC in progress ("macro fragment fields") to allow
`macro_rules!` macros to better leverage the Rust parser for complex
constructs. Over the next 6 months, I'll shepherd and refine that RFC, and
design extensions of it to help parse additional constructs. (I expect this RFC
to potentially require an additional design discussion before acceptance.) The
goal will be to have enough capabilities to simplify many common cases of
attribute macros and derive macros.

I'll propose initial prototypes of additional macro metavariable expressions to
make `macro_rules!` easier to write, such as by handling multiple cases or
iterating without having to recurse. This provides one of the key
simplification benefits of proc macros, with minimal added complexity in the
language. I expect these to reach pre-RFC form and be suitable for discussion
at RustWeek in May 2025, and hopefully reach consensus, but I do not expect
them to be fully accepted or shipped in the next 6 months.

In addition, as part of this goal, I intend to work with @eholk and
@vincenzopalazzo to revitalize the wg-macros team, and evaluate potential
policies and delegations from [lang], in a similar spirit to wg-const-eval,
t-types, and t-opsem.

Much as with the const eval system, I expect this to be a long incremental
road, with regular improvements to capabilities and simplicity. Crate authors
can adopt new features as they arise, and transition from proc macros to
declarative macros once they observe sufficient parity to support such a
switch.

### The "shiny future" we are working towards

In the shiny future of Rust, the vast majority of crates don't need to use proc
macros. They can easily implement attributes, derives, and complex macros using
exclusively the declarative `macro_rules!` system.

Furthermore, crate authors will not feel compelled to use proc macros for
simplicity, and will not have to contort their procedural logic in order to
express it as a declarative macro macro. Crate authors will be able to write
macros using `macro_rules!` in either a recursive or semi-procedural style. For
instance, this could include constructs like `for` and `match`.

I expect that all of these will be available to macros written in any edition,
though I also anticipate the possibility of syntax improvements unlocked by
future editions or within future macro constructs. For instance, currently Rust
macros do not reserve syntax like `$keyword` (e.g. `$for`). Existing editions
could require the `${...}` macro metavariable syntax to introduce new
constructs. Rust 2027 could reserve `$keyword`, and new syntax like `macro`
could reserve such syntax in all editions.

## Design axioms

- Incremental improvements are often preferable to a ground-up rewrite. The
  ecosystem can adopt incremental improvements incrementally, and give feedback
  that inspires further incremental improvements.
- There should never be a capability that *requires* using a proc macro.
- The most obvious and simplest way to write a macro should handle all cases a
  user might expect to be able to write. Where possible, macros should
  automatically support new syntax variations of existing constructs, without
  requiring an update.
- Macros should not have to recreate the Rust parser (or depend on crates that
  do so). Macros should be able to reuse the compiler's parser. Macros
  shouldn't have to parse an entire construct in order to extract one component
  of it.
- Transforming iteration or matching into recursion is generally possible, but
  can sometimes obfuscate logic.

## Ownership and team asks

**Owner / Responsible Reporting Party:** @joshtriplett


| Task                                             | Owner(s) or team(s) | Notes                                                                                      |
|--------------------------------------------------|---------------------|--------------------------------------------------------------------------------------------|
| Propose discussion session at RustWeek           | @joshtriplett       |                                                                                            |
| Policy decision                                  | ![Team][] [lang] [wg-macros] | Discussed with @eholk and @vincenzopalazzo; lang would decide whether to delegate specific matters to wg-macros |

### `macro_rules!` attributes

| Task                                   | Owner(s) or team(s) | Notes |
|----------------------------------------|---------------------|-------|
| Author/revise/iterate RFCs             | @joshtriplett       |       |
| Prioritized nominations                | ![Team][] [lang]    |       |
| RFC decision                           | ![Team][] [lang]    |       |
| Implementation of RFC                  | @eholk, @vincenzopalazzo |  |
| Iterate on design as needed            | @joshtriplett       |       |
| Inside Rust blog post on attribute macros | @joshtriplett    |       |
| Process feedback from crate authors    | @joshtriplett       |       |
| Author stabilization report (if ready) | @joshtriplett       |       |
| Stabilization decision                 | ![Team][] [lang]    |       |

### `macro_rules!` derives

| Task                                   | Owner(s) or team(s) | Notes |
|----------------------------------------|---------------------|-------|
| Author/revise/iterate RFCs             | @joshtriplett       |       |
| Prioritized nominations                | ![Team][] [lang]    |       |
| RFC decision                           | ![Team][] [lang]    |       |
| Implementation of RFC                  | @eholk, @vincenzopalazzo |  |
| Iterate on design as needed            | @joshtriplett       |       |
| Inside Rust blog post on derive macros | @joshtriplett       |       |
| Process feedback from crate authors    | @joshtriplett       |       |
| Author stabilization report (if ready) | @joshtriplett       |       |
| Stabilization decision                 | ![Team][] [lang]    |       |

### Design and iteration for macro fragment fields

| Task                                          | Owner(s) or team(s)          | Notes |
|-----------------------------------------------|------------------------------|-------|
| Author initial RFC                            | @joshtriplett                |       |
| Design meeting                                | ![Team][] [lang]             |       |
| RFC decision                                  | ![Team][] [lang]             |       |
| Implementation of RFC                         | @eholk, @vincenzopalazzo     |       |
| Iterate on design as needed                   | @joshtriplett                |       |
| Inside Rust blog post on additional capabilities | @joshtriplett             |       |
| Process feedback from crate authors           | @joshtriplett                |       |
| Author stabilization report (if ready)        | @joshtriplett                |       |
| Stabilization decision                        | ![Team][] [lang]             |       |
| Support lang experiments for fragment fields  | @joshtriplett                |       |
| Author small RFCs for further fragment fields | @joshtriplett                |       |

### Design for macro metavariable constructs

| Task                            | Owner(s) or team(s)          | Notes |
|---------------------------------|------------------------------|-------|
| Design research and discussions | @joshtriplett                |       |
| Discussion and moral support    | ![Team][] [lang] [wg-macros] |       |
| Author initial RFC              | @joshtriplett                |       |

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

### What about "macros 2.0"

Whenever anyone proposes a non-trivial extension to macros, the question always
arises of how it interacts with "macros 2.0", or whether it should wait for
"macros 2.0".

"Macros 2.0" has come to refer to a few different things, ambiguously:

- Potential future extensions to declarative macros to improve
  hygiene/namespace handling.
- An experimental marco system using the keyword `macro` that partially
  implements hygiene improvements and experimental alternate syntax, which
  doesn't have a champion or a path to stabilization, and hasn't seen active
  development in a long time.
- A catch-all for hypothetical future macro improvements, with unbounded
  potential for scope creep.

As a result, the possibility of "macros 2.0" has contributed substantially to
"stop energy" around improvements to macros.

This project goal takes the position that "macros 2.0" is sufficiently nebulous
and unfinished that it should not block making improvements to the macro
system. Improvements to macro hygiene should occur incrementally, and should
not block other improvements.

### Could we support proc macros without a separate crate, instead?

According to reports from compiler experts, this would be theoretically
possible but incredibly difficult, and is unlikely to happen any time soon. We
shouldn't block on it.

In addition, this would not solve the problem of requiring proc macros to
recreate the Rust parser (or depend on such a reimplementation).

### What about a "comptime" system?

This would likewise be possible in the future, but we shouldn't block on it.
And as above, this would not solve the problem of requiring such a system to
recreate the Rust parser. We would still need a design for allowing such
comptime functions to walk the Rust AST in a forward-compatible way.
