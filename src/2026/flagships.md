# Rust in 2026

Flagship themes are long-running efforts that span multiple goal periods. Each theme represents a vision for where Rust is headed, with concrete milestones for this year.

* [Beyond the `&`](./flagship/beyond-the-ampersand.md): Smart pointers that feel as natural as `&` and `&mut`. Key 2026 milestones include:
    * experimental support for [field projections](./field-projections.md),
    * progress on [reborrow traits](./reborrow-traits.md),
    * design alignment on [in-place initialization](./in-place-init.md).
* [Unblocking dormant traits](./flagship/unblocking-dormant-traits.md): Lending iterators, extern types, scalable vectors, and evolvable trait hierarchies. Key 2026 milestones include:
    * stabilize the [next-generation trait solver](./next-solver.md),
    * stabilize the [Sized hierarchy](./scalable-vectors.md).
* [Constify all the things](./flagship/constify-all-the-things.md): Structs and associated constants in generics, compile-time type introspection. Key 2026 milestones include:
    * stabilize [const generics](./const-generics.md) extensions,
    * prototype [reflection](./reflection-and-comptime.md).
* [Higher-level Rust](./flagship/higher-level-rust.md): Single-file scripts with dependencies, ergonomic ref-counting. Key 2026 milestones include:
    * stabilize [cargo-script](./cargo-script.md),
    * prototype the [Share trait](./ergonomic-rc.md).
* [Secure your supply chain](./flagship/secure-your-supply-chain.md): Control over public API dependencies, breaking change detection, SBOM generation. Key 2026 milestones include:
    * stabilize [public/private dependencies](./pub-priv.md),
    * stabilize [SBOM support](./stabilize-cargo-sbom.md).
* [Building blocks](./flagship/building-blocks.md): Rebuild std with custom flags, integrate Cargo into larger build systems, better test tooling. Key 2026 milestones include:
    * design work on [build-std](./build-std.md),
    * prototype [cargo plumbing commands](./cargo-plumbing.md).
