# Expose experimental LLVM features for automatic differentiation and GPU offloading

| Metadata |                             |
| -------- | --------------------------- |
| Owner(s) | ZuseZ4 / Manuel S. Drehwald |
| Teams    | t-lang, t-compiler          |
| Status   | Proposed                         |

## Summary

Expose experimental LLVM features for automatic differentiation and GPU offloading.

## Motivation

Scientific computing, high performance computing (HPC), and machine learning (ML) all share the interesting challenge in that they each, to different degrees, care about highly efficient library and algorithm implementations, but that these libraries and algorithms are not always used by people with deep experience in computer science. Rust is in a unique position because ownership, lifetimes, and the strong type system can prevent many bugs. At the same time strong alias information allows compelling performance optimizations in these fields, with performance gains well beyond that otherwise seen when comparing C++ with Rust. This is due to how automatic differentiation and GPU offloading strongly benefit from aliasing information.

### The status quo

Thanks to PyO3, Rust has excellent interoperability with Python. Conversely, C++ has a relatively weak interop story.  This can lead Python libraries to using slowed C libraries as a backend instead, just to ease bundling and integration. Fortran is mostly used in legacy places and hardly used for new projects.

As a solution, many researchers try to limit themself to features which are offered by compilers and libraries built on top of Python, like JAX, PyTorch, or, more recently, Mojo. Rust has a lot of features which make it more suitable to develop a fast and reliable backend for performance critical software than those languages. However, it lacks two major features which developers now expect. One is high performance autodifferentiation. The other is easy use of GPU resources.

Almost every language has some way of calling hand-written CUDA/ROCm/Sycl kernels, but the interesting feature of languages like Julia, or of libraries like JAX, is that they offer users the ability to write kernels in the language the users already know, or a subset of it, without having to learn anything new. Minor performance penalties are not that critical in such cases, if the alternative are a CPU-only solution. Otherwise worthwhile projects such as Rust-CUDA end up going unmaintained due to being too much effort to maintain outside of LLVM or the Rust project.

*Elaborate in more detail about the problem you are trying to solve. This section is making the case for why this particular problem is worth prioritizing with project bandwidth. A strong status quo section will (a) identify the target audience and (b) give specifics about the problems they are facing today. Sometimes it may be useful to start sketching out how you think those problems will be addressed by your change, as well, though it's not necessary.*

### The next six months

1) Merge the `#[autodiff]` fork.
2) Expose the experimental batching feature of Enzyme, preferably by a new contributor.
3) Merge an MVP `#[offloading]` fork which is able to run simple functions using rayon parallelism on a GPU or TPU, showing a speed-up.

### The "shiny future" we are working towards

All three proposed features (batching, autodiff, offloading) can be combined and work nicely together. We have state-of-the-art libraries like faer to cover linear algebra, and we've started to see more and more libraries in other languages use Rust with these features as their backend. Cases which don't require interactive exploration will also become more popular in pure Rust.

## Design axioms

### Offloading

- We try to provide a safe, simple and opaque offloading interface.
- The "unit" of offloading is a function.
- We try to not expose explicit data movement if Rust's ownership model gives us enough information.
- Users can offload functions which contains parallel CPU code, but do not have final control over how the parallelism will be translated to co-processors.
- We accept that hand-written CUDA/ROCm/etc. kernels might be faster, but actively try to reduce differences.
- We accept that we might need to provide additional control to the user to guide parallelism, if performance differences remain unacceptably large.
- Offloaded code might not return the exact same values as code executed on the CPU. We will work with t-opsem to develop clear rules.

### Autodiff

- We try to provide a fast autodiff interface which supports most autodiff features relevant for scientific computing.
- The "unit" of autodiff is a function.
- We acknowledge our responability since user-implemented autodiff without compiler knowledge might struggle to cover gaps in our features.
- We have a fast, low level, solution with further optimization opportunities, but need to improve safety and usability (i.e. provide better high level interfaces).
- We need to teach users more about autodiff "pitfalls" and provide guides on how to handle them. See, e.g. <https://arxiv.org/abs/2305.07546>.
- We do not support differentiating inline assembly. Users are expected to write custom derivatives in such cases.
- We might refuse to expose certain features if they are too hard to use correctly and provide little gains (e.g. derivatives with respect to global vars).

*Add your [design axioms][da] here. Design axioms clarify the constraints and tradeoffs you will use as you do your design work. These are most important for project goals where the route to the solution has significant ambiguity (e.g., designing a language feature or an API), as they communicate to your reader how you plan to approach the problem. If this goal is more aimed at implementation, then design axioms are less important. [Read more about design axioms][da].*

