# rustc-perf improvements

| Metadata           |                                                          |
| :--                | :--                                                      |
| Point of contact   | @Jamesbarford                                            |
| Teams              | <!-- TEAMS WITH ASKS -->                                 |
| Task owners        | <!-- TASK OWNERS -->                                     |
| Status             | Proposed                                                 |
| Zulip channel      | [#project-goals/2025h1/rustc-perf-improvements][channel] |
| Tracking issue     | [rust-lang/rust-project-goals#275]                       |

[channel]: https://rust-lang.zulipchat.com/#narrow/channel/478771-project-goals.2F2025h1.2Frustc-perf-improvements

*This goal will be primarily worked on by @Jamesbarford, but @davidtwco or
@Kobzol can always be contacted for updates.*

## Summary

Continue our efforts from 2025H1 to add support to rustc-perf for distributed
benchmarking across multiple platforms and configuration.

## Motivation

Improving the performance of the Rust compiler is a long-standing objective of
the Rust project and compiler team, which has led to the development of the
project's performance tracking infrastructure. While the performance tracking
infrastructure has seen many improvements in recent years, it cannot scale to
support multiple benchmarking machines simultaneously.

There are increasingly demands on the performance infrastructure which require a
more scalable benchmarking infrastructure - benchmarking the parallel compiler
with different thread counts, different codegen backends, or different
architectures.

### The status quo

rustc-perf does not currently support scheduling and accepting benchmarks from
multiple machines, requiring a non-trivial rearchitecting to do so. None of our
policies around performance triage and handling regressions currently consider
what to do in case of conflicting benchmarking results.

...

### The next 6 months

...

### The "shiny future" we are working towards

Following the completion of this goal, it is anticipated that new platforms and
configurations will be added to rustc-perf, but this is unlikely to warrant
further goals.

## Ownership and team asks

| Task                                          | Owner(s) or team(s)    | Notes                                           |
|-----------------------------------------------|------------------------|-------------------------------------------------|
| Discussion and moral support                  | ![Team][] [infra]      |                                                 |
| Improve rustc-perf implementation work        | @Jamesbarford, @Kobzol |                                                 |
| Standard reviews                              | ![Team][] [infra]      |                                                 |
| Deploy to production                          | ![Team][] [infra]      | rustc-perf improvements, testing infrastructure |
| Draft performance regression policy           | @davidtwco             |                                                 |
| Policy decision                               | ![Team][] [compiler]   | Update performance regression policy            |
| Inside Rust blog post announcing improvements | @Kobzol, @davidtwco    |                                                 |

### Definitions

Definitions for terms used above:

* *Discussion and moral support* is the lowest level offering, basically
  committing the team to nothing but good vibes and general support for this
  endeavor.
* *Author RFC* and *Implementation* means actually writing the code, document,
  whatever.
* *Design meeting* means holding a synchronous meeting to review a proposal and
  provide feedback (no decision expected).
* *RFC decisions* means reviewing an RFC and deciding whether to accept.
* *Org decisions* means reaching a decision on an organizational or policy
  matter.
* *Secondary review* of an RFC means that the team is "tangentially" involved in
  the RFC and should be expected to briefly review.
* *Stabilizations* means reviewing a stabilization and report and deciding
  whether to stabilize.
* *Standard reviews* refers to reviews for PRs against the repository; these PRs
  are not expected to be unduly large or complicated.
* *Prioritized nominations* refers to prioritized lang-team response to
  nominated issues, with the expectation that there will be *some* response from
  the next weekly triage meeting.
* *Dedicated review* means identifying an individual (or group of individuals)
  who will review the changes, as they're expected to require significant
  context.
* Other kinds of decisions:
    * [Lang team experiments][experiment] are used to add nightly features that
      do not yet have an RFC. They are limited to trusted contributors and are
      used to resolve design details such that an RFC can be written.
    * Compiler [Major Change Proposal (MCP)][mcp] is used to propose a 'larger
      than average' change and get feedback from the compiler team.
    * Library [API Change Proposal (ACP)][acp] describes a change to the
      standard library.

[experiment]: https://lang-team.rust-lang.org/how_to/experiment.html
[mcp]: https://forge.rust-lang.org/compiler/mcp.html
[acp]: https://std-dev-guide.rust-lang.org/development/feature-lifecycle.html

## Frequently asked questions

None yet.