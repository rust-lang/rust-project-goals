# Safety-Critical Rust

| Metadata         |                                                                                                                              |
|:-----------------|------------------------------------------------------------------------------------------------------------------------------|
| Short title      | Safety-Critical Rust                                                                                                         |
| What and why     | MC/DC coverage, a specification that tracks stable releases, and `unsafe` documentation — the evidence safety assessors need |
| Point of contact | @PLeVasseur                                                                                                                  |
| Application area | Safety-critical & regulated                                                                                                  |

## Summary

Make Rust viable for certified safety-critical systems by delivering coverage, specification, linting, and `unsafe` documentation foundations.

## Motivation

### The status quo

Engineers building software under functional safety standards need auditable evidence and predictable tooling. The major standards families include:

| Domain | Standards |
|--------|-----------|
| Automotive | [ISO 26262][iso-26262] |
| Aerospace | [DO-178C][do-178c], [DO-333][do-333] (formal methods) |
| Industrial | [IEC 61508][iec-61508], [IEC 61511][iec-61511] (process), [IEC 62061][iec-62061] (machinery) |
| Medical devices | [IEC 62304][iec-62304] |
| Railway | [EN 50128][en-50128], [EN 50716][en-50716] |
| Nuclear | [IEC 60880][iec-60880], [IEC 61513][iec-61513] |
| Space | [ECSS-E-ST-40C][ecss-e-st-40c], [ECSS-Q-ST-80C][ecss-q-st-80c] |
| Agriculture/Forestry | [ISO 25119][iso-25119] |

This includes OEMs, suppliers, integrators, and toolchain vendors. Safety cases must reference a specification for the language being used, and the toolchain must produce evidence that assessors accept.

What these teams need most:

* **Coverage tooling.** [MC/DC][mcdc] reports that assessors accept.
* **A specification that tracks stable releases.** So safety cases can reference current language features via the [FLS][fls].
* **Continued progress on `unsafe` documentation.** The [Reference][rust-reference], [Rustonomicon][rustonomicon], and [standard library docs][std-docs] have improved substantially over the years, but gaps remain for common patterns.
* **Stable foundations.** Product lifetimes of 10-20 years require confidence in toolchain support and MSRV conventions.

Teams at SIL 2 and ASIL B are [shipping Rust today][vision-doc]. These deliverables reduce the workarounds they need and create a path to higher integrity levels.

### What we are shooting for

We are building a capability ladder that unlocks Rust at increasing safety-integrity levels. The 2026 focus is the foundation, while keeping a clear path to higher tiers.

* **Foundation (ASIL A/B, SIL 1/2, DO-178C Level C).** Stable branch/DC coverage baselines and predictable [FLS][fls] releases, plus initial safety-critical linting to enforce [Safety-Critical Rust Consortium][scrc] coding standards.
* **Intermediate (ASIL C, SIL 3, DO-178C Level B).** Normative `unsafe` pattern documentation and expanded lint coverage, with coordination for mixed-language interop and async runtime patterns.
* **Highest integrity (ASIL D, SIL 4, DO-178C Level A).** [MC/DC][mcdc] coverage in rustc and formal-methods coordination toward contracts, semantics documentation, and verified tooling.

### Key use cases

* **Certification evidence**: Generate coverage and spec references that auditors accept.
* **Qualified toolchains**: Tool vendors can qualify rustc/Clippy with predictable releases and lint sets.
* **Mixed-language systems**: Integrate Rust into C and C++ stacks with well-defined `unsafe` contracts.
* **Long-lived products**: Maintain 10-20 year systems with stable MSRV and documentation baselines.

### Design axioms

* **Evidence first.** Prioritize deliverables that produce audit-ready evidence.
* **Use standard tooling.** Improve rustc, Clippy, and core docs rather than forks.
* **Ship a ladder.** Foundations first, with an explicit path to higher integrity levels.
* **Document safety contracts.** `unsafe` patterns must have normative, citable guidance.

## 2026 goals

(((ROADMAP GOALS: Safety-Critical Rust)))

## Frequently asked questions

### How do these goals relate to each other?

They form a coherent evidence chain: [FLS][fls] releases provide a citable specification, `unsafe` documentation defines safety contracts, Clippy lints enforce coding guidelines, and [MC/DC][mcdc] coverage produces the evidence required at the highest integrity levels.

### Does this certify Rust for safety-critical use?

No. Certification is per product and toolchain. These goals deliver the foundations that make qualification and certification feasible without bespoke tooling.

### Why focus on foundations in 2026?

Teams at lower integrity levels are already shipping Rust and need practical improvements now. The foundation work also makes the higher-integrity goals achievable later.

[rust-reference]: https://doc.rust-lang.org/reference/
[rustonomicon]: https://doc.rust-lang.org/nomicon/
[std-docs]: https://doc.rust-lang.org/std/
[mcdc]: https://en.wikipedia.org/wiki/Modified_condition/decision_coverage
[fls]: https://spec.ferrocene.dev/
[scrc]: https://github.com/rustfoundation/safety-critical-rust-consortium
[vision-doc]: https://blog.rust-lang.org/2026/01/14/what-does-it-take-to-ship-rust-in-safety-critical/
[iec-61508]: https://webstore.iec.ch/en/publication/5515
[iec-61511]: https://webstore.iec.ch/en/publication/5527
[iec-62061]: https://webstore.iec.ch/en/publication/59927
[iec-62304]: https://webstore.iec.ch/en/publication/4316
[iec-60880]: https://webstore.iec.ch/en/publication/3799
[iec-61513]: https://webstore.iec.ch/en/publication/5532
[iso-26262]: https://www.iso.org/standard/68383.html
[iso-25119]: https://www.iso.org/standard/69025.html
[do-178c]: https://my.rtca.org/productdetails?id=a1B36000001IcmqEAC
[do-333]: https://my.rtca.org/productdetails?id=a1B36000001IcmuEAC
[en-50128]: https://www.cenelec.eu/dyn/www/f?p=104:110:0::::FSP_ORG_ID,FSP_PROJECT,FSP_LANG_ID:1257173,65440,25
[en-50716]: https://www.cenelec.eu/dyn/www/f?p=104:110:0::::FSP_ORG_ID,FSP_PROJECT,FSP_LANG_ID:1257173,72818,25
[ecss-e-st-40c]: https://ecss.nl/standard/ecss-e-st-40c-rev-1-software-30-april-2025/
[ecss-q-st-80c]: https://ecss.nl/standard/ecss-q-st-80c-rev-2-software-product-assurance-30-april-2025/
