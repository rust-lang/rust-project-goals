# Crate Slicing for Faster Fresh Builds

| Metadata |  |
| --- | --- |
| Point of contact | @yijunyu |
| Status | Proposed |
| Zulip channel | TBD |

## Summary

Use `cargo-slicer` and PRECC-Rust as proof-of-concept evidence that a significant
**separate compilation gap** exists in Rust fresh builds, and engage the compiler
team on a well-designed rustc-native implementation: emitting **stub rlibs**
(names only, no bodies) for dependencies, then deferring type-checking, borrow
checking, and codegen of used functions to the root crate's compilation — enabling
better parallelism across currently idle cores.

## Motivation

### The status quo

The normal Rust compilation pipeline for each crate is:
`AST → HIR → THIR → MIR → LLVM IR`

Type-checking happens when generating THIR; borrow-checking happens on MIR.
Every dependency crate goes through all these stages regardless of how much of it
is actually used by the downstream consumer. We call the difference between what
the compiler *must* process and what is *actually reachable* from the final
binary's entry points the **separate compilation gap**.

Measurements across real Rust projects (see [5]) show this gap ranges from under
1% to 37% of total CPU instructions:

| Project | Baseline (s) | PRECC-Rust (s) | Speedup | Gap |
|---------|-------------|----------------|---------|-----|
| zed (500K+ LOC) | 1,012 | 719 | **−29%** | 37% |
| rustc | 135.8 | 112.4 | **−17%** | 26% |
| zeroclaw (AI agent) | 192.9 | 170.4 | **−12%** | 13% |
| helix | 71.2 | 66.6 | **−6%** | 11% |
| ripgrep | 11.1 | 10.7 | **−4%** | 5% |
| nushell | 106.5 | 108.9 | +2.3% | 0.4% (overhead > savings) |
| bevy | 81.8 | 85.4 | +4.4% | 0.4% (overhead > savings) |

The nushell and bevy results are equally important: when the gap is small,
analysis overhead exceeds savings. A production implementation must apply this
*selectively*, only where predicted benefit exceeds cost.

