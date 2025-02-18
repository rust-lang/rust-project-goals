# build-std

| Metadata           |                                    |
| :--                | :--                                |
| :----------------- | ------------------                 |
| Point of contact   | @davidtwco                         |
| Teams              | <!-- TEAMS WITH ASKS -->           |
| Task owners        | <!-- TASK OWNERS -->               |
| Status             | Proposed                           |
| Zulip channel      | N/A                                |
| Tracking issue     | [rust-lang/rust-project-goals#274] |

*Arm's Rust team is @davidtwco, @adamgemmell, @jacobbramley, @JamieCunliffe and @Jamesbarford. This
goal will be primarily worked on by @adamgemmell, but @davidtwco can always be contacted for
updates.*

## Summary

Write an RFC for a minimum viable product (MVP) of build-std which has the potential to be
stabilised once implemented (as opposed to the currently implemented MVP which is only suitable for
experimentation and testing), and then implement it.

## Motivation

build-std is a well-known unstable feature in Cargo which enables Cargo to re-build the standard
library, this is useful for a variety of reasons:

- Building the standard library for targets which do not ship with a pre-compiled standard library.
- Optimising the standard library for known hardware, such as with non-baseline target features
  or options which optimise for code size. This is a common use case for embedded developers.
- Re-building the standard library with different configuration options (e.g. changing the
  optimisation level, using flags which change the ABI, or which add additional exploit
  mitigations).
- Re-building the standard library with different `cfg`s (e.g. disabling `backtrace` in std), to
  the extent that such configurations are supported by the standard library.
- Stabilisation of various compiler flags which change the ABI, add additional exploit
  mitigations (such as `-Zsanitizers=cfi` or `-Zbranch-protection`), or which otherwise only make
  sense to use when the entire program is compiled with the flag (including std) is blocked on
  these being unable to be used properly without being able to rebuild std.

These features are more useful for some subsets of the Rust community, such as embedded developers
where optimising for size can be more important and where the targets often don't ship with a
pre-compiled std.

The fifty-thousand foot view of the work involved in this feature is:

- Having the standard library sources readily available that match the compiler.
- Being able to build those sources without using a nightly toolchain, which has many
  possible solutions.
- Having a blessed way to build at least `core` without Cargo, which some users like
  Rust for Linux would like.
  - This would be optional but may be a side-effect of whatever mechanism for build-std
    the MVP RFC eventually proposes.
- Being able to tell the compiler to use the resulting prebuilt standard library sources
  instead of the built-in standard library, in a standard way.
- Integrating all of the above into Cargo.
- Making sure all of this works for targets that don't have a pre-built std.

Rust for Linux and some other projects have a requirement to build core themselves without Cargo
(ideally using the same stable compiler they use for the rest of their project), which is a shared
requirement with build-std, as whatever mechanism these projects end up using could be re-used by
the implementation of build-std and vice-versa.

### The status quo

build-std is currently an unstable feature in Cargo which hasn't seen much development or progress
since its initial development in 2019/2020. There are a variety of issues in the
[wg-cargo-std-aware][wg-cargo-std-aware] repository which vary from concrete bugs in the current
experimental implementation to vague "investigate and think about this" issues, which make the
feature difficult to make progress on. 

Some of the work required for this exists in the current perma-unstable `-Zbuild-std`
implementation, which may be re-used if appropriate.

Prior to the submission of this goal, this goal has been discussed with the cargo team and
leads of the compiler and library teams, ensuring that this goal's owners have liaisons from
stakeholder teams and the support of the primary teams involved in the design and
implementation.

[wg-cargo-std-aware]: https://github.com/rust-lang/wg-cargo-std-aware

### The next 6 months

There are two primary objectives of this goal in its first six months:

- Firstly, we will write an MVP RFC that will limit the scope of the feature and make it easier
  to make progress on build-std.

  It is intended that this RFC will summarize all of the previous discussion, use cases and
  feedback on build-std. In this documenting of the current state of build-std, this RFC
  will be well-positioned to propose which use cases should and should not be resolved by
  build-std (for the final feature, not just this MVP). For example, this RFC will decide
  whether patching or modifying std is a supported use case for build-std.

  For those use cases solved by build-std, the RFC will select a subset for the new MVP of
  build-std. It is intended that this MVP be sufficiently useful and complete that it could
  be stabilised. The design of the MVP will be forward-compatible with all of the other use
  cases that build-std is intended to solve.

  It is hoped that this RFC should demonstrate a thorough understanding of the design space
  of build-std and give the responsible upstream teams confidence in our ownership of this
  feature, and enabling those teams to make a fully informed decision on any proposals made.

- Next, after and conditional on acceptance of this RFC, we will proceed with its
  implementation.

### The "shiny future" we are working towards

After the approval and implementation of the MVP RFC, there will naturally be follow-up use cases
which can be designed and implemented to complete the build-std feature.

## Design axioms

- Enabling build-std without changing any compilation options or configuration should produce an
  equivalent library to that distributed by the project.
- Avoid precluding future extensions to build-std.
- build-std should allow std/alloc/core to be treated more like other dependencies than currently.
  - This represents a general move away from treating std/alloc/core as a special case.

## Ownership and team asks

| Task                         | Owner(s) or team(s) | Notes |
|------------------------------|---------------------|-------|
| Discussion and moral support | ![Team][] [cargo]   |       |
| Author RFC                   | @adamgemmell        |       |
| Implementation               | @adamgemmell        |       |
| Standard reviews             | ![Team][] [cargo]   |       |

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

None yet.