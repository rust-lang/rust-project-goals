# SVE and SME on AArch64

| Metadata            |                                    |
| :--                 | :--                                |
| Point of contact    | @davidtwco                         |
| Status              | Proposed                           |
| Zulip channel       | N/A                                |
| Tracking issue      | [rust-lang/rust-project-goals#270] |
| [compiler] champion | @davidtwco                         |
| [lang] champion     | @nikomatsakis                      |
| [libs] champion     | @Amanieu                           |

## Summary

Over the next six months, we will continue our efforts from the 2025H1 goal to
merge nightly support for SVE and establish a path towards stabilisation:

- propose language changes which will enable scalable vector types to be
  represented in Rust's type system
- land an experimental nightly implementation of SVE
- identify remaining blockers for SVE stabilisation and plan their resolution
- gain a better understanding of SME's implications for Rust and identify first
  steps towards design and implementation

## Motivation

AArch64 is an important architecture for Rust, with two tier 1 targets and over
thirty targets in lower tiers. It is widely used by some of Rust's largest
stakeholders and as a systems language, it is important that Rust is able to
leverage all of the hardware capabilities provided by the architecture,
including new SIMD extensions: SVE and SME.

SIMD types and instructions are a crucial element of high-performance Rust
applications and allow for operating on multiple values in a single instruction.
Many processors have SIMD registers of a known fixed length and provide
intrinsics which operate on these registers. Arm's Neon extension is
well-supported by Rust and provides 128-bit registers and a wide range of
intrinsics.

Instead of releasing more extensions with ever increasing register bit widths,
recent versions of AArch64 have a Scalable Vector Extension (SVE), with vector
registers whose width depends on the CPU implementation and bit-width-agnostic
intrinsics for operating on these registers. By using SVE, code won't need to be
re-written using new architecture extensions with larger registers, new types
and intrinsics, but instead will work on newer processors with different vector
register lengths and performance characteristics.

SVE has interesting and challenging implications for Rust, introducing value
types with sizes that can only be known at compilation time, requiring
significant work on the language and compiler. Arm has since introduced Scalable
Matrix Extensions (SME), building on SVE to add new capabilities to efficiently
process matrices, with even more interesting implications for Rust.

Hardware is generally available with SVE, and key Rust stakeholders want to be
able to use these architecture features from Rust. In a recent discussion on
SVE, [Amanieu, co-lead of the library team, said][quote_amanieu]:

> I've talked with several people in Google, Huawei and Microsoft, all of whom
> have expressed a rather urgent desire for the ability to use SVE intrinsics in
> Rust code, especially now that SVE hardware is generally available.

While SVE is specifically an AArch64 extension, the infrastructure for scalable
vectors in Rust should also enable Rust to support for RISC-V's "V" Vector
Extension, and this goal will endeavour to extend Rust in an
architecture-agnostic way. SVE is supported in C through Arm's C Language
Extensions (ACLE) but requires a change to the C standard (documented in [pages
122-126 of the 2024Q3 ACLE][acle_sve]), so Rust has an opportunity to be the
first systems programming language with native support for these hardware
capabilities.

[acle_sve]: https://github.com/ARM-software/acle/releases/download/r2024Q3/acle-2024Q3.pdf
[quote_amanieu]: https://github.com/rust-lang/rust/pull/118917#issuecomment-2202256754

### The status quo

SVE is currently entirely unsupported by Rust, but progress is being made - see the tracking
issues for the Sized Hierarchy prerequisite ([rust#144404]) and for Scalable Vectors
themselves ([rust#145052]).

[rust#144404]: https://github.com/rust-lang/rust/issues/144404
[rust#145052]: https://github.com/rust-lang/rust/issues/145052

### The next 6 months

The primary objective of this initial goal is to land a nightly experiment with
SVE, have both RFCs accepted and establish a path towards stabilisation:

1. Resolve any and all follow-ups to [rust#137944] now that it has been merged
2. Merge a pull request with Part II of [rust#3729]'s implementation
3. Rewrite [rfcs#3838] in light of [rfcs#3729] to defer to [rfcs#3729]'s
   solutions for how scalable vectors will fit in the Rust type system
4. Rebase [rust#143924] on top of Part II of the Sized Hierarchy work, enabling
   it to be merged experimentally
5. Merge [stdarch#1509] adding SVE types and intrinsics
6. Identify and start addressing remaining stabilisation blockers

[rust#143924]: https://github.com/rust-lang/rust/pull/143924
[rust#137944]: https://github.com/rust-lang/rust/pull/137944
[rfcs#3838]: https://github.com/rust-lang/rfcs/pull/3838
[rfcs#3729]: https://github.com/rust-lang/rfcs/pull/3729
[stdarch#1509]: https://github.com/rust-lang/stdarch/pull/1509

### The "shiny future" we are working towards

Adding support for Scalable Matrix Extensions in Rust is the next logical step
following SVE support. There are still many unknowns regarding what this will
involve and part of this goal or the next goal will be understanding these
unknowns better.

## Design axioms

- **Avoid overfitting.** It's important that whatever extensions to Rust's type
  system are proposed are not narrowly tailored to support for SVE/SME, can be
  used to support similar extensions from other architectures, and unblocks or
  enables other desired Rust features wherever possible and practical.
- **Low-level control.** Rust should be able to leverage the full capabilities
  and performance of the underlying hardware features and should strive to avoid
  inherent limitations in its support.
- **Rusty-ness.** Extensions to Rust to support these hardware capabilities
  should align with Rust's design axioms and feel like natural extensions of the
  type system.

## Ownership and team asks

Here is a detailed list of the work to be done and who is expected to do it.
This table includes the work to be done by owners and the work to be done by
Rust teams (subject to approval by the team in an RFC/FCP).

| Task                         | Owner(s) or team(s)                           | Notes                                                      |
| ---------------------------- | --------------------------------------------- | ---------------------------------------------------------- |
| Discussion and moral support | ![Team][] [lang], [types], [compiler], [libs] |                                                            |
| Implementation               | @davidtwco                                    | Part II of Sized Hierarchy implementation                  |
| Dedicated reviewer           | ![Team][] [types]                             | Review Part II of Sized Hierarchy implementation           |
| Author RFC                   | @davidtwco                                    | Write [rfcs#3838] in light of [rfcs#3729]                  |
| Lang-team experiment         | ![Team][] [libs], [compiler]                  | Approve experiment of [rfcs#3838]                          |
| Implementation               | @davidtwco                                    | Update [rust#143924] after Part II                         |
| Standard reviews             | ![Team][] [compiler]                          | Review and approve [rust#143924]                           |
| Implementation               | @davidtwco                                    | Update [stdarch#1509] with SVE types/intrinsics            |
| Standard reviews             | ![Team][] [libs]                              | Review and approve [stdarch#1509]                          |
| RFC decision                 | ![Team][] [lang]                              | Language team decide whether to accept [rfcs#3729]         |
| RFC decision                 | ![Team][] [lang]                              | Compiler/Library team decide whether to accept [rfcs#3838] |

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
