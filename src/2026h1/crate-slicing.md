# Crate Slicing for Faster Fresh Builds

| Metadata |  |
| --- | --- |
| Point of contact | @yijunyu |
| Status | Proposed |
| Zulip channel | TBD |

## Summary

Research and prototype "crate slicing" — a static analysis technique that computes the transitive closure of items actually used from dependency crates and generates minimal sliced versions, reducing frontend parsing and type-checking overhead during fresh builds.

## Motivation

### The status quo

Rust compilation remains I/O and CPU-bound on dependency crates during fresh builds. Consider a typical async web service:

```toml
[dependencies]
tokio = { version = "1.48", features = ["full"] }
axum = "0.7"
serde = { version = "1", features = ["derive"] }
sqlx = { version = "0.8", features = ["postgres", "runtime-tokio"] }
```

This dependency graph expands to 161 crates. The `tokio` crate alone contains 315 module files totaling 92,790 lines of Rust source. Yet an application may only use 
a fraction of tokio's public API surface (e.g., `tokio::spawn`)， and some may depend only on a subcrate e.g., `tokio-macros` rather than on the main crate (as did by
the `cargo-slicer tool`).

The Rust compiler is designed to find and diagnose errors first and generate good code quickly second, hence it needs to do the following steps:
1. **Parse** all the ~90,000 lines
2. **Resolve** names and expand macros across all modules and submodules
3. **Type-check** all items, including unused ones
4. **Monomorphize** generic code (though only used instantiations)

Only step 4 naturally excludes unused code. Steps 1-3 process everything.

Current mitigations:
- **Incremental compilation**: Caches type-checking results but requires prior builds
- **sccache**: Caches compilation artifacts but requires cache hits
- **Cranelift**: Accelerates codegen but not frontend (steps 1-3)
- **Parallel frontend**: Parallelizes work but doesn't reduce total work
- **Hint mostly unused**: Avoid codegen from unused dependencies by hinting on LLVM linker

None of these reduce the fundamental quantity of source code processed during fresh builds.

### The next 6 months

We propose to research crate slicing: statically analyzing which items a project uses from each dependency and generating minimal crate versions containing only those items plus their transitive dependencies.

**Technical approach:**

1. **Usage extraction**: Parse project source using a customized `rust-analyzer` with its `SCIP` crate to identify:
   - Direct type references, e.g., `tokio::net::TcpListener`
   - Use statements, e.g., `use axum::{Router, routing::get}`
   - Method calls via variable type tracking, e.g., `listener.accept()` → `TcpListener::accept`
   - Trait bounds, e.g., `impl<T: Serialize>` → `serde::Serialize`

2. **Crate indexing**: Build a dependency graph of all items in the target crate:
   - Map each `pub` item to its defining module
   - Track `pub use` re-exports through the module tree
   - Evaluate `#[cfg(...)]` expressions against enabled features
   - Handle `#[path = "..."]` module redirections

3. **Transitive closure**: Compute minimal item set:
   - Include all trait impls for used types
   - Follow type definitions for struct fields and enum variants
   - Expand `pub use crate::*` glob re-exports
   - Preserve visibility modifiers for cross-module access
   - Include document cooments and attributes related to the preserved items

4. **Sliced crate generation**: Emit minimal source:
   - Copy only needed module files
   - Filter items within each module
   - Generate `Cargo.toml` with subset of features/dependencies
  
5. **Rustc verification**: For the soundness and correctness with respect to type inferencing on trait bounds, e.g., simply slicing away everything unused may lead to errors unless we have considered the corner cases. To make sure
   it generates correct results, we will use Rust compiler to double check the decisions made at the testing phase, before release the solution as replacement of the Rust compiler.

**Prototype results (December 2025):**

We have an early implementation (`cargo-slicer`) just demonstrates feasibility on major ecosystem crates:

| Crate | Version | Original Modules | Original LOC | Slice Time | Status |
|-------|---------|------------------|--------------|------------|--------|
| tokio | 1.48.0 | 315 | ~90,000 | — | ✅ Compiles |
| actix-web | 4.12.1 | 87 | ~28,000 | — | ✅ Compiles |
| hyper | 1.8.1 | 51 | ~19,000 | — | ✅ Compiles |
| axum | 0.7.9 | 45 | ~15,000 | — | ✅ Compiles |
| reqwest | 0.12.28 | 26 | ~13,000 | — | ✅ Compiles |
| syn | 2.0.x | 47 | ~15,000 | — | ✅ Compiles |
| rand | 0.9.2 | 31 | ~8,000 | — | ✅ Compiles |
| regex | 1.x | 21 | ~6,000 | 0.147s | ✅ Compiles |

Measured slice times for smaller utility crates:

| Crate | Items Needed | Module Files | Lines Generated |
|-------|--------------|--------------|-----------------|
| regex |309 | 20 | 10,671 |
| memchr | 8 | 35 | 14,475 |
| anyhow | 147 | 11 | 4,412 |
| futures | 3 | 0 | 91 |
| once_cell | 2 | 0 | 29 |
| thiserror | 2 | 0 | 28 |

