# Project goal - High-Level ML optimizations

| Metadata            |              |
|:--------------------|--------------|
| Point of contact    | @ZuseZ4      |
| Status              | Proposed     |
| Tracking issue      |              |
| Zulip channel       | N/A          |
| [lang] champion     | @traviscross |
| [compiler] champion | @oli-obk     |

## Motivation

### The status quo

rustc does an excellent job generating efficient llvm-ir, competing with languages like C++ or Fortran. Over the last years, we also started leveraging LLVM based automatic differentiation and gpu support. `std::autodiff` just got enabled in nightly, and `std::offload` wil soon follow once we update rustc to LLVM 22. We will develop further LLVM optimizations targeting offload performance in Rust, and we will continue to accept gsoc students for both projects, to grow the maintainer pool. After having worked on the autodiff frontend for 5 years, the offload frontend for 1 year, and both projects landing in nightly, I feel like this is a good moment to slow down on frontend changes for both projects, and give users a chance to catch up, test it, and provide feedback. At the same time, we can start thinking about future paths to stabilization, see Work item `3)` and the FAQ [2] for details.

Together, those two projects should enable projects in the fields of high-performance computing and scientific computing, which rely on GPU support and or gradients for their workloads. However, there is still a big piece missing, which is especially relevant for Machine Learning workloads: Tensor/Matrix optimizations. I care about three specific aspects here:
1) Run matrix operations on accelerators (e.g. TPUs, Trainium/Inferentium, LPU, etc.)
2) Recognize individual high-level matrix operations (e.g. matmul, conv2d, etc.) in code, and replace them with more efficient implementations.
3) Optimize sequences of high-level matrix operations, e.g. by reordering, fusing, or replacing them.

Accelerator vendors often see MLIR as the expected input format for their hardware. MLIR also has various dialects which are suitable to represent and optimize these high-level operations. LLVM has `Polly` as a subproject which also targets higher-level optimizations, but it is more limited as discussed in the FAQ. MLIR on the other hand has it's own challenges, mainly related to a higher development speed and few stability guarantees. 

### What we propose to do about it

We are working towards a shiny future where developers are able to develop full ML models in pure Rust. Based on their available hardware, training can happen on GPUs, other accelerators like TPUs, or if desired, still CPUs. Developers don't need to think about "gradients", "derivatives" or a "reverse pass", since in all cases the `std::autodiff` module can compute the gradients needed during the training for them. Beyond that, users can write their inference or training logic only considering a single training example, since the compiler can automatically batch their code for them, to enable vectorization or improve throughput. 

We want to explore the possibility of building an MLIR based backend for rustc. This MVP intentionally limits complexity in some areas, to be able to focus on the hard parts first. We especially intend to limit us here:
1) Target only one accelerator, since targeting the following ones should be easier. The std::offload project currently only supports AMD+NVIDIA GPUs, but this was enough to motivate a new contributor to reach out and start working on an Intel GPU target. We expect similar effects once we demonstrate how to target the first accelerator via MLIR.
2) No bikeshedding about which dialects to target. MLIR (Multi-Level IR) lives from its dialects, it is common to raise or lower between multiple dialects in one compilation pipeline to benefit from all of them. Once we lower to one dialect it should therefore be easy to replace that dialect or add additional ones. 
3) No full Rust support. It would be infeasible for one person (plus collaborators) to achieve this in one year, but it's also not needed, since most of the ML runtime will be focused around a quite limited set of operations.

We instead intend to tackle these hard problems first:

1) Get the full infrastructure up. Previous experience from Enzyme and LLVMs Offload integration should help here.
2) Enable splitting the compilation between the MLIR accelerated part and the current LLVM based backend. This has some overlap with `std::offload`, which uses two compilation passes to compile for the host (CPU) and the device (GPU).
3) Support MLIR without pushing a full DSL into Rust. Targeting MLIR could be comparably easy if we would require the usage of a special Matrix type, and if we add all functions from the chosen MLIR dialects into the standard library. However, blessing one specific matrix implementation prevents competition (see e.g. faer/ndarray/nalgebra). There is also a very large number of MLIR dialects with different stability guarantees, so we would significantly bloat our standard library and might struggle with breaking changes later.
4) Demonstrate the combination of std::autodiff, std::offload, and the new MLIR backend. We are already able to combine the first two features, but only if we can combine all three we have all the basics needed to support modern ML workloads.

There is a chance that we need to compromise on target 3), but I believe it's in our best interest to first aim for the best case. In the case of `std::autodiff` and `std::offload` we have already demonstrated that its possible to differentiate and offload almost arbitrary Rust code. I hope that we can achieve the same with this goal and even if not, we at least have a full prototype based on which we can start a discussion about which compromises are acceptable.

### Work items over the next year

1) Continue maintaining std::autodiff (whole year).

2) Support @karolzwolak who is working on adding a new Intel GPU target, to allow `std::offload` on all three vendors (AMD, NVIDIA, Intel) (Winter+Spring).

3) Discuss the memory model of `std::offload` with the lang team, to identify potential issues and stabilization blockers (whole year, "background task").

