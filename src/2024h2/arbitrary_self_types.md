# Arbitrary self types

| Metadata | |
| --- | --- |
| Owner(s) | [Adrian Taylor][] |
| Teams | [Compiler], [Libs] |
| Status | WIP |

[Compiler]: https://www.rust-lang.org/governance/teams/compiler
[Libs]: https://www.rust-lang.org/governance/teams/library
[Alice Ryhl]: https://github.com/Darksonn/

## Motivation

The goal for 2024H2 is to stabilize support for Arbitrary Self Types as described in [RFC 3519][]. This will enable a number of use-cases, including C++ interop (as described in [RFC 3519][]) and the [ARC][] type from the Rust for Linux project.

[RFC 3519]: https://github.com/rust-lang/rfcs/pull/3519
[ARC]: https://rust-for-linux.com/arc-in-the-linux-kernel

### The status quo

The compiler permits self types for specific "smart pointers" (`Box`, `Rc`, etc) but it is not possible for users to define their own. There has been extensive discussion and design work culminating in the approval [RFC 3519][] with a specific proposal.

### The next few steps

This year the goal is to implement and stabilize the design from [RFC 3519][].

### The "shiny future" we are working towards

The ultimate goal is that users are able to define their own smart pointer types with equal capabilities to the builtin options in the standard library. Related and possible future work includes:

* [Supporting unsizing](./rfl_arc_unsizing.md)
* Ability to move out, as one can do with `Box`
* Integration with matching (e.g., [deref patterns][])

[deref patterns]: https://github.com/rust-lang/lang-team/blob/master/projects/deref-patterns.md

## Design axioms

None authored.

## Ownership and other resources

**Owner:** [Adrian Taylor][]

[Adrian Taylor]: https://github.com/adetaylor

### Support needed from the project

* Reviews from the [compiler] and/or [libs] teams
    * QUESTION: Should we assign a reviewer?
* Lang team design is considered "done", but occasional design meetings or triage discussion may be required

## Outputs and milestones

### Outputs

Stable version of the Arbitrary Self Types feature.

### Milestones

* Nightly implementation
* Stabilization report
* Stabilization PR

## Frequently asked questions

None.