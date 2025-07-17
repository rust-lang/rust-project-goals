# Finish the std::offload module

| Metadata              |                                                  |
| :-------------------- | -------------------------------------------------|
| Point of contact      | @ZuseZ4                                          |
| Teams                 | T-compiler                                       |
| Task owners           | @ZuseZ4                                          |
| Status                | Proposed                                         |
| Tracking issue        | [rust-lang/rust-project-goals#109]               |
| Other tracking issues | [rust-lang/rust#124509], [rust-lang/rust#124509] |
| Zulip channel         | [#wg-autodiff][channel]                          |

[channel]: https://rust-lang.zulipchat.com/#narrow/channel/390790-wg-autodiff

## Summary

This project aims to get the `std::offload` module finished. We currently have the ability to automatically move data to and from a GPU, but we can not launch kernels yet. We will add this ability during the next project goal period.

## Motivation

Scientific computing, high performance computing (HPC), and machine learning (ML) all share the interesting challenge in that they each, to different degrees, care about highly efficient library and algorithm implementations, but that these libraries and algorithms are not always used by people with deep experience in computer science. Rust is in a unique position because ownership, lifetimes, and the strong type system can prevent many bugs. At the same time strong alias information allows compelling performance optimizations in these fields, with performance gains well beyond that otherwise seen when comparing C++ with Rust. This is due to how automatic differentiation and GPU offloading strongly benefit from aliasing information.

## Status quo

The `std::autodiff` module is fully upstreamed, but not shipped on nightly yet to open CI questions. 
"Batched" autodiff, which supports array-of-struct or struct-of-array style code generation is also mostly upstreamed, but not yet publically advertised due to some open design questions and requested changes.
The standalone `std::batching` feature is mostly implemented in a PR, but not yet upstreamed due to requested changes, to better interact with Rust's SIMD types. I hope to train a contributor to implement the requested changes to increase the bus factor.
The `std::offload` feature is partly upstreamed. The "host" side, which handles the CPU code is ready. For the "device" side, a first PR exists, but is not yet reviewed or sufficiently tested. We also expect further follow-up PRs to expose more GPU features.

@oli-obk has done a great job reviewing my offload host PR to make sure that code quality matches rustc standards, but expressed that he isn't comfortable reviewing it on a technical side for correct LLVM/offload usage.
For that I collaborated with @jdoerfert and @kevinsala from the LLVM side. Further, individuals at both AMD and NVIDIA gave feedback on my design and will continue to do so.
In general, the design is somewhat different from what we have in other languages, so we will likely run into some challenges and keep iterating.

## The next 6 months

I will spend most of my time on the offload "Backend", especially the device side, to improve how we lower Rust code to GPUs. To verify the progress, I will add increasingly more complex gpu compute kernels to the rust test suite.
Similar to the autodiff work, I will spend a significant fraction of my time with onboarding new contributors, to increase the bus factor. So far I already have one offer from a potential contributor, 
which is interested in developing the offload frontend. Due to the popularity of GPU programming, I expect even more support than for the autodiff work and hope to have a solid contributor base by the end of the project goal.

One of the lessons learned from the `std::autodiff` work is the challenge of enabling a new feature in CI. While autodiff has been usable for more 6 months, we still do not ship it on nightly. The two main challenges were
A) challenges of reproducing CI issues locally B) enabling autodiff in CI increases the binary size.

Due to the lack of GPUs in CI we will not run GPU binaries in CI, which should avoid issue A)
To prevent issue B) I already opened a PR to enable `std::offload` in CI. This allows us to test the binary size increase early, and gives the infra team more time to provide feedback.

I expect that we will find a solution with the infra team at some point within the next 6 months to enable `std::autodiff` in CI. Once that happens, I will likely take a one month break from `std::offload`
to clean up autodiff docs and finish the upstreaming of `std::batching`, which is based on the same LLVM plugin as autodiff (Enzyme).

### The "shiny future" we are working towards

In the future, developers will be able to write a single Rust function and use `std::batching` to get a SIMD/fused version of it, use `std::autodiff` to differentiate it, and `std::offload` to run the resulting code on their GPUs.
Authors of Machine Learning or Linear Algebra libraries will further be able to optimize their libraries performance by opting into a new MLIR based compiler backend, which automatically rewrites their compute heavy operations for better performance.


## Ownership and team asks

| Task                 | Owner(s) or team(s)                              | Notes                                                                                                                                |
| -------------------- | ------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------ |
| Lang-team experiment | ![Team][] [lang][]                               | ![Complete][]                                                                                                                        |
| Lang-team champion   | ![Team][] [lang][]                               | @traviscross                                                                                                                         |
| Standard reviews     | ![Team][] [compiler]                             | Review contributions to `rustc_codegen_llvm` and other parts of the backend                                                          |
| LLVM reviews         | LLVM offload/GPU contributors                    | Individual contributors at AMD/NVIDIA/LLNL agreed to review my code from the LLVM or GPU side                                        |
| Do the work          | @ZuseZ4                                          |                                                                                                                                      |
## Frequently asked questions