4) **Develop the MLIR prototype (from Spring onwards, main focus)**. Target one accelerator and accelerate one common instruction (e.g. matmul). Demonstrate one high-level optimization.

5) Continue growing a contributor team accross Rust and LLVM to maintain `std::autodiff`, `std::offload`, and our new MLIR backend (whole year).

## Team asks

| Team       | Support level | Notes                                   |
| ---------- | ------------- | --------------------------------------- |
| [compiler] | Medium | My changes should be contained to few places in the compiler. Potentially one frontend macro/intrinsic, and otherwise almost exclusively in the backend. |
| [infra]    |  Small | I will work with @Kobzol to add more bootstrap options to build and configure MLIR (an LLVM subproject)                                         |
| [lang]     | Medium  | Discussions to understand which parts of gpu programming and `std::offload` are problematic wrt. stabilization, from a lang perspective. Non-blocking, since we are not rushing stabilization.         |

## Frequently Asked Questions (FAQ)

### Why can these features not be implemented in crates, why does it need compiler support?

A: automatic differentiation must be performed on optimized code ("Intermediate Representation", IR) to be performant (in the general case). Crates are generally unable to modify their own IR, unless reimplementing parts of a compiler. Further, both `autodiff` and `offload` provide their own improvements to the LLVM optimization pipeline, used by the Rust compiler. Both projects further rely on various unstable compiler internals (e.g. the layout of non-repr-C types).

### What are related crates or projects?

A: The basic features discussed (autodiff, batching, gpus, kernel optimizations) are common accross ML Compilers/Frameworks in Rust and other languages. Examples are PyTorch, JAX, Burn and Candle (Rust) and Reactant.jl. 

Rust did not provide any of the required features so far, forcing Rust ML projects like Burn or Candle to call out to other languages or even reimplement aspects of a compiler, without having the full features available that we can use within rustc. Based on these limitations, their autodiff implementation is (based on previous communication) not competitive on general scientific or HPC code (like e.g. measured by ADBench). Their GPU and autodiff implementation are further tied to their own Tensor type, which significantly limits it's reusability within the larger crates ecosystem. If we as Rust compiler start providing these features, ML framworks don't have to keep re-implementing gradient computations anymore, won't need to call other languages, can provide consistently good/better performance over a larger area of workloads, and interoperate better with other crates by not having to enforce the usage of specific types or traits.

### What are some related proposals?

Request A: Better compile time support. Users should be able to enforce that certain code sections are evaluated at compile time. This can give peace of mind to developers who want to ensure that within hot code sections, some expressions are already evaluated at compile time. It can further prevent bugs if developers can enforce that certain constraints (e.g. matrix dimensions) at compile times. 

Request B: Better compile times. Training a Neural Network from scratch is going to overshadow any compile times. However, common workflows also include using a pre-trained model and tinkering with it (e.g. postprocessing some output, printing or visualizing it, updating only parts of your surrounding simulation, etc.). Such tasks are usually done in an interactive language like Python or Julia. How close can we get our compile times to such Interpreted/JIT uses? Promising candidates are Wild (a parallel linker), [TPDE](github.com/rust-lang/google-summer-of-code/pull/46) (a faster LLVM debug backend), the parallel Frontend/Macro Expansion, and others.

Request C (optional): The Reflection proposal might be one potential path to stabilize the autodiff and offload projects by "outsourcing" it into common crates, *if* we decide to previde the necessary abilities via a future reflection implementation. We should sync up!

### What are the paths towards stabilization?
    
A) `std::offload`. This project is based on LLVM's upstream offload/openmp project, which are sufficiently tested via C++ and Fortran. It also seems reasonable to support it in the GCC backend in the future. While we want to give nightly users plenty of time to test this feature and provide feedback, we also will start a discussion with the lang team to identify and hopefully remove potential stabilization blockers. We do not intend to request a stabilizatoin during this Project Goal Period.
    
B) `std::autodiff`. This project is based on an LLVM incubator project and not part of upstream LLVM yet. It is also heavily research focused, so we expect more bugs than in the `std::offload` project. Before discussing stabilization, we therefore want to explore paths to improve the reliability of this project. Potential paths forward include working with Enzyme developers to achieve better support with bugfixes affecting Rust, training Rust developers to improve Enzyme, or understanding that reimplementing a smaller and more production focused AD tool in the style of Enzyme (based on a potential LIR) is the most efficient path forward.
    
C) **Both:** the `autodiff` and `offload` project both can offer performance or features that are not achievable by normal crates. What if we focus on enabling normal crates to implement this type of project, instead of adding them to the library? We currently require certain abilities like looking up the layout of unstable types, or rewriting the backend representation (LLVM-IR) of certrain functions and objects. We expect that such abilities could be helpful beyond our projects. While it's beyond the scope of the reflection MVP, we will also reach out to that project to discuss enabling these abilities for normal crates. Such advanced reflection would enable "outsourcing" `autodiff` and `offload` from the standard library into normal crates, which then would not need to follow the traditional stabilization paths. It would however introduce new problems, for example each crate release would be tied to specific Rust compiler versions, due to the LLVM version vendored by rustc.
