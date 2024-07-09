# Stabilize Parallel Front End

| Metadata |             |
| -------- | ----------- |
| Owner(s) | @SparrowLii |
| Teams    | [Compiler]  |
| Status   | Proposed    |

## Summary

We will move rustc's support for parallel front end closer to stability by resolving [ICE] and [deadlock] issues, completing the [test] suit for multithreaded scenario and integrating parallel front end into bootstrap. This fits into our larger goal of improving rustc build times by 20% by leveraging multiple cores and enhance its robustness.

## Motivation

The parallel front end has been implemented in nightly, but there are still many problems that prevent it from being stable and used at scale.

### The status quo

Many current [issues] reflect ICE or deadlock problems that occur during the use of parallel front end. We need to resolve these issues to enhance its robustness.

The existing compiler testing framework is not sufficient for the parallel front end, and we need to enhance it to ensure the correct functionality of the parallel front end.

The current parallel front end still has room for further improvement in compilation performance, such as parallelization of HIR lowering and macro expansion, and reduction of data contention under more threads (>= 16).

We can use parallel front end in bootstrap to alleviate the problem of slow build of the whole project.

Cargo does not provide an option to enable the use of parallel front end, so it can only be enabled by passing rustc options manually.

### The next 6 months

- Solve [ICE] and [deadlock] issues (unless they are due to lack of rare hardware environment or are almost impossible to reproduce).
- Improve the parallel compilation test framework and enable parallel front end in UI tests.
- Enable parallel frontends in bootstrap.
- Continue to improve parallel compilation performance, with the average speed increase from 20% to 25% under 8 cores and 8 threads.
- Communicate with Cargo team on the solution and plan to support parallel front end.

### The "shiny future" we are working towards

The existing rayon library implementation is difficult to fundamentally eliminate the deadlock problem, so we may need a better scheduling design to eliminate deadlock without affecting performance.

The current compilation process with `GlobalContext` as the core of data storage is not very friendly to parallel front end. Maybe try to reduce the granularity (such as modules) to reduce data competition under more threads and improve performance.

## Design axioms

The parallel front end should be:
- safe: Ensure the safe and correct execution of the compilation process
- consistent: The compilation result should be consistent with that in single thread
- maintainable: The implementation should be easy to maintain and extend, and not cause confusion to developers who are not familiar with it.

[da]: ../about/design_axioms.md

## Ownership and team asks

**Owner:** @SparrowLii and Parallel Rustc WG own this goal

| Subgoal                      | Owner(s) or team(s)  | Notes |
| ---------------------------- | -------------------- | ----- |
| Implementation               | @SparrowLii          |       |
| Author tests                 | @SparrowLii          |       |
| Discussion and moral support | ![Team][] [Compiler] |       |

## Frequently asked questions


[ICE]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3AWG-compiler-parallel+ice
[deadlock]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3AWG-compiler-parallel+deadlock
[test]: https://github.com/rust-lang/rust/issues/118698
[issues]: https://github.com/rust-lang/rust/labels/WG-compiler-parallel