As the [parallel-rustc blog post](https://blog.rust-lang.org/2023/11/09/parallel-rustc/)
shows, there are often idle cores during normal Rust compilation. The key insight
is that the total amount of work does not necessarily decrease — we still generate
THIR and MIR for every used function — but by deferring this work to the root
crate's compilation it can proceed **in parallel** with other compilation work,
exploiting those idle cores.

Existing mitigations (incremental compilation, sccache, Cranelift, parallel
frontend) reduce repeated or parallelized work but none defer cross-crate
type-checking and borrow-checking to exploit idle parallelism.

### The next 6 months

**Part 1 — Consolidate POC findings.**

Two prototypes establish the feasibility:

- **`cargo-slicer`** [4]: source-level slicer that statically identifies used items
  from dependency crates and generates minimal sliced versions. Validates that
  sliced major ecosystem crates (tokio, axum, hyper, reqwest, syn, rand, regex)
  still compile correctly.
- **PRECC-Rust** [5]: operates as a `RUSTC_WRAPPER`, hooking into rustc *after
  type checking* to replace unreachable function bodies with MIR abort stubs,
  eliminating downstream codegen work without modifying source code. Produces the
  measured results in the table above.

**Part 2 — Engage the compiler team on a rustc-native design.**

Neither prototype is suitable for direct adoption:

- `cargo-slicer` bypasses rustc's name resolution, type inference, and coherence
  checking. It cannot safely handle blanket impls, proc-macro interactions, or
  build scripts.
- PRECC-Rust's `syn`-based call graph is an approximation; the design described
  below is architecturally cleaner and does not require Cargo involvement.

A robust solution requires a full-time engineer working *inside* rustc. The
proposed architecture (informed by compiler team feedback) is:

1. **Stub rlibs**: rustc compiles each dependency crate only to `AST → HIR`,
   emitting a stub rlib containing names and signatures but no function bodies.
   This is analogous to how LTO defers LLVM IR generation — but stopping even
   earlier in the pipeline, before THIR and MIR.

2. **Deferred compilation at the root**: a single rustc process for the root
   binary crate loads the stub rlibs and, for each function body actually
   reachable from `main`, performs the full `THIR → MIR → LLVM IR` pipeline.
   This work can proceed in parallel across dependency crates, filling idle cores
   that normal sequential per-crate compilation leaves unused.

3. **No Cargo changes needed**: the entire mechanism is internal to rustc.
   Cargo invokes rustc as usual; rustc decides how to schedule stub vs. full
   compilation.

The 2026 goal is to use the POC measurements to:

1. **Formalize soundness criteria**: under what conditions is deferred compilation
   semantically equivalent to the standard pipeline? (trait coherence,
   monomorphization, proc-macro crates, build scripts)

2. **Work with the compiler team** to prototype stub rlib emission and the
   deferred root-crate compilation pass inside rustc.

3. **Quantify the parallelism benefit**: measure wall-clock improvement from
   exploiting idle cores, using the separate compilation gap data as a predictor.

### The "shiny future"

`cargo build` automatically emits stub rlibs for dependencies and defers
type-checking, borrow-checking, and codegen of used functions to the root crate's
compilation. This saturates idle cores during fresh builds, achieving meaningful
wall-clock speedups for large projects with no changes required to Cargo or the
ecosystem. It complements parallel frontend (reduces wall-clock within a crate)
and Cranelift (reduces codegen time) by better utilizing available parallelism
across crates.

## Design notes

### Why cargo-slicer is a POC, not a production solution

The source-level approach has fundamental limitations:

- Re-emitting source bypasses rustc's coherence checking and impl resolution
- Fragile against language evolution (relies on parsing heuristics, not rustc IRs)
- No integration with rustc's incremental fingerprints
- `syn`-based call graph is unsound for generic instantiation and dynamic dispatch

### The stub rlib architecture

The proposed rustc-native design is analogous to LTO but defers work even earlier:

| Phase | Normal compilation | LTO | Stub rlib approach |
|-------|--------------------|-----|--------------------|
| AST → HIR | per crate | per crate | per crate (stub rlib emitted here) |
| HIR → THIR (type-check) | per crate | per crate | deferred to root crate |
| THIR → MIR (borrow-check) | per crate | per crate | deferred to root crate |
| MIR → LLVM IR (codegen) | per crate | deferred to link | deferred to root crate |

Dependencies emit stub rlibs after `AST → HIR`: names, signatures, and trait
definitions, but no function bodies. The root binary crate then loads these stub
rlibs and completes the pipeline only for functions reachable from `main`,
running the deferred work in parallel across all dependencies within a single
rustc process. This exploits the idle cores visible in the
[parallel-rustc measurements](https://blog.rust-lang.org/2023/11/09/parallel-rustc/).

Note: the total THIR/MIR/codegen work for reachable functions is unchanged —
the benefit comes from *parallelism*, not from eliminating work.

### Rust-specific challenges

| Challenge | Notes |
|-----------|-------|
| Trait coherence | All impls for used types must be included |
| Generics / monomorphization | Analysis may miss instantiation paths |
| Proc-macro crates | Execute at compile time; preserve as-is |
| `build.rs` generators | Code invisible to static analysis; fall back to full crate |
| Blanket impls | `impl<T> Trait for T` required if `Trait` is used |

## Task list

| Task | Owner | Status |
|------|-------|--------|
| Prototype cargo-slicer and PRECC-Rust | @yijunyu | Done |
| Measure separate compilation gap across 8 projects | @yijunyu | Done |
| Write technical paper (ASE 2026 submission) | @yijunyu | In Progress |
| Formalize soundness requirements for deferred compilation | @yijunyu | Not started |
| Design stub rlib emission in rustc | TBD | Not started |
| Prototype deferred root-crate compilation pass | TBD | Not started |
| Present findings to compiler team | TBD | Not started |

## Team asks

| Team | Support level | Notes |
|------|---------------|-------|
| compiler | Medium | Collaborate on stub rlib design and deferred compilation pass; consult on soundness and rustc internals |
| types | Small | Consultation on trait coherence requirements for deferred type-checking |
| lang | Small | Feedback on corner cases (blanket impls, proc-macros, build scripts) |

## Frequently asked questions

### Isn't cargo-slicer already solving the problem?

No. It is a POC demonstrating the opportunity. It bypasses rustc semantics and
cannot safely handle all language constructs. The POC is the argument for
investing in a proper rustc-native implementation.

### Does this reduce the total amount of compilation work?

Not necessarily. The total THIR, MIR, and codegen work for reachable functions
remains the same. The benefit is **parallelism**: type-checking and borrow-checking
of dependency functions is deferred to the root crate's compilation, where it can
run concurrently with other work on idle cores, reducing wall-clock time.

### How is this different from dead code elimination?

DCE operates after full compilation. The stub rlib approach defers
type-checking and borrow-checking *before* codegen — it is closer to LTO
(which defers LLVM IR generation) but stops even earlier in the pipeline.

### How is this different from feature flags?

Feature flags require manual crate-author effort and are coarse-grained. Slicing
is automatic, usage-based, and operates at the item level.

### What about crates that can't be sliced?

Fall back to full crate compilation. Known hard cases: proc-macro crates, heavy
`build.rs` generators, `#[doc(hidden)]` items used via macros, blanket impls.

## References

[1] Y. Yu et al. "Reducing Build Time through Precompilations for Evolving Large Software." ICSM 2005.

[2] Y. Yu. "Precompilation: Splitting C/C++ Source Files for Faster Incremental Builds." Open University TR2012/01.

[3] demo-precc: https://github.com/yijunyu/demo-precc

[4] cargo-slicer: https://github.com/yijunyu/cargo-slicer (proof-of-concept source-level crate slicer)

[5] PRECC paper (ASE 2026, under review): "PRECC: Predictive Precompilation Cutting via Pareto-Optimal Selective Slicing"
