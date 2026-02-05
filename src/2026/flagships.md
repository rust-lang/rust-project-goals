# Rust in 2026

## Stabilizations in 2026

The following project goals involve new features or changes that will be stabilized (made available on the stable Rust channel) in 2026. You can click on each goal to learn more about it.

(((STABILIZATION SUMMARIES)))

## Flagship themes: looking to the future

Looking past 2026, we have a number of ongoing *flagship themes*. These themes capture the active work that is going on to evolve Rust and better meet the needs of our users.

* [Just Add Async](./flagship-just-add-async.md): Patterns that work in sync Rust should work in async Rust. Key 2026 milestones include:
    * stabilize [return type notation](./rtn.md),
    * stabilize [async fn in dyn trait](./afidt.md),
    * prototype [immobile types and guaranteed destructors](./move-trait.md),
    * prototype [ergonomic ref-counting](./ergonomic-rc.md).
* [Beyond the `&`](./flagship-beyond-the-ampersand.md): Smart pointers that feel as natural as `&` and `&mut`. Key 2026 milestones include:
    * experimental support for [field projections](./field-projections.md),
    * progress on [reborrow traits](./reborrow-traits.md),
    * design alignment on [in-place initialization](./in-place-init.md).
* [Unblocking dormant traits](./flagship-unblocking-dormant-traits.md): Lending iterators, extern types, scalable vectors, and evolvable trait hierarchies. Key 2026 milestones include:
    * stabilize the [next-generation trait solver](./next-solver.md),
    * stabilize the [Sized hierarchy](./scalable-vectors.md).
* [Constify all the things](./flagship-constify-all-the-things.md): Structs and associated constants in generics, compile-time type introspection. Key 2026 milestones include:
    * stabilize [const generics](./const-generics.md) extensions,
    * prototype [reflection](./reflection-and-comptime.md).
* [Higher-level Rust](./flagship-higher-level-rust.md): Single-file scripts with dependencies. Key 2026 milestones include:
    * stabilize [cargo-script](./cargo-script.md).
* [Secure your supply chain](./flagship-secure-your-supply-chain.md): Control over public API dependencies, breaking change detection, SBOM generation. Key 2026 milestones include:
    * stabilize [public/private dependencies](./pub-priv.md),
    * stabilize [SBOM support](./stabilize-cargo-sbom.md).
* [Safety-Critical Rust](./flagship-safety-critical-rust.md): Certified tooling, specifications, and evidence for functional safety. Key 2026 milestones include:
    * implement [MC/DC coverage support](./mcdc-coverage-support.md),
    * publish [normative unsafe documentation](./safe-unsafe-for-safety-critical.md),
    * establish [safety-critical lints in Clippy](./safety-critical-lints-in-clippy.md),
    * stabilize [FLS release cadence](./stabilize-fls-releases.md).
* [Building blocks](./flagship-building-blocks.md): Rebuild std with custom flags, integrate Cargo into larger build systems, better test tooling. Key 2026 milestones include:
    * design work on [build-std](./build-std.md),
    * prototype [cargo plumbing commands](./cargo-plumbing.md).
