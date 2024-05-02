# Polonius

| Metadata |                 |
| -------- | --------------- |
| Owner(s) | [lqd]           |
| Teams    | [Lang], [Types] |
| Status   | WIP             |

[lqd]: https://github.com/lqd
[Lang]: https://www.rust-lang.org/governance/teams/lang
[Types]: https://www.rust-lang.org/governance/teams/compiler#team-types

## Motivation

Polonius is an improved version of the borrow checker that [resolves common limitations of the borrow checker][pc3] and which is needed to support future patterns such as ["lending iterators"][#92985].

[pc3]: https://blog.rust-lang.org/inside-rust/2023/10/06/polonius-update.html#background-on-polonius
[#92985]: https://github.com/rust-lang/rust/issues/92985

### The status quo

### The next few steps

* Land polonius on nightly

*Sketch out the specific things you are trying to achieve in 2024. This should be short and high-level -- we don't want to see the design!*

### The "shiny future" we are working towards

*If this goal is part of a larger plan that will extend beyond this goal period, sketch out the goal you are working towards. It may be worth adding some text about why these particular goals were chosen as the next logical step to focus on.*

## Design axioms

*Add your [design axioms][da] here. Design axioms clarify the constraints and tradeoffs you will use as you do your design work. These are most important for project goals where the route to the solution has significant ambiguity (e.g., designing a language feature or an API), as they communicate to your reader how you plan to approach the problem. If this goal is more aimed at implementation, then design axioms are less important. [Read more about design axioms][da].*

[da]: ../about/design_axioms.md

## Ownership and other resources

**Owner:** lqd

Other support provided by Amanda Stjerna.

### Support needed from the project

XXX

## Outputs and milestones

### Outputs

* Nightly implementation of polonius that passes [problem case #3][pc3] and accepts [lending iterators][#92985].

### Milestones

| Milestone                                                              | Expected date |
| ---------------------------------------------------------------------- | ------------- |
| Factoring out higher-ranked concerns from the main path                | TBD           |
| Location-insensitive loans in scope                                    | TBD           |
| Verify full test suite passes with location-insensitive Polonius       | TBD           |
| Replace parts of the borrow checker with location-insensitive Polonius | TBD           |
| Location-sensitive pass on nightly                                     | TBD           |

## Frequently asked questions

None yet.