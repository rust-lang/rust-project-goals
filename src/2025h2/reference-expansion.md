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
* Behavior of Rust being documented in the Rust Reference does not prevent it from changing in the future. Documenting behavior in the Reference does not inherently change a two-way door to a one-way door, though it *can* contribute to people relying on behavior or to behavior otherwise ossifying.
  * While we do not want to over-constrain the future evolution of Rust, if we do unintentionally document behavior we don't want to specify or require, we haven't necessarily created a *new* stability guarantee by doing so, and we can still evolve Rust and fix bugs.
  * Attempting to document something may result in discovering issues with the way it works. If that happens, report it, but don't let attempting to fix it block documenting its current behavior (potentially with appropriate disclaimers to support future changes).
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
| Standard reviews                   | ![Team][] [spec][]             | |

## Frequently asked questions
