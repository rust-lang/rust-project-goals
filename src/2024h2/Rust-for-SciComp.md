# Scientific Computing in Rust

| Metadata | |
| --- | --- |
| Owner(s) | ZuseZ4 / Manuel S. Drehwald |
| Teams | t-lang, t-compiler |
| Status | WIP |

## Motivation

Scientific Computing, High Performance Computing, and Machine Learning share the interesting challenge that they (to different degrees) care about (very) efficient library and algorithm implementations, but are not always used by people experienced in computer science. Rust is in a nice position because Ownership, Lifetimes, and the strong type system can prevent a descent amount of bugs. At the same time strong alias information allows very nice performance optimizations in these fields, with performance gains well beyond what you see in normal C++ vs. Rust Performance Comparisons. This is the case because these fields often use Automatic Differentiation and GPU/Co-Processor offloading, both cases which benefit strongly from knowing which pointers or references alias.

### The status quo

Rust has an excellent Python InterOp Story thanks to PyO3. C++ has a weak interop story and as such I've seen cases where a slower C library is used as the backend for some Python libraries, because it was easier to bundle. Fortran is mostly used in legacy places and hardly used for new projects. As a solution, many researchers try to limit themself to features which are offered by compilers and libraries build on top of Python, like JAX, PyTorch, or newly Mojo. Rust has a lot of features which make it more suitable to develop a fast and reliable backend for performance critical software than those languages. However, it lacks features which developers now got used to. These features are *trivial GPU usage*. Almost every language has some way of calling hand-written CUDA/ROCm/Sycl Kernels, but the interesting feature of languages like Julia, or libraries like JAX is that they offer users to write Kernels in a (subset) of their already known language, without having to learn anything new. Minor performance penalties are not that critical in such cases, if the alternative are CPU only solution, because projects like Rust-CUDA end up being unmaintained due to being too much effort to maintain outside of the LLVM or Rust project.

*Elaborate in more detail about the problem you are trying to solve. This section is making the case for why this particular problem is worth prioritizing with project bandwidth. A strong status quo section will (a) identify the target audience and (b) give specifics about the problems they are facing today. Sometimes it may be useful to start sketching out how you think those problems will be addressed by your change, as well, though it's not necessary.*

### The next few steps

1) Merge the `#[autodiff]` fork.
2) Expose the experimental Batching feature of Enzyme, preferably by a new contributor.
3) Merge a MVP `#[offloading]` fork which is able to run simple functions using rayon parallelism on a GPU or TPU, showing a speed-up.

### The "shiny future" we are working towards

All three proposed features (batching, autodiff, offloading) can be combined and work nicely together. We have State-of-the-art libraries like faer to cover linear algebra and we start to see more and more libraries in other languages use Rust with these features as their backend. Cases which don't require interactive exploration also become more popular in pure Rust.

## Design axioms

### Offloading

- We try to provide a safe, simple and opaque offloading interface.
- The "unit" of offloading is a function.
- We try to not expose explicit data movement if Ownership gives us enough information.
- Users can offload functions which contains parallel CPU code, but do not have final control over how the parallelism will be translated to co-processors.
- We accept that hand-written CUDA/ROCm/.. Kernels might be faster, but actively try to reduce differences.
- We accept that we might need to provide additional control to the user to guide parallelism, if performance differences remain unacceptable large.
- Offloaded code might not return exact same values as code executed on the CPU. We will work with t-(opsem?) to develop clear rules.

### Autodiff

