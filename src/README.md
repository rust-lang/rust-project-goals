# Project goals

This repo tracks the effort to set and track goals for the Rust project.

## Current goal period (2025H2)

The 2025H2 goal period runs from Sept 1 to Dec 31. We have identified 12 flagship goals, broken out into four themes:

* [Beyond the `&`](#beyond-the-), making it possible to create user-defined smart pointers that are as ergonomic as Rust's built-in references `&`.
* [Unblocking dormant traits](#unblocking-dormant-traits), extending the core capabilities of Rust's trait system to unblock long-desired features for language interop, lending iteration, and more.
* [Flexible, fast(er) compilation](#flexible-faster-rust-compilation), making it faster to build Rust programs and improving support for specialized build scenarios like embedded usage and sanitizers.
* [Higher-level Rust](#higher-level-rust), making higher-level usage patterns in Rust easier.

### "Beyond the `&`"

| Goal                                                                         | Point of contact | Team(s) and Champion(s)                      |
| :--                                                                          | :--          | :--                                          |
| [Reborrow traits](https://rust-lang.github.io/rust-project-goals/2025h2/autoreborrow-traits.html)                                    | [Aapo Alasuutari][]    | [compiler] ([Oliver Scherer][]), [lang] ([Tyler Mandry][])     |
| [Design a language feature to solve Field Projections](https://rust-lang.github.io/rust-project-goals/2025h2/field-projections.html) | [Benno Lossin][] | [lang] ([Tyler Mandry][])                            |
| [Continue Experimentation with Pin Ergonomics](https://rust-lang.github.io/rust-project-goals/2025h2/pin-ergonomics.html)            | [Frank King][]  | [compiler] ([Oliver Scherer][]), [lang] ([TC][]) |


One of Rust's core value propositions is that it's a "library-based language"—libraries can build abstractions that feel built-in to the language even when they're not. Smart pointer types like `Rc` and `Arc` are prime examples, implemented purely in the standard library yet feeling like native language features. However, Rust's built-in reference types (`&T` and `&mut T`) have special capabilities that user-defined smart pointers cannot replicate. This creates a "second-class citizen" problem where custom pointer types can't provide the same ergonomic experience as built-in references.

The "Beyond the `&`" initiative aims to share `&`'s special capabilities, allowing library authors to create smart pointers that are truly indistinguishable from built-in references in terms of syntax and ergonomics. This will enable more ergonomic smart pointers for use in cross-language interop (e.g., references to objects in other languages like C++ or Python) and for low-level projects like Rust for Linux which use smart pointers to express particular data structures.

### "Unblocking dormant traits"

| Goal                                                    | Point of contact | Team(s) and Champion(s)                                        |
| :--                                                     | :--       | :--                                                            |
| [Evolving trait hierarchies](https://rust-lang.github.io/rust-project-goals/2025h2/evolving-traits.html)        | [Taylor Cramer][] | [compiler], [lang] ([Taylor Cramer][]), [libs-api], [types] ([Oliver Scherer][]) |
| [In-place initialization](https://rust-lang.github.io/rust-project-goals/2025h2/in-place-initialization.html)   | [Alice Ryhl][] | [lang] ([Taylor Cramer][])                                             |
| [Next-generation trait solver](https://rust-lang.github.io/rust-project-goals/2025h2/next-solver.html)          | [lcnr][]     | [types] ([lcnr][])                                                |
| [Stabilizable Polonius support on nightly](https://rust-lang.github.io/rust-project-goals/2025h2/polonius.html) | [Rémy Rakic][]      | [types] ([Jack Huey][])                                            |


Rust's trait system is one of its most powerful features, but it has a number of longstanding limitations that are preventing us from adopting new patterns. The goals in this category unblock a number of new capabilities:

* [Polonius](https://rust-lang.github.io/rust-project-goals/2025h2/./polonius.html) will enable new borrowing patterns, and in particular [unblock "lending iterators"](https://github.com/rust-lang/rust/issues/92985). Over the last few goal periods we have identified an "alpha" vesion of polonius that addresses the most important cases while being relatively simple and optimizable. Our goal for 2025H2 is to implement this algorithm in a form that is ready for stabilization in 2026.
* The [next gen trait solver](https://rust-lang.github.io/rust-project-goals/2025h2/./next-solver.html) is a refactored trait solver that unblocks better support for numerous language features (implied bounds, negative impls, the list goes on) in addition to closing a number of existing bugs and unsoundnesses. Over the last few goal periods, the trait solver went from early prototype to being production use in coherence. The goal for 2025H2 is to prepare it for stabilization.
* The work on [evolving trait hierarchies](https://rust-lang.github.io/rust-project-goals/2025h2/./evolving-traits.html) will make it possible to refactor some parts of an existing trait out into a new supertrait so they can be used on their own. This unblocks a number of features where the existing trait is insufficiently general, in particular stabilizing support for custom receiver types, a prior project goal that wound up blocking on this refactoring. This will also make it safer to provide stable traits in the standard library, while preserving the ability to evolve them in the future.
* The work to [expand Rust's `Sized` hierarchy](https://rust-lang.github.io/rust-project-goals/2025h2/./scalable-vectors.html) will permit us to express types that are neither `Sized` nor `?Sized`, such as extern types (which have no size) or ARM's Scalable Vector Extensions (which have a size that is known at runtime, but not compilation time). This goal builds on [RFC #3729] and [RFC #3838], authored in previous project goal periods.
* [In-place initialization](https://rust-lang.github.io/rust-project-goals/2025h2/./in-place-initialization.html) allows creating structs and values that are tied to a particular place in memory. While useful directly for projects doing advanced C interop, it also unblocks expanding `dyn Trait` to support for `async fn` and `-> impl Trait` methods, as compiling such methods requires the ability for the callee to return a future whose size is not known to the caller.

### "Flexible, fast(er) compilation"

| Goal                                                                | Point of contact | Team(s) and Champion(s)                                      |
| :--                                                                 | :--         | :--                                                          |
| [build-std](https://rust-lang.github.io/rust-project-goals/2025h2/build-std.html)                                           | [David Wood][]  | [cargo] ([Eric Huss][]), [compiler] ([David Wood][]), [libs] ([Amanieu d'Antras][]) |
| [Promoting Parallel Front End](https://rust-lang.github.io/rust-project-goals/2025h2/parallel-front-end.html)               | [Sparrow Li][] | [compiler]                                                   |
| [Production-ready cranelift backend](https://rust-lang.github.io/rust-project-goals/2025h2/production-ready-cranelift.html) | [Folkert de Vries][] | [compiler], [wg-compiler-performance]                        |


The "Flexible, fast(er) compilation" initiative focuses on improving Rust's build system to better serve both specialized use cases and everyday development workflows:

* We are improving compilation performance through (1) [parallel compilation in the compiler front-end](https://rust-lang.github.io/rust-project-goals/2025h2/./parallel-front-end.html), which delivers 20-30% faster builds, and (2) [making the Cranelift backend production-ready for development use](https://rust-lang.github.io/rust-project-goals/2025h2/./production-ready-cranelift.html), offering roughly 20% faster code generation compared to LLVM for debug builds.
* We are working to [stabilize a core MVP of the `-Zbuild-std` feature](https://rust-lang.github.io/rust-project-goals/2025h2/./build-std.html), which allows developers to rebuild the standard library from source with custom compiler flags. This unblocks critical use cases for embedded developers and low-level projects like Rust for Linux, while also enabling improvements like using sanitizers with the standard library or building `std` with debug information.

### "Higher-level Rust"

| Goal                                                                | Point of contact | Team(s) and Champion(s)                                                           |
| :--                                                                 | :--           | :--                                                                               |
| [Stabilize cargo-script](https://rust-lang.github.io/rust-project-goals/2025h2/cargo-script.html)                           | [Ed Page][]        | [cargo] ([Ed Page][]), [compiler], [lang] ([Josh Triplett][]), [lang-docs] ([Josh Triplett][]) |
| [Ergonomic ref-counting: RFC decision and preview](https://rust-lang.github.io/rust-project-goals/2025h2/ergonomic-rc.html) | [Niko Matsakis][] | [compiler] ([Santiago Pastorino][]), [lang] ([Niko Matsakis][])                                  |


People generally start using Rust for foundational use cases, where the requirements for performance or reliability make it an obvious choice. But once they get used to it, they often find themselves turning to Rust even for higher-level use cases, like scripting, web services, or even GUI applications. Rust is often "surprisingly tolerable" for these high-level use cases -- except for some specific pain points that, while they impact everyone using Rust, hit these use cases particularly hard. We plan two flagship goals this period in this area:

* We aim to stabilize [cargo script](https://rust-lang.github.io/rust-project-goals/2025h2/./cargo-script.html), a feature that allows single-file Rust programs that embed their dependencies, making it much easier to write small utilities, share code examples, and create reproducible bug reports without the overhead of full Cargo projects.
* We aim to finalize the design of [ergonomic ref-counting](https://rust-lang.github.io/rust-project-goals/2025h2/./ergonomic-rc.html) and to finalize the experimental impl feature so it is ready for beta testing. Ergonomic ref counting makes it less cumbersome to work with ref-counted types like `Rc` and `Arc`, particularly in closures.

[The full list of 2025H2 goals is available here.](./2025h2/goals.md) We author monthly blog posts about our overall status, but you can also follow the tracking issue for a [particular goal](./2025h2/goals.md) to get updates specific to that goal.

[cargo]: https://github.com/rust-lang/cargo
[clippy]: https://github.com/rust-lang/rust-clippy
[compiler]: http://github.com/rust-lang/compiler-team
[crates-io]: https://github.com/rust-lang/crates.io
[docs-rs]: https://github.com/rust-lang/docs.rs
[edition]: http://github.com/rust-lang/edition-team
[infra]: https://github.com/rust-lang/infra-team
[lang]: http://github.com/rust-lang/lang-team
[leadership-council]: https://github.com/rust-lang/leadership-council
[libs]: https://github.com/rust-lang/libs-team
[libs-api]: https://www.rust-lang.org/governance/teams
[miri]: https://github.com/rust-lang/miri
[opsem]: https://github.com/rust-lang/opsem-team
[ospp]: https://www.rust-lang.org/governance/teams
[rust-analyzer]: https://github.com/rust-lang/rust-analyzer
[rust-for-linux]: https://www.rust-lang.org/governance/teams
[rustdoc]: https://github.com/rust-lang/rust
[rustfmt]: https://github.com/rust-lang/rustfmt
[rustup]: https://github.com/rust-lang/rustup
[spec]: https://github.com/rust-lang/spec
[style]: https://github.com/rust-lang/style-team
[types]: https://github.com/rust-lang/types-team
[Bastian Kersting]: https://github.com/1c3t3a
[Amanieu d'Antras]: https://github.com/Amanieu
[Benno Lossin]: https://github.com/BennoLossin
[Boxy]: https://github.com/BoxyUwU
[Alice Ryhl]: https://github.com/Darksonn
[Guillaume Gomez]: https://github.com/GuillaumeGomez
[James]: https://github.com/Jamesbarford
[Pete LeVasseur]: https://github.com/PLeVasseur
[Ralf Jung]: https://github.com/RalfJung
[Sparrow Li]: https://github.com/SparrowLii
[Wesley Wiser]: https://github.com/WesleyWiser
[Manuel Drehwald]: https://github.com/ZuseZ4
[Aapo Alasuutari]: https://github.com/aapoalas
[Alona Enraght-Moony]: https://github.com/adotinthevoid
[b-naber]: https://github.com/b-naber
[Jon Bauman]: https://github.com/baumanj
[Boxy]: https://github.com/boxyuwu
[Carol Nichols]: https://github.com/carols10cents
[Taylor Cramer]: https://github.com/cramertj
[David Wood]: https://github.com/davidtwco
[Ding Xiang Fei]: https://github.com/dingxiangfei2009
[David Tolnay]: https://github.com/dtolnay
[Eric Huss]: https://github.com/ehuss
[Ed Page]: https://github.com/epage
[Folkert de Vries]: https://github.com/folkertdev
[Frank King]: https://github.com/frank-king
[Ian McCormack]: https://github.com/icmccorm
[Jack Huey]: https://github.com/jackh726
[Jakob Koschel]: https://github.com/jakos-sec
[Josh Triplett]: https://github.com/joshtriplett
[Jack Wrenn]: https://github.com/jswrenn
[Jakub Beránek]: https://github.com/kobzol
[lcnr]: https://github.com/lcnr
[Rémy Rakic]: https://github.com/lqd
[Marco Ieni]: https://github.com/marcoieni
[Niko Matsakis]: https://github.com/nikomatsakis
[Predrag Gruevski]: https://github.com/obi1kenobi
[Oliver Scherer]: https://github.com/oli-obk
[Vadim Petrochenkov]: https://github.com/petrochenkov
[Ross Sullivan]: https://github.com/ranger-ross
[Ben Kimock]: https://github.com/saethlin
[Scott McMurray]: https://github.com/scottmcm
[Santiago Pastorino]: https://github.com/spastorino
[Tyler Mandry]: https://github.com/tmandry
[Tomas Sedovic]: https://github.com/tomassedovic
[TC]: https://github.com/traviscross
[Weihang Lo]: https://github.com/weihanglo
[Jane Lusby]: https://github.com/yaahc

## Next goal period (2026H1)

The next goal period will be 2026H1. We will start the process of assembling goals soon. If you'd like to propose a goal, [instructions can be found here](./how_to/propose_a_goal.md).

## About the process

Want to learn more? Check out some of the following:

* [RFC #3614, which describes the overall goals and plan](https://github.com/rust-lang/rfcs/blob/master/text/3614-project-goals.md)
* [How to propose a goal of your own](./how_to/propose_a_goal.md)
* [What it means to be a goal point of contact](./about/owners.md)
