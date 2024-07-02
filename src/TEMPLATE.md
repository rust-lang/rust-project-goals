# TEMPLATE (replace with title of your goal)

> **Instructions:** Copy this template to a fresh file with a name based on your plan.
> Give it a title that describes what you plan to get done in the next 6 months
> (e.g., "stabilize X" or "nightly support for X" or "gather data about X").
> Feel free to replace any text with anything, but there are placeholders
> designed to help you get started. 

| Metadata |                                                              |
| -------- | ------------------------------------------------------------ |
| Owner(s) | *Github usernames or other identifying info for goal owners* |
| Teams    | *Names of teams being asked to commit to the goal*           |
| Status   | WIP                                                          |

## Summary

*Short description of what you will do over the next 6 months.*

## Motivation

*Begin with a few sentences summarizing the problem you are attacking and why it is important.*

### The status quo

*Elaborate in more detail about the problem you are trying to solve. This section is making the case for why this particular problem is worth prioritizing with project bandwidth. A strong status quo section will (a) identify the target audience and (b) give specifics about the problems they are facing today. Sometimes it may be useful to start sketching out how you think those problems will be addressed by your change, as well, though it's not necessary.*

### The next 6 months

*Sketch out the specific things you are trying to achieve in this goal period. This should be short and high-level -- we don't want to see the design!*

### The "shiny future" we are working towards

*If this goal is part of a larger plan that will extend beyond this goal period, sketch out the goal you are working towards. It may be worth adding some text about why these particular goals were chosen as the next logical step to focus on.*

*This text is NORMATIVE, in the sense that teams should review this and make sure they are aligned. If not, then the shiny future should be moved to frequently asked questions with a title like "what might we do next".*

## Design axioms

*This section is optional, but including [design axioms][da] can help you signal how you intend to balance constraints and tradeoffs (e.g., "prefer ease of use over performance" or vice versa). Teams should review the axioms and make sure they agree. [Read more about design axioms][da].*

[da]: ../about/design_axioms.md

## Ownership and other resources

**Owner:** *Identify a specific person or small group of people if possible, else the group that will provide the owner*

This section defines the specific work items that are planned and who is expected to do them. It should also include what will be needed from Rust teams.

* Subgoal:
    * Describe the work to be done and use `↳` to mark "subitems".
* Owner(s) or team(s):
    * List the owner for this item (who will do the work) or ![Help wanted][] if an owner is needed.
    * If the item is a "team ask" (i.e., approve an RFC), put ![Team][] and the team name(s).
* Status:
    * List ![Help wanted][] if there is an owner but they need support, for example funding.
    * Other needs (e.g., complete, in FCP, etc) are also fine.

*Adjust the table below; some common examples are shown below.*

| Subgoal                                        | Owner(s) or team(s)  | Status |
| ---------------------------------------------- | -------------------- | ------ |
| Stabilize Feature X (typical language feature) |                      |        |
| ↳ author RFC                                   |                      |        |
| ↳ implementation                               |                      |        |
| ↳ design meeting                               | ![Team][] [Lang]     |        |
| ↳ approve RFC                                  | ![Team][] [Lang]     |        |
| ↳ stabilization report                         |                      |        |
| ↳ stabilization decision                       | ![Team][] [Lang]     |        |
| Nightly experiment for X                       |                      |        |
| ↳ author RFC                                   |                      |        |
| ↳ approve lang-team experiment                 | ![Team][] [Lang]     |        |
| ↳ implementation                               |                      |        |
| ↳ dedicated reviewer (not normally needed)     | ![Team][] [Compiler] |        |
| Inside Rust blog post inviting feedback        |                      |        |
| Top-level Rust blog post inviting feedback     | ![Team][] [LC]       |        |

[Help wanted]: https://img.shields.io/badge/Help%20wanted-yellow
[Complete]: https://img.shields.io/badge/Complete-green
[TBD]: https://img.shields.io/badge/TBD-red
[Team]: https://img.shields.io/badge/Team%20ask-red

[Compiler]: https://www.rust-lang.org/governance/teams/compiler
[Lang]: https://www.rust-lang.org/governance/teams/lang
[LC]: https://www.rust-lang.org/governance/teams/leadership-council
[Libs-API]: https://www.rust-lang.org/governance/teams/library#team-libs-api
[Infra]: https://www.rust-lang.org/governance/teams/infra
[Cargo]: https://www.rust-lang.org/governance/teams/dev-tools#team-cargo
[Types]: https://www.rust-lang.org/governance/teams/compiler#team-types

## Frequently asked questions

### What do I do with this space?

*This is a good place to elaborate on your reasoning above -- for example, why did you put the design axioms in the order that you did? It's also a good place to put the answers to any questions that come up during discussion. The expectation is that this FAQ section will grow as the goal is discussed and eventually should contain a complete summary of the points raised along the way.*