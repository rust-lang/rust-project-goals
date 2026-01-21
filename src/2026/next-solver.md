# Stabilize the next-generation trait solver

| Metadata         |                                           |
| :--------------- | ----------------------------------------- |
| Point of contact | @lcnr                                     |
| Status           | Proposed                                  |
| Tracking issue   | [rust-lang/rust-project-goals#113]        |
| Zulip channel    | [#t-types/trait-system-refactor][channel] |
| [types] champion | @lcnr                                     |
| [lang] champion  | @nikomatsakis                             |

[channel]: https://rust-lang.zulipchat.com/#narrow/channel/364551-t-types.2Ftrait-system-refactor

## Summary

Stabilize `-Znext-solver=globally`, replacing the existing trait solver implementation entirely.

## Motivation

### The status quo

The next-generation trait solver is intended to fully replace the existing type system components responsible for proving trait bounds, normalizing associated types, and much more. This fixes many long-standing (soundness) bugs, enables future type system improvements, and improves compile-times. Its tracking issue is [rust-lang/rust#107374](https://github.com/rust-lang/rust/issues/107374).

The new solver has made substantial progress:

- **Coherence checking stabilized**: `-Znext-solver=coherence` shipped in Rust 1.84
- **rust-analyzer migrated**: The new solver replaced Chalk entirely in rust-analyzer
- **Crater regressions resolved**: Most regressions in the top 10,000 crates have been triaged and fixed
- **Performance improved**: From severe regressions to ~30-50% slower on typical crates

The remaining work is focused on resolving final blockers, achieving performance parity, and preparing the stabilization report.

### What we propose to do about it

Stabilize `-Znext-solver=globally` by:

- Resolving remaining issues tracked in the [project board](https://github.com/orgs/rust-lang/projects/61/views/1)
- Achieving performance parity with the old solver
- Specifying and stabilizing the cycle semantics (the new solver implements the desired behavior, but this needs to be documented via RFC)
- Completing the stabilization report and documentation

With the new solver stabilized, all remaining type system unsoundnesses can be fixed, the old solver can be removed (significantly simplifying the type system), and future improvements like coinductive semantics and where-bounds on binders become unblocked.

### Work items over the next year

| Task                                | Owner(s)                          | Notes |
| ----------------------------------- | --------------------------------- | ----- |
| Resolve remaining blocking issues   | @lcnr, @compiler-errors, @BoxyUwU |       |
| Achieve performance parity          | @lcnr, @compiler-errors           |       |
| Cycle semantics RFC                 | @lcnr, @nikomatsakis              |       |
| Complete stabilization report       | @lcnr                             |       |
| Stabilization                       | @lcnr                             |       |

## Team asks

| Team    | Support level | Notes                                       |
| ------- | ------------- | ------------------------------------------- |
| [lang]  | Medium        | Cycle semantics RFC                         |
| [types] | Large         | Stabilization decision, ongoing review work |

## Frequently asked questions

None yet.

[rust-lang/rust-project-goals#113]: https://github.com/rust-lang/rust-project-goals/issues/113
