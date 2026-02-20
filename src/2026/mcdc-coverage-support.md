# Implement and Maintain MC/DC Coverage Support

| Metadata         |                                                              |
| :--------------- | ------------------------------------------------------------ |
| Point of contact | @RenjiSann                                                   |
| Status           | Proposed                                                     |
| Roadmap          | Safety-Critical Rust                                         |
| Tracking issue   |                                                              |
| Zulip channel    | [mc/dc-support][mcdc-zulip] |

## Summary

Re-implement DC (Decision Coverage) and [MC/DC (Modified Condition/Decision Coverage)][mcdc-wikipedia] instrumentation in rustc with a commitment to ongoing maintenance from AdaCore.

## Motivation

MC/DC coverage is required by safety-critical standards including DO-178C (aviation, Level A), ISO 26262 (automotive, ASIL D), and IEC 61508 (industrial, SIL 3/4). Decision coverage is a less strict version of MC/DC only assessing the outcome of decisions (not looking at conditions), and required in DO-178C level B. Both criteria are different from branch coverage.

Without Decision and MC/DC support in rustc, Rust cannot be used in these domains as implementation outside of the compiler looks infeasible.

### The status quo

MC/DC support has a [tracking issue][mcdc-tracking-issue] with initial implementation and updates in 2024 (PRs [#123409][mcdc-implementation-initial-pr], [#126733][mcdc-update-pr]) under the `-Zcoverage-options=mcdc` flag. However, it was **removed in late 2025** (commit [562222b](https://github.com/rust-lang/rust/commit/562222b73765a326fa800a075814deaf627874df)). The removal rationale from the maintainer cited:

> - "Major burden on overall maintenance of coverage instrumentation"
> - "Major obstacle to other planned improvements" (e.g., macro expansion regions)
> - "Not yet complete, and shows little sign of being complete at an acceptable level of code quality in the foreseeable future"

This is a setback for safety-critical adoption of Rust. The previous implementation covered plain boolean expressions and nested decisions, but pattern matching support remained a draft, and there were known bugs with constant conditions (some fixed in LLVM 20).

We found that re-implementing this feature using rustc is the only viable options as to the best of our knowledge there is no way to generate valid rust sources after the expansion of macros. Macros are extensively used, even in the core library, making it unrealistic to restrict their use in safety critical applications. This thus only leaves the option to modify the intermediate representation in the compiler to introduce source coverage tracking instructions.

### The next year

We acknowledge that the previous implementation was removed for valid reasons. This goal is not simply "re-land the old code" but rather "re-implement with a more sustainable architecture and dedicated maintenance commitment."

[Early discussions][mcdc-zulip] with both Niko Matsakis and the compiler team suggest that this effort is large enough that it should go through a [Major Change Proposal](https://github.com/rust-lang/rfcs/blob/master/text/2904-compiler-major-change-process.md) (MCP) to ensure the compiler team can validate the architectural choices prior to implementation.

| Task                                              | Owner(s)              | Notes                                                        |
| ------------------------------------------------- | --------------------- | ------------------------------------------------------------ |
| Understand removal rationale in detail            | Consortium + compiler | Meet with coverage maintainers, e.g. @oli-obk                |
| Investigate potential design architectures        | Adacore + Compiler team | See [Architectural options under consideration](#architectural-options-under-consideration)
| Design revised implementation approach            | AdaCore engineer(s)   | Address code quality concerns; choose architecture based on investigation and submit a compiler MCP|
| Implement core MC/DC for boolean expressions      | AdaCore engineer(s)   | Target: working `-Zcoverage-options=mcdc` on nightly         |
| Establish ongoing maintenance commitment          | AdaCore engineer(s)   | Formal agreement for long-term support                       |
| Coordinate with LLVM 20 for constant condition fixes | Compiler team      | Upstream bugs were blocking some functionality               |

### The "shiny future" we are working towards

Rust has stable, well-maintained MC/DC coverage support that meets the requirements of DO-178C, ISO 26262, and IEC 61508. Safety-critical projects can use `cargo llvm-cov` (or similar tooling) to generate MC/DC reports suitable for certification evidence.

**2026 scope:** We aim for unstable (`-Zcoverage-options=mcdc`) support covering the core use cases. Stabilization would be a subsequent goal, if maintenance sustainability is demonstrated.

## Design notes

### Architectural options under consideration

Based on feedback from Niko Matsakis and the compiler team, we are investigating three potential architectural approaches:

1. **Extend Stable MIR:** Add the syntactic structure information needed for MC/DC to the [Stable MIR][project-stable-mir] API. This provides a stable foundation that is less likely to conflict with other compiler changes, but requires preliminary work prior to implementation of MC/DC coverage instrumentation itself. On Paper the Stable MIR approach aligns well with the needs of a verification tool, with a stable interface usable with multiple versions of the compiler, remains to be seen if the required adaptations would fit within the goal's timeframe.

2. **Compiler hooks approach:** Use the existing mechanism for overriding compiler hooks (similar to what Kani uses today for MIR generation). This is more immediately achievable but may have similar maintenance characteristics to the previous implementation. It retains the benefit of keeping the implementation separate from the rustc codebase.

3. **Re-Implement the feature directly within rustc** Add the coverage instrumentation passes directly within rustc. Like the compiler hook approach, this is more immediate, with easier access to the AST, and has the advantage of keeping the existing coverage instrumentations and the newly proposed one within the same codebase. Careful design would be needed for the instrumentation not to be too invasive, or impede development of other features.

## Team asks

| Team       | Support level | Notes                                                        |
| ---------- | ------------- | ------------------------------------------------------------ |
| [compiler] | Medium        | Review of implementation PRs; guidance on architecture to avoid previous maintenance issues; input on Stable MIR extension feasibility |
| [project-stable-mir] | Medium | Consultation on extending Stable MIR with syntactic structure; review of any proposed API additions (if this approach is chosen)|
| [infra]    | Small         | CI support for MC/DC testing                                 |

**Resources committed:** AdaCore is prepared to commit engineering resources to implementation and ongoing maintenance. We understand this was the missing piece previously.

## Frequently asked questions

### Why should this attempt succeed when the previous one was removed?

Two key differences:

1. **Dedicated maintenance commitment.** The previous implementation was primarily one person's effort without long-term support guarantees. AdaCore can provide ongoing engineering resources specifically for coverage instrumentation maintenance.

2. **Architectural approach validation.** We plan to engage with the coverage maintainers early to understand the architectural concerns and design an implementation that addresses them. This is to ensure buy-in from the compiler team before submitting PRs.

### What about pattern matching?

Pattern matching MC/DC support was only at draft stage previously, the coverage criterion being still ill-defined for this language construct. Ferrous Systems have proposed [one interpretations of patterns as decisions and conditions][pattern-mcdc], to which we could apply MC/DC coverage, but we're uncertain how much additional work this requires. Our initial scope focuses on boolean expressions and nested decisions, which covers the most common safety-critical use cases.

AdaCore plans to instruct customers to avoid pattern matching with an initial offering. Pattern matching support would be a stretch goal or follow-on effort.

### What's the relationship to GNATcoverage?

GNATcoverage is AdaCore's coverage analysis tool. It can consume LLVM coverage data including MC/DC information. Having MC/DC support in rustc means GNATcoverage (and other LLVM-based coverage tools) can provide MC/DC reports for Rust code. This is valuable for organizations already using GNATcoverage for Ada/C in mixed-language safety-critical systems.

[mcdc-wikipedia]: https://en.wikipedia.org/wiki/Modified_condition/decision_coverage
[mcdc-tracking-issue]: https://github.com/rust-lang/rust/issues/124144
[mcdc-implementation-initial-pr]: https://github.com/rust-lang/rust/pull/123409
[mcdc-update-pr]: https://github.com/rust-lang/rust/pull/126733
[project-stable-mir]: https://github.com/rust-lang/project-stable-mir
[mcdc-zulip]: https://rust-lang.zulipchat.com/#narrow/channel/546987-project-goals.2F2026-workshop/topic/mcdc-support/with/569335878
[pattern-mcdc]: https://arc.aiaa.org/doi/10.2514/1.I011558
