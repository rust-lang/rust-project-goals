# Integration of the FLS into the Rust Project

| Metadata |                                                              |
| -------- | ------------------------------------------------------------ |
| Owner(s) | Joel Marcey |
| Teams    | spec          |
| Status   | Proposed                                                     |

## Summary

Ferrous Systems will be transfering the Ferrocene Language Specification (FLS) to the Rust Project, under the ownership of the Specification Team, or `t-spec`. In the first half of 2025, the Specification team will integrate the FLS, under an appropriate name, into both its design and development processes, and the project as a whole.

## Motivation

The Specification Team has been working for the past year on preparing a specification of Rust. Over this time, the Team has made and began executing several distinct plans to achieve this: creating a new document; modifying the reference; and most recently, agreeing with Ferrous Systems to take ownership of the FLS to support its specification delivery efforts. The current plan is to do the latter two processes in parrallel, and in order to do that effectively the Ferrocene Language Specification needs to be adopted and integrated into the project processes and tooling.

### The status quo

[RFC 3355] describes the goals of the specification as  "\[Serving\] the needs of Rust users, such as authors of unsafe Rust code, those working on safety critical Rust software, language designers, maintainers of Rust tooling, and so on," and "Incorporating it in their process for language evolution. For example, the language team could require a new language feature to be included in the specification as a requirement for stabilization."

Presently, the working draft Specification of Rust consists of a modified version of the reference, achieved by adding paragraph identifiers (almost finished), and slowly modifying the content to more normatively describe the language. This may help achieve one of the presented goals for the the specification, namely incorporation into the language evolution process. 
However, Ferrous Systems has, over the past 2 years, developed the Ferrocene Language Specification, which has seen adoption in the Safety Critical Space, and a sharp change in the specification would create substantial financial burdens on those early adopters. 

Based on more recent discussions and agreements with Ferrous Systems, the Specification Team will be incorporating the the Ferrocene Language Specification as-is into its processes. This will leave us with two documents to maintain, with decisions to make on how they will fit into the Specification delivery process overall.

### The next 6 months

In order to properly integrate the Ferrocene Language Specification, presumably under a different name, the specification team will need to adopt processes surrounding modification, editing, review, and release of the document. 

### The "shiny future" we are working towards

The goal is designed to move forward the Rust Specification, in a way that is satisfying to both internal and external consumers, and that makes progress on the overall goals set out in [RFC 3355]. It is also designed to put us in a position for a 2025h2 goal of producing a first useful version of the specification that satisfies those goals, as well as any ancillary work that needs to be done along side the specification itself. 

[RFC 3355]: https://rust-lang.github.io/rfcs/3355-rust-spec.html

## Design axioms

The following [design axioms][da] apply:
* Making Decisions Effectively, but Efficiently: When the goal asks the Team to make a decision, the Team should be prepared in advance with the necessary background, and come to consensus based on as much information as is possible, but at the same time, acting with efficiency and alacrity, not spending more time than is necessary on a decision. In particular, the team should not delay discussing a decision more than is necessary.
    * Elaborating on the last part, decisions the team are well aware of needing to make should not be deferred once all of the requesite information is available, unless a higher priority decision needs to supplant it.
* Iterative changes are better: When it comes to making modifications, particularily to the FLS, slow and gradual ones should be preferred to sharp, major ones.

[da]: ../about/design_axioms.md

## Ownership and team asks

**Owner:** As the hired specification editor, Joel Marcey will own the overall goal. Connor Horman will also aid in bringing the goal to completion in their role as a Contractor. 

Some subgoals list an expected due/completion date. If one is omitted, compeletion by the end of 2025h1 is implied.


| Subgoal                                               | Owner(s) or team(s) | Notes                                   |
| ----------------------------------------------------- | ------------------- | --------------------------------------- |
| Complete Taking Ownership of the FLS                  | ![Team][] [spec]    | Prior to, or shortly into January 2025. |
| Integrate FLS into T-spec processes                   | ![Team][] [spec]    |                                         |
| ↳Review Existing Editorial Standards in the FLS       |                     | End of January 2025                     |
| ↳Review Tooling used by the FLS                       | Connor Horman       | End of January 2025                     |
| ↳Author Proposal for specifics of FLS integration     | Connor Horman       | Mid-Late Februrary 2025                 |
| ↳Review, iterate, and accept Integration Proposal     | ![Team][] [spec]    | End of March 2025                       |
| ↳Adjust Tooling, as needed                            | Connor Horman       | April 2025                              |
| ↳Begin implementing the integration Proposal          | Connor Horman       |                                         |
| Integrate FLS into release process                    | ![Team][] [release]|                                         |
| ↳Discuss requirements with T-release                  |                     | Februrary 2025                          |
| ↳Link tooling used with FLS to the release process    |                     | April 2025                              |
| ↳Review of FLS prior to release                       |                     | May 2025                                |
| ↳Get FLS into a Rust Release                          |                     | Rust 1.89                               |


## Frequently asked questions

