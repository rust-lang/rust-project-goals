# Scalable Polonius support on nightly

| Metadata |          |
| -------- | -------- |
| Owner(s) | @lqd     |
| Teams    | [types]  |
| Status   | Proposed |

## Summary

Improve the Nightly support for [Polonius][pc3], next generation borrow checking, so that it can scale

## Motivation

Polonius is an improved version of the borrow checker that [resolves common limitations of the borrow checker][pc3] and which is needed to support future patterns such as "lending iterators" (see [#92985]). Its model also prepares us for further improvements in the future.

[pc3]: https://blog.rust-lang.org/inside-rust/2023/10/06/polonius-update.html#background-on-polonius

### The status quo

### The next six months

* Land polonius on nightly

### The "shiny future" we are working towards

Stable support for Polonius.

## Design axioms

N/A

## Ownership and team asks

**Owner:** lqd

Other support provided by @amandasystems as part of her PhD.

[amanda]: https://github.com/amandasystems

| Subgoal          | Owner(s) or team(s)  | Notes          |
| ---------------- | -------------------- | -------------- |
| Design review    | @nikomatsakis        |                |
| Implementation   | @lqd, @amandasystems |                |
| Standard reviews | ![Team][] [types]    | @matthewjasper |

### Support needed from the project

We expect most support to be needed from the types team, for design, reviews, interactions with the trait solver, and so on. We expect @nikomatsakis, leading the polonius working group and design, to provide guidance and design time, and @compiler-errors and @matthewjasper to help with reviews.

## Outputs and milestones

### Outputs

Nightly implementation of polonius that passes [NLL problem case #3][pc3] and accepts lending iterators ([#92985]).

Performance should be reasonable enough that we can run the full test suite, do crater runs, and test it on CI, without significant slowdowns. We do not expect to be production-ready yet by then, and therefore the implementation would still be gated under a nightly -Z feature flag.

As our model is a superset of NLLs, we expect little to no diagnostics regressions, but improvements would probably still be needed for the new errors.

### Milestones

| Milestone                                                              | Expected date |
| ---------------------------------------------------------------------- | ------------- |
| Factoring out higher-ranked concerns from the main path                | TBD           |
| Replace parts of the borrow checker with location-insensitive Polonius | TBD           |
| Location-sensitive prototype on nightly                                | TBD           |
| Verify full test suite/crater pass with location-sensitive Polonius    | TBD           |
| Location-sensitive pass on nightly, tested on CI                       | TBD           |

## Frequently asked questions

None yet.