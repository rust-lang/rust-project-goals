# Secure your supply chain

## Summary

Know exactly what code ships in your binaries, catch breaking changes before they break your users, and keep your public API under control.

## Motivation

### The status quo

Every Rust project is built on a foundation of dependencies—the average crate pulls in dozens of transitive dependencies. This is powerful, but it means you're responsible for code you didn't write and may not fully understand.

Today, Rust developers face three challenges:

* **You don't know what's actually in your binary.** `cargo metadata` reports all *possible* dependencies across all configurations, not what actually got compiled. When a vulnerability is announced, you need to know whether your production binary contains the affected code. Regulatory requirements are increasing: the US Executive Order 14028 requires Software Bills of Materials (SBOMs) for federal software, and the EU Cyber Resilience Act mandates SBOMs for products sold in Europe starting in 2027.

* **Breaking changes slip through.** Research shows accidental SemVer violations occur in roughly 3% of releases. When you publish 1.2.3, you might not realize you've removed a method or tightened a trait bound. Users find out when `cargo update` breaks their build.

* **Dependencies leak into your public API.** When you write `pub fn process(data: some_crate::Data)`, that crate becomes part of your API contract. But Cargo doesn't distinguish between dependencies you intentionally expose and ones that accidentally surface.

### What we are shooting for

By the end of 2026:

* **Accurate SBOMs on stable Rust.** `cargo build` produces machine-readable metadata about exactly which crate versions were compiled, in standard formats like CycloneDX or SPDX. When a CVE drops, you can query whether your binary contains the affected code.

* **Better tooling for catching SemVer violations.** `cargo-semver-checks` continues toward integration with `cargo publish`, with progress on type-checking lints and cross-crate analysis. Today, you can run it in CI to catch common breaking changes before they reach users.

* **Public vs private dependencies are explicit.** Marking a dependency as "private" means it can't appear in your public interface. If you accidentally expose a private dependency's type, you get a warning.

### Key use cases

* **Regulatory compliance**: Generate accurate SBOMs directly from the build process for US federal or EU market requirements.

* **Vulnerability response**: Query SBOM data to determine which deployed binaries contain affected code.

* **Confident publishing**: Run SemVer checks in CI, catching breaking changes in PRs before they merge.

* **Intentional API design**: Declare which dependencies are implementation details, catching accidental leakage at compile time.

### Design axioms

* **Accuracy over approximation.** Dependency information must reflect what actually ships. An SBOM with phantom dependencies is worse than none.

* **Integrate into existing workflows.** These capabilities should feel like natural extensions of commands developers already run.

* **Explicit over implicit.** Whether a dependency is public or private should be a deliberate choice in `Cargo.toml`.

* **Incremental value.** Each piece delivers value independently. SBOMs are useful without SemVer checks; SemVer checks are useful without pub/priv dependencies.

## 2026 goals

(((FLAGSHIP GOALS: Secure your supply chain)))

## Frequently asked questions

### How do these goals relate to each other?

The three goals address different aspects of supply chain security and can proceed in parallel:

* **Public/private dependencies** helps control API surface by marking which dependencies should be exposed.

* **cargo-semver-checks** verifies no accidental breaking changes before publishing.

* **Cargo SBOM** generates accurate dependency manifests for compliance and security.

While independent, they're complementary: knowing which dependencies are public helps SemVer checking focus on the right API surface.

### Will cargo-semver-checks catch all breaking changes?

No tool can catch every possible breaking change, but it catches the most common categories (removed items, changed signatures, tightened bounds). The long-term goal is reliability sufficient for `cargo publish` to require compliance by default, with an escape hatch for intentional exceptions. In 2026, work focuses on resolving key blockers like type-checking lints and cross-crate analysis.

### Why does Cargo need its own SBOM feature?

Third-party tools that generate SBOMs from `cargo metadata` see the full dependency graph across all configurations, not what actually gets compiled. Only Cargo knows which crates are selected after feature unification and target filtering.
