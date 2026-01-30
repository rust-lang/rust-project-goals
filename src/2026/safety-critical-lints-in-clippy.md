# Establish a Spot for Safety-Critical Lints in Clippy

| Metadata         |                                                              |
| :--------------- | ------------------------------------------------------------ |
| Point of contact | @plevasseur                                                  |
| Status           | Proposed                                                     |
| Tracking issue   |                                                              |
| Zulip channel    | N/A                                                          |

## Summary

Establish a sustainable arrangement for the Safety-Critical Rust Consortium to contribute and maintain lints within the Clippy ecosystem, with the goal of adding productive members of the clippy team with a focus on lints useful in functional safety.

## Motivation

Safety-critical development requires enforcing project-specific coding standards. The [Safety-Critical Rust Consortium][scrc] is developing [coding guidelines][coding-guidelines] that will need to be reduced to practice through tooling. This effort will generate a significant volume of new lints, potentially 50 to 200 over the next one to two years.

Many of those lints are useful outside of safety-critical, but often in the pedantic category.

[scrc]: https://github.com/rustfoundation/safety-critical-rust-consortium
[coding-guidelines]: https://github.com/rustfoundation/safety-critical-rust-coding-guidelines

### The status quo

The Consortium needs custom lints to enforce its coding guidelines. While alternative approaches exist (such as Dylint for dynamic lint loading), the Consortium prefers to work within the community embraced tool rather than maintain a fork or separate tool. This aligns with the broader goal of Rust adoption in safety-critical domains: using standard, well-supported tools rather than fragmenting the ecosystem.

The challenge is scale. The Clippy team has significant experience in lint design and review, but an organized effort to add 50 to 200 lints could potentially strain review capacity if not managed thoughtfully.

### Exploratory MISRA C to FLS mapping analysis

As an exploratory measure, a mapping of 223 MISRA C:2025 guidelines to the Ferrocene Language Specification (FLS) was performed based on the [MISRA C:2025 Addendum 6: Applicability of MISRA C:2025 to the Rust Programming Language][misra-add6]. This work was done independently and is not yet normative Consortium guidance, though it may become a Consortium artifact if deemed viable and useful.

[misra-add6]: https://misra.org.uk/app/uploads/2025/03/MISRA-C-2025-ADD6.pdf

The mapping reveals the scope of potential lint needs:

| Category | Count | Notes |
|----------|-------|-------|
| Total guidelines | 223 | MISRA C:2025 |
| Directly applicable to Rust | 68 | Need some form of enforcement |
| Already covered by rustc/Clippy | 13 | `dead_code`, `unreachable_code`, etc. |
| Requiring custom lint support | ~50 | Beyond current Clippy coverage |

That's leaving aside the other [mapping being done to CERT C][rust-cert-c] and the coding guidelines which may apply only to Rust.

[rust-cert-c]: https://github.com/rustfoundation/safety-critical-rust-coding-guidelines/issues/336

### The next year

The Clippy team has offered mentorship and onboarding support to help SCRC contributors become effective lint developers and reviewers. The envisioned process:

1. **Initial phase (1 to 2 months):** Clippy team provides hands-on mentorship for a small group of SCRC lint developers, reviewing lint designs in issues and PRs and providing guidance on lint design and implementation patterns. Some members of the SCRC employ clippy contributors that could play the role of a mentor.

2. **Transition phase:** As SCRC contributors gain experience, they take on more review responsibility for safety-critical lints, with Clippy team available for consultation on complex cases.

3. **Steady state:** SCRC maintains its lints with minimal ongoing burden on the Clippy team. SCRC reviewers are members adding to general Clippy review capacity, with a focus point, as all other members.

| Task                                              | Owner(s)            | Notes                                                     |
| ------------------------------------------------- | ------------------- | --------------------------------------------------------- |
| Finalize arrangement for SCRC lints in Clippy     | Clippy              | Determine crate/group structure (see options below)       |
| Identify initial SCRC lint developers             | Consortium          | 2–3 people to receive initial mentorship                  |
| Onboarding and mentorship period                  | Clippy + Consortium | ~1–2 months of active guidance                            |
| Implement initial set of safety-critical lints    | Consortium          | Starting with highest-priority coding guidelines          |
| Establish SCRC review capacity                    | Consortium          | Enable self-sufficient lint review over time              |
| Document lint design patterns for safety-critical use cases | Consortium + Clippy | Capture lessons learned for future contributors    |

### Possible arrangements

