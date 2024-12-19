# Expose experimental LLVM features for GPU offloading

| Metadata              |                                                  |
|-----------------------|--------------------------------------------------|
| Point of contact | @ZuseZ4                                          |
| Teams                 | [lang], [compiler]                               |
| Status                | Proposed                                         |
| Tracking issue        | [rust-lang/rust-project-goals#109]               |
| Other tracking issues | [rust-lang/rust#124509], [rust-lang/rust#124509] |


## Summary

Expose experimental LLVM features for GPU offloading and allow combining it with the `std::autodiff` feature.

## Motivation

Scientific computing, high performance computing (HPC), and machine learning (ML) all share the interesting challenge in that they each, to different degrees, care about highly efficient library and algorithm implementations, but that these libraries and algorithms are not always used by people with deep experience in computer science. Rust is in a unique position because ownership, lifetimes, and the strong type system can prevent many bugs. At the same time strong alias information allows compelling performance optimizations in these fields, with performance gains well beyond that otherwise seen when comparing C++ with Rust. This is due to how automatic differentiation and GPU offloading strongly benefit from aliasing information.

### The status quo

Thanks to PyO3, Rust has excellent interoperability with Python. Conversely, C++ has a relatively weak interop story. This can lead Python libraries to using slowed C libraries as a backend instead, just to ease bundling and integration. Fortran is mostly used in legacy places and hardly used for new projects.

As a solution, many researchers try to limit themself to features which are offered by compilers and libraries built on top of Python, like JAX, PyTorch, or, more recently, Mojo. Rust has a lot of features which make it more suitable to develop a fast and reliable backend for performance critical software than those languages. However, it lacks GPU support which developers now expect.  

Almost every language has some way of calling hand-written CUDA/ROCm/Sycl kernels, but the interesting feature of languages like Julia, or of libraries like JAX, is that they offer users the ability to write kernels in the language the users already know, or a subset of it, without having to learn anything new. Minor performance penalties are not that critical in such cases, if the alternative are a CPU-only solution. Otherwise worthwhile projects such as Rust-CUDA end up going unmaintained due to being too much effort to maintain outside of LLVM or the Rust project.

*Elaborate in more detail about the problem you are trying to solve. This section is making the case for why this particular problem is worth prioritizing with project bandwidth. A strong status quo section will (a) identify the target audience and (b) give specifics about the problems they are facing today. Sometimes it may be useful to start sketching out how you think those problems will be addressed by your change, as well, though it's not necessary.*

### The next six months

We are requesting support from the Rust project for continued experimentation:

1) Merge an MVP `#[offloading]` fork which is able to run simple functions using rayon parallelism on a GPU, showing a speed-up.
2) Show an example of how to combine `#[offloading]` with `#[autodiff]` to run a differentiated function on a GPU.

### The "shiny future" we are working towards

The purpose of this goal is to enable continued experimentation with the underlying LLVM functionality.
The eventual goal of this experimentation is that three important LLVM features (batching, autodiff, offloading) can be combined and work nicely together. The hope is that we will have state-of-the-art libraries like faer to cover linear algebra, and that we will start to see more and more libraries in other languages using Rust with these features as their backend. Cases which don't require interactive exploration will also become more popular in pure Rust.

#### Caveats to this future

There is not yet consensus amongst the relevant Rust teams as to how and/or whether this functionality should be exposed on stable.
Some concerns that continued experimentation will hopefully help to resolve:

* How effective and general purpose is this functionality?
* How complex is this functionality to support, and how does that trade off with the value it provides? What is the right point on the spectrum of tradeoffs?
* Can code using these Rust features still compile and run on backends other than LLVM, and on all supported targets? If not, how should we manage the backend-specific nature of it?
* Can we avoid tying Rust features too closely to the specific properties of any backend or target, such that we're confident these features can remain stable over decades of future landscape changes?
* Can we fully implement every feature of the provided functionality (as more than a no-op) on fully open systems, despite the heavily proprietary nature of parts of the GPU and accelerator landscape?

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

- `std::autodiff` has been upstreamed as part of the last Project Goal. There are till open PRs under review, but I expect them to be merged still in 2024. 
- Currently we work on adding custom-derivatives and will upstream support for batching/vectorization next, but both will be small PRs once the basic infrastructure is in place.
- Some features like safety checks or "TypeTrees" which will improve performance and catch usage mistakes were removed from the previous upstreaming PRs to make reviewing easier. We will upstream them at the side, but those are only 100-300 loc each, and thus should be easy to review.

*Add your [design axioms][da] here. Design axioms clarify the constraints and tradeoffs you will use as you do your design work. These are most important for project goals where the route to the solution has significant ambiguity (e.g., designing a language feature or an API), as they communicate to your reader how you plan to approach the problem. If this goal is more aimed at implementation, then design axioms are less important. [Read more about design axioms][da].*

[da]: ../about/design_axioms.md

## Ownership and team asks

**Owner:** @ZuseZ4

Manuel S. Drehwald is working 5 days per week on this, sponsored by LLNL and the University of Toronto (UofT). He has a background in HPC and worked on a Rust compiler fork, as well as an LLVM-based autodiff tool for the last 3 years during his undergrad. He is now in a research-based master's degree program. Supervision and discussion on the LLVM side will happen with Johannes Doerfert and Tom Scogland.

Minimal "smoke test" reviews will be needed from the compiler-team. The Rust language changes at this stage are expected to be a minimal wrapper around the underlying LLVM functionality and the compiler team need only vet that the feature will not hinder usability for ordinary Rust users or cause undue burden on the compiler architecture itself. There is no requirement to vet the quality or usability of the design.

| Task                 | Owner(s) or team(s)    | Notes      |
|----------------------|------------------------|------------|
| Development          | @ZuseZ4                |            |
| Lang-team experiment | ![Team][] [lang][]     | (approved) |
| Standard reviews     | ![Team][] [compiler][] |            |

[Team]: https://img.shields.io/badge/Team%20ask-red

## Outputs and milestones

### Outputs

- An `#[offload]` rustc-builtin-macro which makes a function definition known to the LLVM offloading backend.
  - [x] Made a PR to enable LLVM's offloading runtime backend.
  - [ ] Merge the offload macro frontend
  - [ ] Merge the offload Middle-end

- An `offload!([GPU1, GPU2, TPU1], foo(x, y,z));` macro (placeholder name) which will execute function `foo` on the specified devices.

- An `#[autodiff]` rustc-builtin-macro which differentiates a given function.
  - [x] Merge the Autodiff macro frontend
  - [x] Merge the Autodiff Enzyme backend
  - [ ] Merge the Autodiff Middle-end

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

### How about Safety?

We want all these features to be safe by default, and are happy to not expose some features if the gain is too small for the safety risk. As an example, Enzyme can compute the derivative with respect to a global. That's probably too niche, and could be discouraged (and unsafe) for Rust.
