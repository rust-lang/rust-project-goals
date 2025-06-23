# Cargo Caching Improvements

| Metadata         |                                                                                  |
|:-----------------|----------------------------------------------------------------------------------|
| Point of contact | @ranger-ross                                                                     |
| Teams            | &lt;!-- TEAMS WITH ASKS --&gt;                                                   |
| Task owners      | &lt;!-- TASK OWNERS --&gt;                                                       |
| Status           | Proposed                                                                         |
| Tracking issue   | *if this is a continuing goal, add the old tracking issue, else leave blank*     |
| Zulip channel    | N/A (an existing stream can be re-used or new streams can be created on request) |

## Summary

Rework the Cargo build directory layout to enable a shared build cache across multiple workspaces.


## Motivation

The primary goal of this effort is to improve build times by reusing builds across projects.

Secondary goals are

* Reduce disk usage of build artifacts
* More precise cross-job caching in CI

### The status quo

When Cargo performs a build, it will build the package you requested and all
dependencies individually, linking them in the end.
These build results (intermediate build artifacts) and the linked result (final
build artifact) are stored in the target-dir, which is per-workspace by
default.

Ways cargo will try to reuse builds today:
- On a subsequent build, Cargo tries to reuse these build results by
  "fingerprinting" the inputs to the prior build and checking if that
  fingerprint has changed.
- When dependencies are shared by host (`build.rs`, proc-macros) and
  platform-target and the platform-target is the host, Cargo will attempt to
  share host/target builds

Some users try to get extra cache reuse by assigning all workspaces to use the same target-dir.
- Cross-project conflicts occur because this shares both intermediate (generally unique) and final build artifacts (might not be unique)
- `cargo clean` will clear the entire cache for every project
- Rebuild churn from build inputs, like `RUSTFLAGS`, that cause a rebuild but aren't hashed into the file path

In CI, users generally have to declare what directory should be cached between jobs.
This directory will be compressed and uploaded at the end of the job.
If the next job's cache key matches, the tarball will be downloaded and decompressed.
If too much is cached, the time for managing the cache can dwarf the benefits of the cache.
Some third-party projects exist to help manage cache size.

### The next 6 months

* Stabilize Cargo `build-dir` which allows users to move intermediate build artifacts out of `target` (rust-lang/cargo#14125)
* Rework the build directory layout so that we have more easily cacheable "units" (rust-lang/cargo#15010)
* Work towards a user wide build cache that can be reused across projects/workspaces. (rust-lang/cargo#5931)
* Explore possibilities of extending this build cache to support remote storage enabling continuous integration use cases.

### The "shiny future" we are working towards

The cache lookup will be extended with plugins to read and/or write to different sources. Open source projects and companies can have their CI read from and write to their cache. Individuals who trust the CI can then configure their plugin to read from the CI cache.

A cooperating CI service could provide their own plugin that, instead of caching everything used in the last job and unpacking it in the next, their plugin could download only the entries that will be needed for the current build (e.g. say a dependency changed) and only upload the cache entries that were freshly built. Fine-grained caching like this would save the CI service on bandwidth, storage, and the compute time from copying, decompressing, and compressing the cache. Users would have faster CI time and save money on their CI service, minus any induced demand that faster builds creates.


## Ownership and team asks

| Task                         | Owner(s) or team(s) | Notes |
|------------------------------|---------------------|-------|
| Standard reviews             | ![Team][] [cargo]   |       |
| Implementation               | @ranger-ross        |       |


### Definitions

For definitions for terms used above, see the [About > Team Asks](https://rust-lang.github.io/rust-project-goals/about/team_asks.html) page.

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

### Why not pre-built packages?

Pre-built packages requires guessing
- CPU Architecture
- Feature flags
- RUSTFLAGS
- Dependency versions

If there are any mismatches there, then the pre-built package can't be used.

A build cache can be populated with pre-built packages and react to the unique circumstances of the user.

### Why not sccache?

Tools like sccache try to infer inputs for hashing a cache key from command-line arguments.
This has us reusing the extra knowledge Cargo has to get more accurate cache key generation.
