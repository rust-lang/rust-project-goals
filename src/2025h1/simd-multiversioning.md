# Nightly support for ergonomic SIMD multiversioning

| Metadata           |                                     |
| :--                | :--                                 |
| Point of contact   | @veluca93                           |
| Teams              | <!-- TEAMS WITH ASKS -->            |
| Task owners        | <!-- TASK OWNERS -->                |
| Status             | Accepted                            |
| Zulip channel      | [#project-portable-simd][channel]   |
| Tracking issue     | [rust-lang/rust-project-goals#261]  |

[channel]: https://rust-lang.zulipchat.com/#narrow/channel/257879-project-portable-simd/


## Summary

Figure out the best way for Rust to support generating code for multiple SIMD targets in a safe and ergonomic way.

## Motivation

Even within the same architecture, CPUs vary significantly in which SIMD ISA extensions they implement[^avx512].
Most libraries that are shipped to users in binary form thus tend to contain code for different SIMD targets and use runtime dispatch.
Having compiler support for this pattern, i.e. by having the compiler generate multiple versions of code written only once, would significantly help drive Rust's adoption in the world of codecs, where some of the [most subtle](https://blog.isosceles.com/the-webp-0day/) memory vulnerabilities are found, as well as other domains where squeezing out the last bits of performance is fundamental.

[^avx512]: For example, `x86` CPUs currently have about [12](https://en.wikipedia.org/wiki/AVX-512#CPUs_with_AVX-512) (!) different possible configurations with respect to AVX-512 support alone.


### The status quo

Currently, generating efficient code for a specific SIMD ISAs requires annotating the function with appropriate attributes. This is incompatible with generating multiple versions through i.e. generics.

This limitation can be worked around in different ways, all of which with some significant downsides:

- Intermediate functions can be annotated as `#[inline(always)]` and inline in a top-level caller, with downsides for code size.
- Calls between "multiversioned" functions do target selection again, which inhibits inlining and has performance implications.
- Programmers explicitly define no-inline boundaries and call functions across such boundaries in a different way; this requires significant boilerplate and has bad ergonomics.
- Macros can create multiple copies of the relevant functions and figure out how to call between those; this is bad for compilation times and not particularly rust-y.

There are currently multiple proposals for ways to resolve the above issues.
In brief: 
- allow ADTs to carry feature information and pass it on to functions that take them as argument
- have functions automatically inherit the target features of their callers
- let features depend on const generic arguments to functions

The trade-offs between the different approaches are complex, and there is no consensus on the best path forward. More details on the proposals can be found in [this document](https://hackmd.io/%40veluca93/simd-multiversioning).

### The next 6 months

- A design meeting is scheduled to discuss the best approach forward on this topic.
- A lang team experiment is approved, enabling exploration in the compiler of the proposed approach.
- A RFC is posted, based on the results of the exploration, and reviewed.
- The implementation is updated to reflect changes from the RFC, and becomes broadly available in the nightly compiler.

### The "shiny future" we are working towards

Once the proposed design is stabilized, Rust will offer one of the most compelling stories for achieving very high performance on multiple targets, with minimal friction for developers.

This significantly increases the adoption of Rust in performance-critical, safety-sensitive low level libraries.

## Design axioms

- The common case should be simple and ergonomic.
- Additional flexibility to unlock the maximum possible performance should be possible and sufficiently ergonomic.
- The vast majority of SIMD usage should be doable in safe Rust.

## Ownership and team asks

**Owner:** *Identify a specific person or small group of people if possible, else the group that will provide the owner. Github user names are commonly used to remove ambiguity.*

| Task                        | Owner(s) or team(s) | Notes |
|-----------------------------|---------------------|-------|
| Design meeting              | ![Team][] [lang]    |       |
| Lang-team experiment        | ![Team][] [lang]    |       |
| Experimental implementation | @veluca93           |       |
| Author RFC                  | @veluca93           |       |
| RFC decision                | ![Team][] [lang]    |       |


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

### What do I do with this space?

*This is a good place to elaborate on your reasoning above -- for example, why did you put the design axioms in the order that you did? It's also a good place to put the answers to any questions that come up during discussion. The expectation is that this FAQ section will grow as the goal is discussed and eventually should contain a complete summary of the points raised along the way.*