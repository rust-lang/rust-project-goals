# Metrics Initiative

| Metadata           |                                    |
| :--                | :--                                |
| :----------------- | --------------------------         |
| Point of contact   | @yaahc                             |
| Teams              | <!-- TEAMS WITH ASKS -->           |
| Task owners        | <!-- TASK OWNERS -->               |
| Status             | Proposed                           |
| Zulip channel      | N/A                                |
| Tracking issue     | [rust-lang/rust-project-goals#260] |

## Summary

Build out the support for metrics within the rust compiler and starting with a Proof of concept dashboard for viewing unstable feature usage statistics over time.

## Motivation

We're envisioning three use cases for the Metrics Initiative:

1. Supporting feature development, e.g. answering specific questions such as when the old and new trait solvers diverge, showing unstable feature usage trends, or helping identify and resolve bugs.
2. Guiding improvements to User Experience, e.g. knowing which compiler errors are causing the most confusion or are hit the most frequently, focusing on improving those first, and verifying that the improvements help.
3. Improving perf feedback loops and insight, e.g. helping identify pathological edge cases, similar to work @nnethercote has done manually in the past

We're focusing initially on the first use case since we see that as the most likely to have a significant impact. 

### The status quo

Currently the Rust compiler has the capability to store to disk a backtrace and additional runtime information whenever an ICE occurs. This is only enabled on nightly due to concerns around where this file is stored, and how the output includes the fully qualified path of the file, which normally includes the username for the user that executed `rustc`.

Additionally, our users can use Cargo's `--timings` flag and `rustc`'s `-Z self-profile` to generate reports on where compile times are going, but these are explicit opt-in actions, that produce output meant for direct human consumption, not for tool analysis.

For the uses of the perf dashboard, internal compiler aggregates can be collected, but lack granularity for complex analysis. These are currently only used to detect changes in behavior between two `rustc` builds.

All together these tools give us the ability to gather information about the inner workings of the compiler on a case by case basis, but any attempt to piece together trends within this information is often left as a manual process if not left undone entirely. This often leaves teams to guess at how people are using the language or to rely on proxies for that information.

### The next 6 months

*Sketch out the specific things you are trying to achieve in this goal period. This should be short and high-level -- we don't want to see the design!*

* Initial prototypes and proof of concept impls
    * initial metrics dumping in compiler e.g. unstable feature usage info
    * backend to store metrics
    * enable metrics dumping on existing project infra for open source crates (e.g. docs.rs or crater) and send metrics to backend
    * proof of concept dashboard for viewing metrics

### The "shiny future" we are working towards

We'd like to get to the point where lang and libs can pull up a simple dashboard to see exactly what features exist, what their status is, and what their usage over time looks like. Beyond that, we want to get to the point where other contributors and teams can leverage the metrics to answer their own questions while we continue to build up the supporting infrastructure. The metrics should make it possiblle to track how often certain ICEs are encountered or if certain code paths are hit or any other question about real world usage of the compiler that our contributors and maintainers may have.

## Design axioms

- **Trust**: Do not violate the trust of our users
  - **NO TELEMETRY, NO NETWORK CONNECTIONS**
  - Emit metrics **locally**
  - User information should **never leave their machine in an automated manner**; sharing their metrics should always be **opt-in**, **clear**, and **manual**.
  - All of this information would only be stored on disk, with some minimal retention policy to avoid wasteful use of users’ hard drives
- **Feedback**: improving feedback loops to assist with iterative improvement within the project
  - answer questions from real production environments in a privacy-preserving way
  - improve legibility of rare or intermittent issues
  - earlier warnings for ICEs and other major issues on nightly, improving the likelihood that we'd catch them before they hit stable.
	  - https://blog.rust-lang.org/2021/05/10/Rust-1.52.1.html
- **Performance impact**
  - leave no trace (minimize performance impact, particularly for default-enabled metrics)
- **Extensible**: 
  - it should be easy to add new metrics as needed
  - Only add metrics as a way to answer a specific question in mind, with an explicitly documented rationale
  - machine readable, it should be easy to leverage metrics for analysis with other tools
- **User experience**: 
  - improving user experience of reporting issues to the project
  - improving the user experience of using the compiler, measuring the impact of changes to user experience

[da]: ../about/design_axioms.md

## Ownership and team asks

| Task                                                                                   | Owner(s) or team(s)           | Notes |
|----------------------------------------------------------------------------------------|-------------------------------|-------|
| Discussion and moral support                                                           | ![Team][] [compiler], [infra] |       |
| Implementation                                                                         | @yaahc                        |       |
| backend for storing metrics                                                            | @estebank                     |       |
| integration with docs.rs or crates.io to gather metrics from open source rust projects | @yaahc                        |       |
| proof of concept dashboard visualizing unstable feature usage data                     | ![Help Wanted][]              |       |
| Standard reviews                                                                       | ![Team][] [compiler]          |       |


## Frequently asked questions