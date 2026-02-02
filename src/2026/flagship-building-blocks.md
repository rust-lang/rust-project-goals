# Building blocks

## Summary

Expose Cargo and Rust's internal infrastructure as stable, composable building blocks for tooling authors and power users.

## Motivation

### The status quo

Rust's tooling ecosystem includes hundreds of third-party tools built on top of Cargo, rustc, and the standard library. Tools like cargo-nextest, rust-analyzer, and cargo-deny extend what ships with Rust. But tool authors face significant friction:

* **Cargo is designed for humans, not scripts.** When tools need dependency information, feature resolution, or build graph data, they must parse human-readable output or use `cargo metadata` (which has known limitations). There's no stable programmatic interface.

* **libtest's output is unstable.** The standard test harness provides no stable machine-readable format. When `--format json` was accidentally stabilized and then reverted to nightly-only, it revealed how many tools depended on it.

* **Building std requires nightly.** The `-Zbuild-std` flag lets users rebuild the standard library with custom settings, but it remains unstable. Embedded developers and those on tier-3 targets must use nightly.

The result: tool authors spend effort reverse-engineering Cargo's behavior and building workarounds for missing capabilities.

### What we are shooting for

By the end of 2026:

* **Cargo plumbing commands are prototyped and validated.** External experimentation with machine-readable commands that expose Cargo's operations, with a path toward eventual inclusion in Cargo. Just as Git distinguishes "porcelain" (user-facing) from "plumbing" (scriptable), this work explores building blocks for programmatic access.

* **libtest JSON output experiment is complete.** The experiment concludes with a proposal for stable JSON output, enabling better test runners and IDE integration.

* **build-std implementation is underway.** RFCs are accepted and implementation has begun, moving toward eventual stabilization of rebuilding the standard library with custom configurations.

* **Interactive dependency exploration is prototyped.** External tooling demonstrates interactive navigation of dependency graphs, with preparation for eventual Cargo integration.

### Key use cases

* **Custom test runners**: Tools like cargo-nextest rely on stable JSON output for parallel execution and better failure reporting.

* **Build system integration**: Organizations using Buck, Bazel, or custom systems need reliable programmatic access to Cargo's dependency and build information.

* **IDE tooling**: rust-analyzer needs fast, reliable project structure and dependency information.

* **Embedded development**: Developers working on embedded systems or kernel development need to rebuild std with specific configurations on stable Rust.

### Design axioms

* **Separate porcelain from plumbing.** User-facing commands remain focused on usability. Programmatic access comes through dedicated plumbing commands with stable schemas.

* **Stable means stable.** When we stabilize an interface, tool authors can depend on it without checking Rust versions.

* **Expose the truth.** Plumbing commands expose what Cargo actually knows, not a simplified approximation that might miss edge cases.

* **Prototype externally, integrate when ready.** New capabilities start as external tools where iteration is fast, then integrate into Cargo once designs stabilize.

## 2026 goals

(((FLAGSHIP GOALS: Building blocks)))

## Frequently asked questions

### How do these goals relate to each other?

The goals share a vision but are largely independent:

* **build-std** focuses on completing RFCs and beginning implementation for the compiler and standard library.
* **cargo-plumbing** prototypes programmatic access to Cargo's operations as an external tool.
* **libtest-json** completes the experiment toward stable test harness output.
* **interactive-cargo-tree** prototypes improved dependency exploration UX externally.

Each delivers value independently, though cargo-plumbing may eventually provide APIs that other tools use.

### Who benefits from this work?

Primarily tooling authors, large organizations integrating Rust into build systems, embedded developers, IDE developers, and power users. Indirectly, all Rust users benefit because better infrastructure enables better tools.

### Why not expose Cargo as a library?

Cargo-as-a-library has significant challenges: API stability commitments, versioning complexity, and architectural constraints. Plumbing commands provide a practical near-term solution with clear boundaries and schema versioning.
