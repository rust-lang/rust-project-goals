# Improve state machine codegen 

<!--

> **Instructions:** Copy this template to a fresh file with a name based on your plan.
> Give it a title that describes what you plan to get done in the next 6 months
> (e.g., "stabilize X" or "nightly support for X" or "gather data about X").
> Feel free to replace any text with anything, but there are placeholders
> designed to help you get started. 

-->

| Metadata         |             |
|:-----------------|-------------|
| Point of contact | @folkertdev |
| Teams | <!-- TEAMS WITH ASKS --> |
| Task owners      | <!-- TASK OWNERS --> |
| Status           | Proposed    |
| Zulip channel    | N/A         |

## Summary

We want to improve rustc codegen, based on [this initialive](https://trifectatech.org/initiatives/workplans/codegen/) by the Trifecta Tech Foundation. The work focusses on improving state machine code generation, and finding (and hopefully fixing) cases where clang produces better code than rustc for roughly equivalent input.

## Motivation

Matching C performance is crucial for rust adoption in performance-sensitive domains. Rust is doing well overall, but not good enough. 

In compression, video decoding and other high-performance areas, nobody will use rust if it is even a couple percent slower: latency, power (i.e. battery) consumption and other factors are just more important than whatever advantages rust can bring. In particular, we've observed that C code translated to rust code, whether manually or mechanically, often performs a couple percent worse than the original C.

Given that we clang and rustc both use LLVM for code generation, there is no fundamental reason that rust should be slower.

### The status quo

Our target audience is users of rust in performance-sensitive domains, where the rustc codegen hinders adoption of rust. Concretely we have most experience with, and knowledge of the bottlenecks in these projects:

- the [`zlib-rs`](https://github.com/trifectatechfoundation/zlib-rs) and [`libbzip2-rs-sys`](https://github.com/trifectatechfoundation/libbzip2-rs) compression algorithms
- the [`rav1d`](https://github.com/memorysafety/rav1d/tree/main) av1 decoder

In the compression libraries, we spotted a specific pattern (in rust terms, a `loop` containing a `match`) where rust is not able to generate good code today. We wrote [RFC 3720](https://github.com/rust-lang/rfcs/pull/3720) to tackle this problem.

In the case of rav1d, the performance is several percent worse than its C equivalent dav1d. The rav1d project used [c2rust](https://github.com/immunant/c2rust) to translate the dav1d C source to rust. Hence the two code bases are basically equivalent, and we'd expect basically identical performance.

The rav1d developers were unable to track down the reason that rav1d performs worse than dav1d: their impression (that we have confirmed with various rustc developers) is that rustc+llvm is just slightly worse at generating code than clang+llvm, because llvm overfits to what clang gives it. 

### The next 6 months

#### Improve state machine codegen

The problem, and a range of possible solutions, is described in [RFC 3720](https://github.com/rust-lang/rfcs/pull/3720).

- recognize the problematic pattern in zlib-rs in HIR, based on a fragile heuristic
- ensure it is eventually turned into a `goto` to the actual target in MIR
- evaluate how effective that is for other projects (e.g. rustc itself)
- depending on how RFC 3720 evolves, implement the specific proposal (syntax, lints, error messages) 

#### Finding performance bottlenecks

We want to build a tool that uses `creduce` and `c2rust` to find small examples where clang+llvm produces meaningfully better code than rust+llvm.

The output will be either issues with small rust snippets that have suboptimal codegen (compared to clang) or PRs fixing these problems.

### The "shiny future" we are working towards

The shiny future is to improve rust codegen to encourage wider adoption of rust in performance-sensitive domains.

<!--

## Design axioms

*This section is optional, but including [design axioms][da] can help you signal how you intend to balance constraints and tradeoffs (e.g., "prefer ease of use over performance" or vice versa). Teams should review the axioms and make sure they agree. [Read more about design axioms][da].*

[da]: ../about/design_axioms.md

-->

## Ownership and team asks

**Owner:** *Identify a specific person or small group of people if possible, else the group that will provide the owner. Github user names are commonly used to remove ambiguity.*

*This section defines the specific work items that are planned and who is expected to do them. It should also include what will be needed from Rust teams. The table below shows some common sets of asks and work, but feel free to adjust it as needed. Every row in the table should either correspond to something done by a contributor or something asked of a team. For items done by a contributor, list the contributor, or ![Heap wanted][] if you don't yet know who will do it. For things asked of teams, list ![Team][] and the name of the team. The things typically asked of teams are defined in the [Definitions](#definitions) section below.*

| Task                         | Owner(s) or team(s)  | Notes |
|------------------------------|----------------------|-------|
| Discussion and moral support | ![Team][] [compiler] |       |
| Lang-team experiment         | ![Team][] [lang]     |       |
| Lang-team champion           | ![Team][] [lang]     | @traviscross      |
| Refine RFC 3720              | @folkertdev          |       |
| Implementation               | @folkertdev, @bjorn3 |       |
| Standard reviews             | ![Team][] [compiler] |       |


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

None yet

<!-- 

### What do I do with this space?

*This is a good place to elaborate on your reasoning above -- for example, why did you put the design axioms in the order that you did? It's also a good place to put the answers to any questions that come up during discussion. The expectation is that this FAQ section will grow as the goal is discussed and eventually should contain a complete summary of the points raised along the way.*

-->