- We try to provide a fast autodiff interface which supports most autodiff features relevant for Scientific Computing.
- The "unit" of autodiff is a function.
- We acknowledge our responability since user-implemented autodiff without compiler knowledge might struggle to cover gaps in our features.
- We have a fast solution ("git plumbing") with further optimization opportunities, but need to improve safety and usability ("git porcelain").
- We need to teach users more about AutoDiff "pitfalls" and provide guides on how to handle them. [https://arxiv.org/abs/2305.07546](paper).
- We do not support differentiating (inline) assembly. Users are expected to write Custom-Derivatives in such cases.
- We might refuse to expose certain features if they are too hard to use correctly and provide little gains (e.g. derivatives with respect to global vars).

*Add your [design axioms][da] here. Design axioms clarify the constraints and tradeoffs you will use as you do your design work. These are most important for project goals where the route to the solution has significant ambiguity (e.g., designing a language feature or an API), as they communicate to your reader how you plan to approach the problem. If this goal is more aimed at implementation, then design axioms are less important. [Read more about design axioms][da].*

[da]: ../about/design_axioms.md

## Ownership and other resources

**Owner:** ZuseZ4 / Manuel S. Drehwald

Manuel S. Drehwald working 5 days/wk, sponsored by LLNL and the University of Toronto (UofT). He has a background in HPC and worked on a rust compiler fork, as well as an LLVM based autodiff tool for the last 3 years during his undergrad. He is now in a research based Master Program. Supervision and Discussion on the LLVM side with Johannes Doerfert and Tom Scogland.

Resources: Domain and CI for the autodiff work provided by MIT. Might be moved to the LLVM org later this year. Hardware for Benchmarks provided by LLNL and UofT. CI for the offloading work provided by LLNL or LLVM(?, see below).

### Support needed from the project

* Discussion on CI: It would be nice to test the Offloading support on at least all 3 mayor GPU Vendors. I am somewhat confident that I can find someone to set up something, but it would be good to discuss how to maintain this best in the longer term.

* Discussions on Design and Maintainability: I will probably keep asking questions to achieve a nice internal Design on zulip, which might take some time (either from lang/compiler, or other teams).

## Outputs and milestones

### Outputs

- An `#[Offload]` rustc-builtin-macro which makes a function definition known to the LLVM offloading backend.

- A bikeshead `offload!([GPU1, GPU2, TPU1], foo(x, y,z));` macro which will execute function foo on the specified devices.

- An `#[Autodiff]` rustc-builtin-macro which differentiates a given function.

- A `#[Batching]` rustc-builtin-macro which fuses N function calls into one call, enabling better vectorization.

### Milestones

- The first offloading step is the automatic copying of a slice or vector of floats to a Device and back.

- The second offloading step is the automatic translation of a (default) Clone implementation to create a Host2Device and Device2Host copy implementation for user types.

- The third offloading step is to run some embarrassingly parallel Rust code (e.g. scalar times Vector) on the GPU.

- Fourth we have examples of how rayon code runs faster on a co-processor using offloading.

- Stretch-goal. Combining Autodiff and Offloading in one example that runs differentiated code on a GPU.

## Frequently asked questions

### Why do you implement these features only on the LLVM Backend?

Performance wise we have LLVM and GCC as performant backends. Modularity wise we have LLVM and especially Cranelift being nice to modify. It seems reasonable that LLVM thus is the first backend to have support for new features in this field. Especially the offloading support should be supportable by other compiler backends, given pre-existing work like OpenMP Offloading and WebGPU.

### Do these changes have to happen in the compiler?

No! Both features could be implemented in user-space, if the Rust compiler would support Reflection. In this case I could ask the compiler for the optimized backend IR for a given function. I would then need use either the AD or Offloading abilities of the LLVM library to modify the IR, generating a new function. The user would then be able to call that newly generated function. This would require some discussion on how we can have crates in the ecosystem that work with various LLVM versions, since crates are usually expected to have a MSRV, but the LLVM (and like GCC/Cranelift) backend will have breaking changes, unlike Rust.

### Batching?

Offered by all autodiff tools, JAX has an extra command for it, whereas Enzyme (the autodiff backend) combines Batching with AutoDiff. We might want to split these since both have value on their own. Some libraries also offer Array-of-Struct vs Struct-of-Array features which are related but often have limited usability or performance when implemented in userspace. To be fair this is a less mature feature of Enzyme, so I could understand concerns. However, following the Autodiff work this feature can be exposed in very few (100?) loc. My main bieksheding thoughts where about whether we want to pass 3 batched args as [x1, x2, x3], (x1, x2, x3), or x1, x2, x3. Also it's a nice feature to get something started, once the main autodiff PR got merged.

### Writing a GPU Backend in 6 months sounds tough..

True. But similar to the Autodiff work I'm exposing something that's already existing in the Backend. I just don't think that Rust, Julia, C++, Carbon, Fortran, Chappel, Haskell, Bend, Python, ... should all write their own GPU or Autodiff Backends. Most of these already share compiler optimization through LLVM or GCC, so let's also share this. Of course, we should still push to use our Rust specific magic.

### Rust Specific Magic?

TODO

### How about Safety?

I want all these features to be safe by default, and I am happy to not expose some features if the gain is too small for the safety risk. As an Example, Enzyme can compute the derivative with respect to a global. Too niche, discouraged (and unsafe) for Rust. `¯\_(ツ)_/¯`

How to parallelize your 3 nested for loops efficiently has been researched for decades. Lately there also has been some more work on how to translate different parallelism type efficiently, e.g. from GPUs to CPUs, or now maybe some rayon parllelism to GPUs? I am therefore not particualrily worried about Correctness.

### What do I do with this space?

*This is a good place to elaborate on your reasoning above -- for example, why did you put the design axioms in the order that you did? It's also a good place to put the answers to any questions that come up during discussion. The expectation is that this FAQ section will grow as the goal is discussed and eventually should contain a complete summary of the points raised along the way.*