[da]: ../about/design_axioms.md

## Ownership and team asks

**Owner:** ZuseZ4 / Manuel S. Drehwald

Manuel S. Drehwald is working 5 days per week on this, sponsored by LLNL and the University of Toronto (UofT). He has a background in HPC and worked on a Rust compiler fork, as well as an LLVM-based autodiff tool for the last 3 years during his undergrad. He is now in a research-based master's degree program. Supervision and discussion on the LLVM side will happen with Johannes Doerfert and Tom Scogland.

Resources: Domain and CI for the autodiff work is being provided by MIT. This might be moved to the LLVM org later this year. Hardware for benchmarks is being provided by LLNL and UofT. CI for the offloading work will be provided by LLNL or LLVM (see below).

| Subgoal | Owner(s) or team(s) | Notes |
| ---------------------------------- | ---------------------- | ---------- |
| Development                        | ZuseZ4                 |            |
| ↳ Lang-team experiment             | ![Team][] [Lang][]     | (approved) |
| ↳ "Smoke test" reviews (see below) | ![Team][] [Compiler][] |            |

[Team]: https://img.shields.io/badge/Team%20ask-red

Minimal "smoke test" reviews will be needed from the compiler-team. The Rust language changes at this stage are expected to be a minimal wrapper around the underlying LLVM functionality and the compiler team need only vet that the feature will not hinder usability for ordinary Rust users or cause undue burden on the compiler architecture itself. There is no requirement to vet the quality or usability of the design.

## Outputs and milestones

### Outputs

- An `#[offload]` rustc-builtin-macro which makes a function definition known to the LLVM offloading backend.

- An `offload!([GPU1, GPU2, TPU1], foo(x, y,z));` macro (placeholder name) which will execute function `foo` on the specified devices.

- An `#[autodiff]` rustc-builtin-macro which differentiates a given function.

- A `#[batching]` rustc-builtin-macro which fuses N function calls into one call, enabling better vectorization.

### Milestones

- The first offloading step is the automatic copying of a slice or vector of floats to a device and back.

- The second offloading step is the automatic translation of a (default) `Clone` implementation to create a host-to-device and device-to-host copy implementation for user types.

- The third offloading step is to run some embarrassingly parallel Rust code (e.g. scalar times Vector) on the GPU.

- Fourth we have examples of how rayon code runs faster on a co-processor using offloading.

- Stretch-goal: combining autodiff and offloading in one example that runs differentiated code on a GPU.

## Frequently asked questions

### Why do you implement these features only on the LLVM backend?

Performance-wise, we have LLVM and GCC as performant backends. Modularity-wise, we have LLVM and especially Cranelift being nice to modify. It seems reasonable that LLVM thus is the first backend to have support for new features in this field. Especially the offloading support should be supportable by other compiler backends, given pre-existing work like OpenMP offloading and WebGPU.

### Do these changes have to happen in the compiler?

Yes, given how Rust works today.

However, both features could be implemented in user-space if the Rust compiler someday supported reflection. In this case we could ask the compiler for the optimized backend IR for a given function. We would then need use either the AD or offloading abilities of the LLVM library to modify the IR, generating a new function. The user would then be able to call that newly generated function. This would require some discussion on how we can have crates in the ecosystem that work with various LLVM versions, since crates are usually expected to have a MSRV, but the LLVM (and like GCC/Cranelift) backend will have breaking changes, unlike Rust.

### Batching?

This is offered by all autodiff tools. JAX has an extra command for it, whereas Enzyme (the autodiff backend) combines batching with autodiff. We might want to split these since both have value on their own.

Some libraries also offer array-of-struct vs struct-of-array features which are related but often have limited usability or performance when implemented in userspace.

### Writing a GPU backend in 6 months sounds tough...

True. But similar to the autodiff work, we're exposing something that's already existing in the backend.

Rust, Julia, C++, Carbon, Fortran, Chappel, Haskell, Bend, Python, etc. should not all have write their own GPU or autodiff backends. Most of these already share compiler optimization through LLVM or GCC, so let's also share this. Of course, we should still push to use our Rust specific magic.

### Rust Specific Magic?

TODO

### How about Safety?

We want all these features to be safe by default, and are happy to not expose some features if the gain is too small for the safety risk. As an example, Enzyme can compute the derivative with respect to a global. That's probably too niche, and could be discouraged (and unsafe) for Rust.