The prototype handles complex patterns:
- **Feature flag evaluation**: Parses `#[cfg(all(feature = "std", not(feature = "parking_lot")))]` expressions and evaluates against cargo metadata
- **Facade crates**: Handles `futures` (re-exports from `futures-*`) and `sqlx` (facade over `sqlx-core`)
- **Platform-specific code**: Filters `#[cfg(unix)]` / `#[cfg(windows)]` based on target
- **Proc-macro crates**: Preserves as external dependencies (not sliced)
- **Re-export chains**: Traces `pub use self::module::Item` through module hierarchy

**Research goals for 2026H1:**

1. **Quantify compilation time reduction**: Benchmark `cargo build` with sliced vs. original dependencies across diverse projects (CLI tools, web services, embedded)

2. **Formalize soundness criteria**: Under what conditions is `sliced_crate` semantically equivalent to `original_crate` for a given usage set? Key concerns:
   - Trait coherence: Are all required impl blocks included?
   - Monomorphization: Does slicing affect which generic instantiations exist?
   - Build scripts: How to handle `build.rs` code generation?

3. **Evaluate integration architectures**:
   - **Cargo plugin**: `cargo slice` generates sliced deps before `cargo build`
   - **Build script**: Slice at `build.rs` time with caching
   - **Registry proxy**: Serve pre-sliced crates based on declared usage
   - **Compiler integration**: rustc-assisted slicing via query system

4. **Identify fundamental limitations**: Which crate patterns cannot be safely sliced?
   - Proc-macro crates (execute at compile time)
   - Heavy `build.rs` generators
   - `#[doc(hidden)]` internal APIs used via macros

5. **Engage compiler/cargo teams**: Present findings and gather feedback on viable integration paths

### The "shiny future"

The end state: `cargo build` automatically slices dependencies based on static usage analysis, achieving:

- **30-50% reduction in fresh build time** for dependency-heavy projects
- **Faster CI pipelines**: Cold builds complete much faster
- **No ecosystem changes required**: Works with existing crates.io crates
- **Slicing on transitive dependencies**: cache the SCIP dump of frequently used (stably dependent) crates by large project such as Zed

This complements ongoing efforts:
- **Parallel frontend** reduces wall-clock time; slicing reduces total work
- **Cranelift** accelerates codegen; slicing reduces frontend overhead


## Design notes

### Prior art: C/C++ precompilation

This research extends techniques proven effective for C/C++:

**ICSM 2005** [1]: Demonstrated function-level compilation units for faster incremental builds. Changing one function recompiles only that unit.

**TR2012** [2]: Formalized CTags-based splitting architecture. Identified that preprocessing overhead was the bottleneck — AWK scripts took 4.68s to process 2,000 LOC.

**precc (2024-2025)**: Rewrote AWK to Rust, achieving **200× preprocessing speedup** (4.68s → 0.022s). Achieved **1.8× fresh build speedup** on Vim codebase (125 files, 100K LOC):

| Mode | Time | Compilation Units | Speedup |
|------|------|-------------------|---------|
| Baseline (gcc -j48) | 15s | 125 | 1.0× |
| Passthrough (precc) | 8.5s | 125 | **1.8×** |
| Split (fine-grained) | 12s | 8,311 | 1.25× |

Key insight: Preprocessing overhead must be < 10% of compilation time for fresh build benefits.

### Rust-specific considerations

| Aspect | C/C++ | Rust | Implication |
|--------|-------|------|-------------|
| Compilation unit | .c file | Crate | Slicing operates at crate granularity |
| Name resolution | Headers + includes | Module system | Must trace `pub use` chains |
| Generics | Templates (header-only) | Monomorphization | Only used instantiations codegen'd |
| Trait impls | N/A | Coherence rules | Must include all impls for used types |
| Conditional compilation | `#ifdef` | `#[cfg]` + features | Must evaluate feature expressions |
| Macros | Preprocessor (textual) | Proc-macros (semantic) + `macro_rules!` | Proc-macro crates kept as-is |

### Technical challenges solved

The prototype required solving several non-trivial problems:

1. **Cfg expression parsing**: Hand-rolled parser for `all()`, `any()`, `not()`, `feature = "x"`, `target_os = "linux"` expressions

2. **Feature resolution**: Query cargo metadata for enabled features, expand `default` features from crate's own Cargo.toml

3. **Module path resolution**: Handle `#[path = "imp_std.rs"]` with cfg gates like `#[cfg(feature = "std")]`

4. **Re-export tracing**: Follow `pub use self::module::Item` through arbitrary nesting

5. **Trait coherence**: Conservative inclusion of all trait impls where the implementing type or trait is used

6. **Macro handling**: Skip `macro_rules!` bodies during re-export extraction; preserve proc-macro crate references by utilizing a function similar to `cargo-expand`

