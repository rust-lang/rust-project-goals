# rustc-perf improvements

| Metadata         |                     |
|------------------|---------------------|
| Point of contact | @kobzol             |
| Teams            | [infra], [compiler] |
| Status           | Proposed            |

*Arm's Rust team is @davidtwco, @adamgemmell, @jacobbramley, @JamieCunliffe and @Jamesbarford.
This goal will be primarily worked on by @Jamesbarford, but @davidtwco can always be contacted for
updates.*

## Summary

Add support to rustc-perf for distributed benchmarking across multiple platforms and configuration.

## Motivation

Improving the performance of the Rust compiler is a long-standing objective of the Rust project
and compiler team, which has led to the development of the project's performance tracking
infrastructure. While the performance tracking infrastructure has seen many improvements in recent
years, it cannot scale to support multiple benchmarking machines simultaneously.

There are increasingly demands on the performance infrastructure which require a more scalable
benchmarking infrastructure - benchmarking the parallel compiler with different thread counts,
different codegen backends, or different architectures.

### The status quo

rustc-perf does not currently support scheduling and accepting benchmarks from multiple machines,
requiring a non-trivial rearchitecting to do so. None of our policies around performance triage
and handling regressions currently consider what to do in case of conflicting benchmarking results.

### The next 6 months

rustc-perf's maintainers have [written a rough draft of the work required to support multiple
collectors](https://hackmd.io/X1wsQvkwQB-gb5PO7t2Czw) which will form the basis of the work
completed during this goal. After aligning on a implementation plan with the upstream maintainers
of rustc-perf and ensuring that the implementation can proceed while placing as little burden on
the infra team as possible, the work will largely consist of:

1. Establish a parallel testing infrastructure to avoid any disruption to the live rustc-perf
   service
2. Plan and implement necessary refactorings to the rustc-perf infrastructure enabling distributed
   benchmarking
3. Writing tests for the new distributed rustc-perf infrastructure, enabling future development to
   avoid breakage
4. Make changes to the database schema to support receiving results from multiple collectors (both
   being able to distinguish between results from each configuration and be have multiple writes
   simultaneously)
5. Update queries and statistics used in summarising collected performance data and identifying
   outliers
6. Update perf.rust-lang.org to be able to display performance data from multiple collectors and
   make appropriate comparisons (within a configuration, not between configurations)

As this work nears completion, this goal's owners will collaborate with the compiler team and its
performance working group to extend and update the compiler team's triage and regression handling
policies. It is important that there are clear guidelines and procedures for circumstances where a
 benchmark improves on one platform and regresses on another, or how to weigh benchmark results
from unstable features or configurations (e.g. `-Zthreads=2`) vs the primary benchmarking platforms
and configurations.

### The "shiny future" we are working towards

Following the completion of this goal, it is anticipated that new platforms and configurations
will be added to rustc-perf, but this is unlikely to warrant further goals.

## Ownership and team asks

| Task                                          | Owner(s) or team(s)  | Notes                                           |
|-----------------------------------------------|----------------------|-------------------------------------------------|
| Discussion and moral support                  | ![Team][] [infra]    |                                                 |
| Improve rustc-perf implementation work        | @Jamesbarford        |                                                 |
| Standard reviews                              | ![Team][] [infra]    |                                                 |
| Deploy to production                          | ![Team][] [infra]    | rustc-perf improvements, testing infrastructure |
| Draft performance regression policy           | @davidtwco           |                                                 |
| Policy decision                               | ![Team][] [compiler] | Update performance regression policy            |
| Inside Rust blog post announcing improvements | @davidtwco           |                                                 |

### Definitions

Definitions for terms used above:

* *Discussion and moral support* is the lowest level offering, basically committing the team to
  nothing but good vibes and general support for this endeavor.
* *Author RFC* and *Implementation* means actually writing the code, document, whatever.
* *Design meeting* means holding a synchronous meeting to review a proposal and provide feedback
  (no decision expected).
* *RFC decisions* means reviewing an RFC and deciding whether to accept.
* *Org decisions* means reaching a decision on an organizational or policy matter.
* *Secondary review* of an RFC means that the team is "tangentially" involved in the RFC and
  should be expected to briefly review.
* *Stabilizations* means reviewing a stabilization and report and deciding whether to stabilize.
* *Standard reviews* refers to reviews for PRs against the repository; these PRs are not expected
  to be unduly large or complicated.
* *Prioritized nominations* refers to prioritized lang-team response to nominated issues, with
  the expectation that there will be *some* response from the next weekly triage meeting.
* *Dedicated review* means identifying an individual (or group of individuals) who will review the
  changes, as they're expected to require significant context.
* Other kinds of decisions:
    * [Lang team experiments](https://lang-team.rust-lang.org/how_to/experiment.html) are used
      to add nightly features that do not yet have an RFC. They are limited to trusted contributors
      and are used to resolve design details such that an RFC can be written.
    * Compiler [Major Change Proposal (MCP)](https://forge.rust-lang.org/compiler/mcp.html) is
      used to propose a 'larger than average' change and get feedback from the compiler team.
    * Library [API Change Proposal (ACP)](https://std-dev-guide.rust-lang.org/development/feature-lifecycle.html)
      describes a change to the standard library.

## Frequently asked questions

None yet.
