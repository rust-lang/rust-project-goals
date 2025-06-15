# Implement restrictions, prepare for stabilization

| Metadata           |                                    |
| :--                | :--                                |
| Point of contact   | @jhpratt                           |
| Teams              | <!-- TEAMS WITH ASKS -->           |
| Task owners        | <!-- TASK OWNERS -->               |
| Status             | Accepted                           |
| Zulip channel      | N/A                                |
| Tracking issue     | [rust-lang/rust-project-goals#257] |

[rfc]: https://rust-lang.github.io/rfcs/3323-restrictions.html
[pr]: https://github.com/rust-lang/rust/pull/106074

## Summary

[RFC 3323][rfc] will be implemented and feature-complete, with all syntax questions resolved. The
features will be prepared for stabilization.

## Motivation

The [RFC for restrictions][rfc] was accepted over two years ago, but the [pull request implementing
it][pr] has been stalled for a long time for a variety of reasons. Implementing the feature will
permit testing, feedback, and stabilization.

### The status quo

Sealed traits are a common pattern in Rust, but are not currently supported by the language itself.
Instead, they are implemented using a combination of visibility modifiers and nested modules. Fields
with restricted mutability are currently only possible with getters and setters, setting aside
(ab)using `Deref` implementations.

More details are available in [the RFC][rfc].

### The next 6 months

The [accepted restrictions RFC][rfc] represents the end goal of this project goal. All unresolved
questions should be discussed and resolved, with the two features (`impl_restrictions` and
`mut_restrictions`) being ready for stabilization. Future possibilities are likely considered at a
high level, but are not the focus of this project goal.

## Ownership and team asks

**Owner:** @jhpratt

| Task                                    | Owner(s) or team(s)  | Notes                                      |
|-----------------------------------------|----------------------|--------------------------------------------|
| Discussion and moral support            | ![Team][] [lang]     |                                            |
| Implementation                          | @jhpratt             | [old PR][pr] is plausibly workable         |
| Standard reviews                        | ![Team][] [compiler] |                                            |
| Author stabilization report             | @jhpratt             |                                            |
| Author specification 1st draft          | @jhpratt             |                                            |
| Finalize specification text             | ![Team][] [spec]     | @joelmarcey                               |
| Stabilization decision                  | ![Team][] [lang]     |                                            |
| Inside Rust blog post inviting feedback | @jhpratt             | feedback on syntax if no team consensus    |

### Definitions

Definitions for terms used above:

- _Discussion and moral support_ is the lowest level offering, basically committing the team to nothing but good vibes and general support for this endeavor.
- _Author RFC_ and _Implementation_ means actually writing the code, document, whatever.
- _Design meeting_ means holding a synchronous meeting to review a proposal and provide feedback (no decision expected).
- _RFC decisions_ means reviewing an RFC and deciding whether to accept.
- _Org decisions_ means reaching a decision on an organizational or policy matter.
- _Secondary review_ of an RFC means that the team is "tangentially" involved in the RFC and should be expected to briefly review.
- _Stabilizations_ means reviewing a stabilization and report and deciding whether to stabilize.
- _Standard reviews_ refers to reviews for PRs against the repository; these PRs are not expected to be unduly large or complicated.
- _Prioritized nominations_ refers to prioritized lang-team response to nominated issues, with the expectation that there will be _some_ response from the next weekly triage meeting.
- _Dedicated review_ means identifying an individual (or group of individuals) who will review the changes, as they're expected to require significant context.
- Other kinds of decisions:
  - [Lang team experiments](https://lang-team.rust-lang.org/how_to/experiment.html) are used to add nightly features that do not yet have an RFC. They are limited to trusted contributors and are used to resolve design details such that an RFC can be written.
  - Compiler [Major Change Proposal (MCP)](https://forge.rust-lang.org/compiler/mcp.html) is used to propose a 'larger than average' change and get feedback from the compiler team.
  - Library [API Change Proposal (ACP)](https://std-dev-guide.rust-lang.org/development/feature-lifecycle.html) describes a change to the standard library.

## Frequently asked questions

### Isn't the syntax already decided?

While the RFC was accepted without this being an unresolved question (aside from a simpler syntax
for common cases), I believe that an attribute-based syntax such as `#[restrict(impl(crate))]` may
be, but is not necessarily, favorable to the syntax in the RFC. This is because it is
backwards-compatible with all existing macros and prevents nontrivial additions to the width of
code.