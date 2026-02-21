# Crate Slicing for Faster Fresh Builds

| Metadata |  |
| --- | --- |
| Point of contact | @yijunyu |
| Status | Proposed |
| Zulip channel | TBD |

## Summary

Use `cargo-slicer` and PRECC-Rust as proof-of-concept evidence that exploiting
the **separate compilation gap** can meaningfully reduce Rust fresh build times,
and engage the compiler and cargo teams on a well-designed rustc-native
implementation using the query system and cross-crate MIR analysis.

## Motivation

### The status quo

The Rust compiler must process all visible items at each crate boundary — parsing,
name resolution, and type-checking proceed regardless of whether an item is
reachable from the final binary. We call the difference between what the compiler
*must* process and what is *actually reachable* the **separate compilation gap**.

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
analysis overhead exceeds savings. A production implementation must apply slicing
*selectively*, only where predicted benefit exceeds cost.

Existing mitigations (incremental compilation, sccache, Cranelift, parallel
frontend) reduce repeated or parallelized work but none reduce the total quantity
of source processed in a fresh build.

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

**Part 2 — Engage rustc and cargo teams on a production path.**

Neither prototype is suitable for direct adoption:

- `cargo-slicer` bypasses rustc's name resolution, type inference, and coherence
  checking. It cannot safely handle blanket impls, proc-macro interactions, or
  build scripts.
- PRECC-Rust approximates the call graph with `syn`; the rustc query system
  provides far more precise semantic information.

A robust solution requires a full-time engineer working *inside* rustc and Cargo.
The 2026 goal is to use the POC evidence to:

1. **Formalize soundness criteria**: under what conditions is a sliced crate
   semantically equivalent to the original? (trait coherence, monomorphization,
   build scripts, proc-macro crates)

2. **Design the rustc integration**: evaluate candidate approaches with the compiler team:
   - A `used_items(crate)` query so Cargo can pass reachability data to dependency compilations
   - Native MIR-level stub replacement inside rustc proper, replacing the wrapper approximation
   - Cargo two-pass orchestration (analyse root crate → compile dependencies with used-item sets)

3. **Present findings** to compiler and cargo teams for feedback on the viable integration path.

### The "shiny future"

`cargo build` automatically exploits cross-crate reachability, achieving
**20–40% reduction in fresh build time** for large dependency-heavy projects
with no ecosystem changes required. This complements parallel frontend (reduces
wall-clock) and Cranelift (reduces codegen overhead) by reducing the total
*amount* of work.

## Design notes

### Why cargo-slicer is a POC, not a production solution

The source-level approach has fundamental limitations:

- Re-emitting source bypasses rustc's coherence checking and impl resolution
- Fragile against language evolution (relies on parsing heuristics, not rustc IRs)
- No integration with rustc's query system or incremental fingerprints
- `syn`-based call graph is unsound for generic instantiation and dynamic dispatch

A proper implementation operates inside rustc (post-type-checking, at MIR level)
and inside Cargo (first-class build orchestration), with the query system providing
precise, cached, incremental reachability.

### Path to a rustc-native implementation

1. **Query-system reachability**: add a `used_items(crate)` query that Cargo
   populates from the downstream crate's compilation, so dependency crates compile
   with accurate reachability information.
2. **MIR-level dead item elimination**: post-type-checking, unreachable items are
   replaced with abort stubs or excluded from codegen, using the same mechanism as
   existing MIR optimizations.
3. **Cargo orchestration**: two-pass build — analyse root crate, compile
   dependencies with used-item sets — mirroring existing feature resolution.
4. **Incremental integration**: sliced item sets feed smaller fingerprints,
   benefiting incremental builds too.

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
| Formalize soundness requirements | @yijunyu | Not started |
| Design rustc query-system integration architecture | TBD | Not started |
| Present findings to compiler and cargo teams | TBD | Not started |

## Team asks

| Team | Support level | Notes |
|------|---------------|-------|
| compiler | Medium | Review query-system integration design; consult on soundness and rustc internals |
| cargo | Medium | Discuss two-pass build orchestration and crate-level reachability integration |
| types | Small | Consultation on trait coherence requirements for slicing |
| lang | Small | Feedback on corner cases and research methodology |

## Frequently asked questions

### Isn't cargo-slicer already solving the problem?

No. It is a POC demonstrating the opportunity. It bypasses rustc semantics and
cannot safely handle all language constructs. The POC is the argument for
investing in a proper rustc-native implementation.

### How is this different from dead code elimination?

DCE operates after full compilation. Slicing reduces work *before* compilation —
parsing, name resolution, type-checking, and macro expansion are all cheaper on
a smaller item set.

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
