# Stabilize public/private dependencies

| Metadata         |                                    |
|:-----------------|:-----------------------------------|
| Point of contact | @epage                             |
| Status           | Proposed                           |
| Needs            | Contributor                        |
| Zulip channel    | N/A                                |
| Tracking issue   | [rust-lang/rust-project-goals#272] |
| [cargo] champion | @epage                             |

## Summary

Find a MVP for stabilization and move it forward.

## Motivation

This will allow users to tell Rustc and Cargo what dependencies are private
- Help users catch ways they unexpectedly expose their implementation details
- Help tooling better identify what all constitutes an API

### The status quo

[RFC #1977](https://github.com/rust-lang/rfcs/pull/1977) has been superseded by 
[RFC #3516](https://github.com/rust-lang/rfcs/pull/3516) to reduce complexity on the Cargo side to help get this over the line.
However, there is still a lot of complexity on the compiler side to get this right
(
[rust#3516](https://github.com/rust-lang/rfcs/pull/3516),
[rust#119428](https://github.com/rust-lang/rust/issues/119428),
),
keeping this feature in limbo

### The next 6 months

Work with [compiler] to identify a minimal subset of functionality for what the lint can do and close out the remaining stabilization tasks.

### The "shiny future" we are working towards

## Design axioms

- False negatives are likely better than false positives

## Ownership and team asks

*This section defines the specific work items that are planned and who is expected to do them. It should also include what will be needed from Rust teams. The table below shows some common sets of asks and work, but feel free to adjust it as needed. Every row in the table should either correspond to something done by a contributor or something asked of a team. For items done by a contributor, list the contributor, or ![Heap wanted][] if you don't yet know who will do it. For things asked of teams, list ![Team][] and the name of the team. The things typically asked of teams are defined in the [Definitions](#definitions) section below.*

| Task                         | Owner(s) or team(s)           | Notes |
|------------------------------|-------------------------------|-------|
| Discussion and moral support | ![Team][] [cargo], [compiler] |       |
| Work through #3516, #119428  | ![Help wanted][]              |       |
| Finish any remaining tasks   | ![Help wanted][]              |       |
| Mentoring                    | @epage                        |       |
| Stabilization report         | ![Help wanted][]              |       |

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
