# Expanding a-mir-formality to work better as a Rust type system spec

| Metadata         |                                                                                  |
| :--------------- | -------------------------------------------------------------------------------- |
| Point of contact | @jackh726                          |
| Status           | Proposed                                                                         |
| Tracking issue   | [rust-lang/rust-project-goals#122]     |
| Zulip channel    | [#types/formality] |

[#types/formality]: https://rust-lang.zulipchat.com/#narrow/channel/402470-t-types.2Fformality

## Summary

The goal here is to begin to move a-mir-formality into a position where it can be used as a formal specification of the Rust type system:
- Identify type system areas lacking in a-mir-formality
- Build a "roadmap" to add those lacking areas
- Continuing to contribute to a-mir-formality and build towards completeness
- Experiment with integration of a-mir-formality into an experimental reference

## Motivation

### The status quo

Most communication and definition of Rust's type/trait system today takes place through informal argument and with reference to compiler internals. a-mir-formality offers a model of Rust at a much higher level, but it remains very incomplete compared to Rust.

Previously, there has been some progress in increasing contributions to a-mir-formality by others besides @nikomatsakis, but we still haven't achieved a "fully living" project. That being said, steady progress has been and continues to be made, and we are at a point where it makes sense to try to outline a concrete plan for the future.

### What we propose to do about it

The work here is primarily divided into two main goals:
1) Document the status of a-mir-formality implementation, identify a plan towards "completion", and work towards that.
2) Experiment with integration of a-mir-formality into an "experimental reference" (led primarily as a separate goal).

For the former, it is expected this will intersect with other ongoing work (such as implementing borrow checking in a-mir-formality). It is also likely that this will be done through some combination of manual identification (such as through census of open issues) and more automatic identification (such as cross-testing the rustc test suite).

For the latter, it is not yet clear in what form the integration with the reference will occur. Possibilities may range from simple linking of behavior from the reference to the relevant rules in a-mir-formality, to more complex integration, such as embedding of a-mir-formality tests or rules within the reference itself. Notably, the goal is to allow the reference to remain a user-friendly text while providing a platform to share the more concrete formalism of the type system.

| Task        | Owner(s) | Notes |
| ----------- | -------- | ----- |
| Implementation documentation | @jackh726   |       |
| Reviews | @nikomatsakis   |       |
| Implementation work | @jackh726, @nikomatsakis, @tiff   |       |
| Design and implementation of a-mir-formality integration | @jackh726 |

## Team asks

| Team       | Support level | Notes                                   |
| ---------- | ------------- | --------------------------------------- |
| [types]    |  Small   | Members may have comments/thoughts on direction and priorities; Review work for a-mir-formality     |
| [spec]     |  Vibes   | General vibes for integration of a-mir-formality with reference |
| [lang-docs] | Vibes | Thoughts on shape of integration of a-mir-formality into reference |
 
## Frequently asked questions

### Does this cover XYZ work in a-mir-formality?

Generally, the work under the first subgoal can be thought of as the "meta" work that may be covered under other goals. The hope is that *this* goal should be mainly focused on the planning and organization around "finishing" the implementation of a-mir-formality to be a "complete" definition of the Rust type system, and that may include discussions about other ongoing work.

### Is there an expectation that the integration of a-mir-formality into the reference must eventually be merged into the main reference?

Generally, *no*. The goal of the proposed work is not *necessarily* to find some single solution to integrate a-mir-formality into the reference, but rather to learn more about how this integration *could* happen, and moreso to *inform* us about how we can document the Rust type system in the reference in the best way possible to maintain preciseness with the constraint of user-friendly text.