The specific structure for SCRC lints within Clippy is still under discussion. Two options are being considered:

**Option A: Separate crate for SCRC lints**

A dedicated crate within the Clippy repository for safety-critical lints. This approach:
- Provides a clear boundary for SCRC-maintained code
- Allows experimentation without affecting existing Clippy lints
- Lints could be organized as a dedicated lint group that users can enable/disable
- Aligns with ongoing Clippy work to split lints into multiple crates

**Option B: Direct integration into Clippy**

SCRC lints go directly into Clippy, potentially starting in a separate lint group (e.g., `restriction` or a new `safety-critical` group). This approach:
- Integrates safety-critical lints into the standard tool from the start
- Leverages existing Clippy infrastructure without additional crate boundaries
- Lints that make sense for general use can be promoted to other groups over time

Both options share the same mentorship model and goal of building SCRC's self-sufficiency. The choice affects organizational structure rather than the fundamental collaboration approach.

### The "shiny future" we are working towards

The Clippy ecosystem accommodates safety-critical lint development as a first-class use case:

1. SCRC can develop and maintain lints needed for coding guideline enforcement
2. The arrangement scales without overburdening the Clippy team
3. Safety-critical users benefit from community tooling rather than fragmented alternatives
4. Cross-pollination between SCRC and Clippy improves both efforts

## Design notes

### Why not a fork or separate tool?

The Consortium considered alternatives like maintaining a Clippy fork or using Dylint, but prefers working within the community tool:

- **Forks split maintenance effort** and create confusion about which tool to use.
- **Dylint** requires managing lint-driver-compiler version coordination and lacks official Rust Project support
- **Community integration** means safety-critical users get a standard, well-supported tool. Many safety-critical techniques are generally applicable methodologies and community exchange is worthwhile.
- **Configuration consistency** All lints can be configured in one program.

### Relationship to stable API investigation

The original goal proposal focused on investigating stable APIs for custom lints (via Stable MIR or other mechanisms). That investigation remains valuable for the long term, particularly for lints requiring deep analysis like whole-program call graphs, but is not a prerequisite for the immediate work of contributing lints to Clippy.

## Team asks

| Team       | Support level | Notes                                           |
| ---------- | ------------- | ----------------------------------------------- |
| [clippy]   | Mentorship    | Initial onboarding support for SCRC contributors; guidance on lint design |

**Resources committed:** The Consortium will commit engineering time for lint development, with the goal of becoming self-sufficient in review capacity over time.
**Members committment:** Consortium members are willing to bring staff with experience in mentoring.

## Frequently asked questions

### Won't 50–200 lints overwhelm the Clippy team?

This is exactly what the mentorship model addresses. Rather than the Clippy team reviewing all SCRC lints indefinitely, the goal is to build SCRC's capacity to review their own lints. The Clippy team's investment is front-loaded: intensive mentorship for 1 to 2 months, then decreasing involvement as SCRC contributors become proficient. If review capacity becomes strained, we can revisit the arrangement.

Clippy already maintains a lot of lints, so there is experience in managing the scale.

### What about lints that need deep analysis (call graphs, cross-crate)?

Some safety-critical lints, like detecting recursion or tracking allocation across modules, require analysis capabilities beyond what's easily accessible today. These lints may need to:

- Use existing internal APIs (with the understanding that maintenance across compiler versions requires effort)
- Wait for stable APIs (Rustc Public / Stable MIR) to mature
- Be implemented as external tools initially

The immediate focus is on lints that can be implemented with current Clippy infrastructure. More complex lints are a longer-term challenge that may warrant separate investigation.

### How does this relate to safety-qualification of Clippy?

Some safety-critical companies are exploring safety-qualification of Clippy or Clippy-like tooling. Having SCRC lints within Clippy (rather than a fork) supports this effort by:

- Keeping the codebase unified
- Allowing qualified toolchains to include these lints with minimal divergence from upstream
- Building a community of reviewers familiar with safety-critical lint requirements

### What specific lints will SCRC contribute?

The initial focus will be lints that enforce the Consortium's coding guidelines. Priority will be determined by:

1. Guidelines with highest safety impact
2. Lints implementable with current Clippy infrastructure
3. Lints likely useful beyond safety-critical contexts (for potential promotion to general Clippy groups)

The exploratory MISRA C:2025 mapping provides a starting point, but the Consortium will also consider domain-specific needs from automotive, aerospace, medical, and other safety-critical sectors.
