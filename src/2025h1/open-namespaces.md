# Implement Open API Namespace Support
| Metadata         |                                    |
|:-----------------|:-----------------------------------|
| Point of contact | @epage                             |
| Status           | Accepted                           |
| Needs            | Contributor                        |
| Zulip channel    | N/A                                |
| Tracking issue   | [rust-lang/rust-project-goals#256] |

## Summary

Navigate the cross-team design work to get [RFC 3243](https://github.com/rust-lang/rfcs/pull/3243) implemented.

## Motivation

[RFC 3243](https://github.com/rust-lang/rfcs/pull/3243) proposed opening up namespaces in Rust to extension,
managed by the package name with crates-io putting access control on who can publish to a crate's API namespace.
This covers multiple teams and needs a lot of coordination to balance the needs of each team as shown on the [rustc tracking issue](https://github.com/rust-lang/rust/issues/122349).

### The status quo

Cargo support is partially implemented.
No compiler support.
There is a crates-io prototype for a previous iteration of RFC 3243 but that code base has likely diverged a lot since then.

### The next 6 months

Implement at least Cargo and compiler support for this to be experimented with and allow crates-io work.

### The "shiny future" we are working towards

## Design axioms

## Ownership and team asks

| Task                                | Owner(s) or team(s)           | Notes |
|-------------------------------------|-------------------------------|-------|
| Discussion and moral support        | ![Team][] [cargo], [compiler] |       |
| Compiler implementation             | @b-naber                      |       |
| Work through lingering cargo issues | @epage, @b-naber              |       |

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