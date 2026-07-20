# Binary size reduction

| Metadata | |
|:--|--|
| Short title | Binary size reduction |
| What and why | Easily building smaller binaries with only what's needed |
| Point of contact | @nia-e |

## Summary

Allowing users to build smaller, leaner binaries makes it more practical to deploy Rust to resource-constrained environments such as embedded systems and containers.

## Motivation

### The status quo

Many parts of Rust's build architecture treat build time and runtime speed as the only first-class objectives. Usually, that's true, with reasonable binary sizes coming naturally out of performance considerations - but in many other cases, there are huge leaps to be made in size reduction if we're willing to slightly compromise on those other points. This is something that should be left to users to decide, with certain environments - web containers, embedded, wasm, kernels - being often willing to tolerate losing some runtime conveniences or performance for the sake of smaller binaries.

Rust has also garnered an unfortunate reputation as a language with very large binaries. While ways exist to mitigate this, some aspects are simply impossible to express at build time; whole mechanisms in the language are impossible to strip out or replace with native platform versions (backtrace machinery), or require using unstable flags (build-std).

### Design axioms

* *Keep it simple, stupid.* Cutting binary size should, where possible, be as simple as a config change; it shouldn't require changing code or using unstable features.

* *Win in multiple areas.* Most features that would benefit binary sizes also are helpful elsewhere or enable other usecases too; these can benefit from preexisting efforts, and we can achieve a lot by just ensuring their design takes binary size reduction into account instead of being ground-up focused on it.

* *Document, document, document.* Discoverability has so far suffered significantly, with downstream unofficial guides on cutting sizes being the go-to resource. Many relevant features are only documented with regard to their original intended usecases, despite being extremely relevant for binary sizes as well; we should make it clear when something is size-relevant the same way we do for performance relevance.

### What we are shooting for

A language that makes size reduction as easy to achieve as performance maximisation, with compiler, cargo, and library documentation that clearly spell out size considerations for relevant features and stable first-class ways to strip out or replace unnecessary parts of the language.

### How we get there

| Goal | Timespan | What and why |
| --- | --- | --- |
| (((ROADMAP ROWS: Binary size reduction))) |
| Alternate std implementations | Future | Enable transparent use of size-optimised implementations of features in the standard library when size optimisations are enabled |
| Overhaul size flags | Future | Add proper analysis in the compiler for size-sensitive optimisations, instead of the current approach of having a small handful of peephole optimisations that relate to size |
| More dynamic linking | Future | Expand the guarantees of some extern ABI to allow dynamically linking more code |
| Alternatives to monomorphization | Future | Offer a transparent mechanism for polymorphic compilation of generics |
| Separate runtime machinery | Future | Split out bundled runtime such as the machinery for backtrace handling and formatting into their own libraries, to allow optionally dynamically linking to a different implementation if available |
| More devirtualization | Future | Replace dyn traits with concrete types even outside of fat LTO to minimise code bloat, especially on embedded targets |
| Better automatic outlining | Future | Explore options for function outlining, either through work on making LLVM's outliner usable in Rust or MIR-based outlining |

Many of these features are relatively independent of each other and constitute discrete wins for binary size, while also touching on other aspects of the standard library and compiler, with some noted exceptions:
- stabilising the use of alternate std implementations will require stable build-std first in order to be usable;
- separating parts of the runtime machinery may require progress on dynamic linking, depending on how the implementation looks.

Additionally, the former binary size working group is to be reestablished in order to coordinate work on all of the above points.

## Funding

Beyond per-topic funding, broad cross-topic funding for binary size reduction is welcome and should be coordinated with the listed point of contact.

(((FUNDING TABLE: Binary size reduction)))

## Frequently asked questions

### Who stands to gain from this?
Any situation where Rust binaries are deployed many times over a network - such as wasm or container images - stand to gain massively from smaller binaries thanks to lower latency, translating into measurably faster startup times. Additionally, potentially memory-constrained environments - such as many embedded targets, bootloaders, or kernels - require small binaries in order to minimise memory requirements or even fit into fixed-size buffers.