### Open research questions

1. **Monomorphization interaction**: Does slicing reduce monomorphization work (fewer generic items) ?

2. **Incremental compilation interaction**: Do sliced crates have smaller/faster incremental compilation fingerprints?

3. **Build determinism**: Is sliced output deterministic across platforms and Rust versions?

4. **Semver compatibility**: If a crate updates and the sliced subset is unchanged, can we skip recompilation?

## Task list

| Task | Owner | Status |
|------|-------|--------|
| Benchmark sliced crate compilation (10 diverse projects) | @yijunyu | In progress |
| Formalize soundness requirements (trait coherence, visibility) | @yijunyu | Not started |
| Prototype cargo integration (plugin or build script) | @yijunyu | Not started |
| Document cfg expression evaluation algorithm | @yijunyu | Partial |
| Measure rust-analyzer indexing time with sliced deps | TBD | Not started |
| Present research findings to cargo team | TBD | Not started |
| Write technical paper on crate slicing | @yijunyu | Not started |

## Team asks

| Team | Support level | Notes |
|------|---------------|-------|
| compiler | Medium | Consultation on approach feasibility and soundness concerns |
| cargo | Medium | Discussion on cargo integration options |
| types | Small | Consultation on trait coherence requirements for slicing |
| rust-analyzer | Small | Discussion on rust-analyzer integration potential |
| lang | Small | Discussion on review of research methodology and findings |

## Frequently asked questions

### What exactly is being "sliced"?

Source code at the item level. Given:

```rust
// In dependency crate `foo`
pub struct Used { ... }      // Kept: referenced by project
pub struct Unused { ... }    // Removed: never referenced
pub fn helper() { ... }      // Removed: never called
impl Used {
    pub fn needed(&self) { } // Kept: called by project
    pub fn extra(&self) { }  // Removed: never called
}
impl Unused { ... }          // Removed: type not used
impl Display for Used { }    // Kept: trait impl for used type
```

The sliced crate contains only `Used`, `Used::needed`, and the `Display` impl.

### How is this different from dead code elimination?

Dead code elimination (DCE) happens during codegen for the final binary. Slicing happens *before compilation*, reducing:
- Parsing time (fewer tokens to lex/parse)
- Name resolution (smaller module tree)
- Type checking (fewer items to check)
- Macro expansion (fewer macro invocations)

DCE doesn't help with these frontend costs.

### How is this different from feature flags?

Feature flags require crate authors to manually partition their API:

```toml
[features]
http1 = []
http2 = ["h2"]
full = ["http1", "http2", "websocket" ] # ... and so forth
```

This is labor-intensive, imperfect, and doesn't capture per-project usage patterns. Slicing is automatic and usage-based.

### What about proc-macro crates?

Proc-macro crates (like `serde_derive`, `tokio-macros`) are small and execute during compilation rather than being compiled into the output. They're preserved as-is and referenced as external dependencies in sliced crates.

### Won't sliced crates break things?

The research phase will establish correctness criteria. The prototype uses conservative inclusion:
- All trait impls for any used type or trait
- All items in the dependency closure
- All visibility requirements satisfied

Current prototype compiles 100% of tested crates (8/8 major async crates, 9/9 utility crates).

### What's the expected speedup?

Preliminary hypothesis based on:
- C/C++ precompilation achieved 1.8× speedup when preprocessing overhead was eliminated
- Many projects use <20% of dependency API surface
- Frontend (parsing + type-checking) is ~30-50% of compilation time

We hypothesize 20-40% reduction in fresh build time for dependency-heavy projects. The research phase will produce concrete measurements across diverse workloads.

### How does this interact with caching?

Complementary:
- **sccache**: Caches compiled artifacts; slicing reduces what needs caching
- **Incremental**: Caches type-check results; sliced crates have smaller fingerprints
- **cargo-chef**: Caches dependency layer in Docker; sliced layer is smaller

### What about crates that can't be sliced?

Known limitations:
- **Proc-macro crates**: Keep as-is (small anyway)
- **Heavy build.rs**: May generate code not visible to static analysis
- **`#[doc(hidden)]` internals**: Macros may use these; conservative inclusion needed
- **Blanket impls**: `impl<T> Trait for T` must be included if `Trait` is used

These are documented and handled gracefully (fall back to full crate).

## References

[1] Y. Yu, H. Dayani-Fard, J. Mylopoulos, P. Andritsos. "Reducing Build Time through Precompilations for Evolving Large Software." ICSM 2005.
[2] Y. Yu. "Precompilation: Splitting C/C++ Source Files for Faster Incremental Builds." Open University TR2012/01.
[3] demo-precc: https://github.com/yijunyu/demo-precc (binaries for C/C++ precompilation)
[4] cargo-slicer: https://github.com/yijunyu/cargo-slicer (proof-of-concept implementation of the proposed prototype)
