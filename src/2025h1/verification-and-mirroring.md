# Secure quorum-based cryptographic verification and mirroring for crates.io

| Metadata |                         |
|----------|-------------------------|
| Point of contact | @walterhpearce          |
| Teams | <!-- TEAMS WITH ASKS --> |
| Task owners      | <!-- TASK OWNERS --> |
| Status   | Proposed                |

## Summary

Within 6 months, we will provide preliminary infrastructure to cryptographically verify the crates.io repository and experimental mirrors of it. This will include a chain-of-trust to the Rust Project via a quorum-based mechanism, and methods to verify singular Rust crates, their singular index entries, as well as the index and the artifacts as a whole.

## Motivation

Rustaceans need to be able to download crates and know that they're getting the crate files that were published to crates.io without modification. Rustaceans everywhere should be able to use local mirrors of crates.io, such as geographically distributed mirrors or mirrors within infrastructure (e.g. CI) that they're using.

### The status quo

Currently cargo and crates.io provide no cryptographic security for the index or crates. The only verification which occurs is via HTTPS validation of the URLs and tamperable hashes within the index. This provides assurance that cargo is talking to crates.io, but does not allow for the possibility of secure mirroring nor protects against compromise or tampering with the index (either at rest or in transit).

There are places where Rust is difficult to use right now. Using Cargo with crates.io works well for Rustaceans with unfirewalled access to high speed Internet, but not all are so lucky. Some are behind restrictive firewalls which they are required to use. Some don't have reliable access to the Internet. In cases like these, we want to support mirrors of crates.io in a secure way that provides cryptographic guarantees that they are getting the same packages as are provided by the Rust Project, without any risk of tampering.

Another reason for wanting to be able to better support mirrors is to address cost pressures on Rust. Approximately half of Rust release and crate traffic is from CI providers. Being able to securely distribute Rust crates from within CI infrastructure would be mutually beneficial, since it would both allow the Rust Foundation to reallocate budget to other uses and would make Rust CI actions faster and more reliable on those platforms.

Finally, supply chain security is a growing concern, particularly among corporate and government users of Rust. The Log4j vulnerability brought much greater attention to the problems that can occur when a single dependency nested arbitrarily deep in a dependency graph has a critical vulnerability. Many of these users are putting significant resources into better understanding their dependencies, which includes being able to attest that their dependencies verifiably came from specific sources like crates.io.

### The next 6 months

We would like to have a working production signing pipeline for all crates published to crates.io, which can be verified back to the Rust Project. The leadership council will have selected a trusted root quorum for the project, and that quorum will have completed their first signing ceremony. Crates.io will have integrated automatic signing of published crates into their pipeline and the signatures will be included in the index. Finally, we'll provide some method for end users to verify these signatures (ideally in cargo, but at a minimum as a cargo subcommand for proof-of-concept). We'll use that infrastructure to demonstrate how a mirror could function.

### The "shiny future" we are working towards

In the future, we intend to provide production mirroring capabilities, and some mechanism for automatic mirror discovery. Cargo should be able to automatically discover and use mirrors provided within CI infrastructure, within companies, or within geographic regions, and cryptographically verify those that mirrors are providing unmodified crates and indexes from crates.io.

We'll extend this cryptographic verification infrastructure to rustup-distributed Rust releases and nightly versions, and support mirroring of those as well.

We'll provide cryptographic verification of our GitHub source repositories, and some demonstration of how to verify mirrors of those repositories.

We hope to have follow-up RFCs which will enable authors to generate their own quorums for author and organization level signing and validation of the crates they own.

We'll add support for similar cryptographic security for third-party crate repositories.

## Ownership and team asks

| Task                                              | Owner(s) or team(s) | Notes |
|---------------------------------------------------|---------------------|-------|
| Inside Rust blog post about staging deployment    | @walterhpearce      |       |
| Top-level Rust blog post on production deployment | @walterhpearce      |       |

### Quorum-based cryptographic infrastructure (RFC 3724)

| Task                                  | Owner(s) or team(s)                   | Notes              |
|---------------------------------------|---------------------------------------|--------------------|
| Further revisions to RFC              | @walterhpearce                        |                    |
| RFC decision                          | ![Team][] [cargo] [crates-io] [infra] |                    |
| Implementation and staging deployment | @walterhpearce, [crates-io], [infra]  |                    |
| Miscellaneous                         | ![Team][] [leadership-council]        | Select root quorum |
| Deploy to production                  | ![Team][] [crates-io] [infra]         |                    |

### Draft RFC for mirroring crates.io via alternate repositories

| Task                                   | Owner(s) or team(s)          | Notes |
|----------------------------------------|------------------------------|-------|
| Discussion and moral support           | ![Team][] [cargo]            |       |
| Author RFC                             | @walterhpearce @joshtriplett |       |
| Proof of concept technical experiments | @walterhpearce               |       |


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
