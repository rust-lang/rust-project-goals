# Expand the Rust Reference to specify more aspects of the Rust language

| Metadata           |                                    |
| :--                | :--                                |
| Point of contact   | @joshtriplett                      |
| Teams              | <!-- TEAMS WITH ASKS -->           |
| Task owners        | <!-- TASK OWNERS -->               |
| Status             | Proposed                           |
| Zulip channel      | [#t-spec][channel]                 |
| Tracking issue     | [rust-lang/rust-project-goals#NNN] |

[channel]: https://rust-lang.zulipchat.com/#narrow/channel/399173-t-spec

## Summary

The Rust Reference (https://doc.rust-lang.org/nightly/reference/) describes and
documents the Rust language. The Reference has coverage of many areas of Rust
already, and it has been integrated into language evolution processes, so that
new changes to the language get corresponding updates to the Reference.
However, there are some remaining gaps in the Reference's existing coverage of
Rust.

This project goal proposes to write new chapters for the Reference to fill
these gaps, making its coverage of Rust more comprehensive. This project goal
also proposes to provide additional review bandwidth for other changes, to
ensure that the new material does not overwhelm existing review capacity.

## Motivation

The Rust Reference is one of the core pieces of documentation for Rust. It
serves as documentation for users and implementers alike. The Reference should,
ideally, describe all aspects of the Rust language comprehensively. It is
easier to contribute changes to existing material than large swaths of new
material; the latter is a larger commitment. Thus, this project goal will
coordinate the efforts of several prospective contributors (who have already
been identified and lined up) to write, contribute, and merge new material for
the Reference.

Writing new material for the Reference does not suffice to get it merged,
however. Each change to the Reference requires review and approval, which
sometimes requires calling in additional expertise, and can require more and
scarcer resources. This project goal serves to request such review resources,
and additionally to coordinate contribution of additional review resources in
other areas to share the overall review load, in an effort to alleviate this
and not put as much additional load on existing contributors.

### The status quo

We consulted multiple experts on the Rust Reference, and based on that and on
analysis of the current Rust Reference, some of the key areas of Rust that the
Reference doesn't fully specify (and in some cases no documentation fully
specifies) are:

- The behavior of type inference, including both when the language can infer
  types and the boundaries and limitations of when it cannot.
- The trait solver (up to date with the new solver work by @lcnr). 
- More details on macros and expansion, in general. Macros-by-example
  (declarative `macro_rules!` macros) are mostly covered, but not things like
  the exact behavior of proc macros and the general process such as cfg
  pruning.
- Name resolution: the process by which the Rust compiler resolves a name used
  in Rust code, including methods of traits, and items imported from various
  modules in various crates. Name resolution includes various extensions and
  special cases that have been added over the years, and the reference should
  document those and their behavior.
- Completing documentation of const eval.

#### Out of scope

Other items, which are out of the scope of this project goal:

- The behavior of the borrow checker. This is extensive work that has been the
  subject of PhD theses. There have been multiple attempts to document and
  bound the behavior of the borrow checker, the latest of which is Tree Borrows
  ( https://www.ralfj.de/blog/2023/06/02/tree-borrows.html ) by Ralf Jung, a
  successor to the previous Stacked Borrows model. Specifying this will also
  carefully distinguish between description of the current behavior and
  bounds on future behavior, as the borrow checker likely will improve further
  in the future. 
- The operational semantics of the Rust language, which includes the work
  formerly described as "unsafe code guidelines". This has been the focus of
  the opsem team (https://github.com/rust-lang/opsem-team).

Both of these have substantial efforts already working on them. In some cases,
this work is as much about determining and negotiating the correct semantics as
documenting them. These two areas have been the work of years and are unlikely
to get completed or substantively accelerated in the goal period.

### The next 6 months

We propose to have a team of established Rust developers working on different
new chapters of the Reference, writing and contributing material for those
chapters. Along the way, we expect to make various other contributions based on
what turns up during the project period while re-reviewing existing material.
We hope to substantially close the gap between the Reference and the Rust language.

The developers working on this project goal have expert-level knowledge of
multiple areas of Rust, and in several cases, directly worked on the portions
of Rust that need documenting.

We're expecting to coordinate contributions of new reference material from
@lcnr, @jackh726, @Amanieu, @GuillaumeGomez, and @yaahc.

Additional people, who may provide review, mentoring, support, and drafting,
include @joshtriplett, @lcnr, @jackh726, @Amanieu, @GuillaumeGomez, @m-ou-se,
@petrochenkov, @yaahc, @lucarlig, @midiareshadi, @borsakv, and @SparrowLii.

### The "shiny future" we are working towards

We hope to continue providing ongoing support for reference updates in our
areas of expertise, as well as making it easier for others to make ongoing
contributions by providing a starting point. While we don't expect the
Reference to ever be "done" (much as the Rust language will not be), we hope to
substantially close the gap between the Reference and the language.

In the future, we hope that the reference will include full and complete
documentation for all parts of the Rust language, including the borrow checker
and the operational semantics.

## Design axioms

The following [design axioms][da] apply:
* Some documentation may be better suited for the rustc-dev-guide rather than the Rust Reference. If we find ourselves with material better suited for the rustc-dev-guide, we can submit it there, and submit appropriate user-focused subsets of it to the Rust Reference.
* Large changes all at once can be harder to review. When possible, we should endeavor to share outlines of new material in advance, and make sure we're broadly aligned, before writing and submitting full prose.
* When writing down descriptions of Rust based on the observed current behavior of `rustc`, we sometimes encounter areas where we're not entirely sure whether we want to guarantee the observed behavior as the behavior of Rust. There are two cases:  
  1. We're unhappy with the current behavior of some edge cases. Here, we will prefer to document the current user-visible behavior while leaving an appropriate disclaimer in the Reference (following conventions of the Reference) that there are open questions about whether we may wish to change this behavior.
  2. We're uncertain about whether the behavior represents a stable guarantee or simply a description of one of many behaviors that Rust could validly exhibit. Here, we will describe the current behavior in an explicitly non-normative fashion (following conventions of the Reference), noting specifically that this does not represent a Rust language guarantee and that other behaviors are possible.
* Sometimes, writing things down may uncover behaviors that are clearly bugs in `rustc` (e.g., because those behaviors would be unsound, would contradict other settled language in the Reference, or are otherwise just obviously wrong) and where the correct behavior is obvious (e.g. because it's implied by other text in the Reference, by existing lang RFCs, due to being the only choice, etc.). In these cases, we'll prefer to normatively document the correct behavior, to file the relevant bug report in `rust-lang/rust`, and if appropriate (e.g. because we expect the bug to be longstanding) to add a disclaimer in the Reference (in the standard format) describing the incorrect behavior and citing the bug report.
* Other times, we'll uncover behaviors where either 1) it's unclear whether or not the behavior represents a bug in `rustc` or 2) it probably does represent a bug in `rustc` but it's unclear what the correct behavior is. In these cases, we'll prefer to non-normatively document the current behavior (in the standard way for the Reference), to file an issue in `rust-lang/rust` asking the lang team (potentially along with one of its subteams, as appropriate) to make a decision about what the correct behavior is, and to cite that issue in the Reference.
* Tests demonstrating the behavior being documented, whether as `rust-lang/rust` tests or as tested reference examples, can provide value in making a change easier to review and validate.
* We should be explicit about any material that is primarily descriptive rather than normative.
* We should keep an eye out for potential process improvements that may make it easier to maintain the Reference in the future.
  * Jack Huey (@jackh726) will work with T-types and T-compiler on potential process improvements to loop in the Reference when making relevant changes as part of those teams.

[da]: ../about/design_axioms.md

## Ownership and team asks

**Owner:** @joshtriplett, in his capacity as `t-lang` and `t-spec` team member,
will lead this project goal.

| Task                               | Owner(s) or team(s)            | Notes                           |
|------------------------------------|--------------------------------|---------------------------------|
| Discussion and moral support       | ![Team][] [spec][]             | |
| New reference chapters             | @lcnr, @jackh726, @Amanieu, @GuillaumeGomez, @yaahc | |
| Standard reviews                   | @joshtriplett, @lcnr, @jackh726, @Amanieu, @GuillaumeGomez, @m-ou-se, @petrochenkov, @yaahc | |
| Standard reviews                   | ![Team][] [spec][] [lang-docs][]            | |

## Frequently asked questions
