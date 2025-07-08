# Promoting Parallel Front End

| Metadata         |                                                   |
| :--------------- | ------------------------------------------------- |
| Point of contact | @SparrowLii                                       |
| Teams            | <!-- TEAMS WITH ASKS -->                          |
| Task owners      | <!-- TASK OWNERS -->                              |
| Status           | Proposed                                          |
| Tracking issue   | [https://github.com/rust-lang/rust/issues/113349] |
| Zulip channel    | [#t-compiler/wg-parallel-rustc][channel]          |

[channel]: https://rust-lang.zulipchat.com/#narrow/channel/187679-t-compiler.2Fwg-parallel-rustc/


## Summary

Continue to parallelize front-end stabilization and performance improvements, continuing from the [2025h1 goal](https://rust-lang.github.io/rust-project-goals/2025h1/parallel-front-end.html).

## Motivation

Verify and resolve the few remaining deadlock and other issues, stabilize the features, and try to further improve the parallel compilation performance through several means.

### The status quo

Parallel front end progressed well in the first half of the year. We resolved the deadlock issue caused by the deadlock handler, added the rustc-rayon  that the parallel front end depends on to the rustc working tree, and optimized the work-stealing scheduling algorithm to avoid deadlocks in the parallelization of rustc queries.

Next we need to resolve and verify the remaining issue list, improve the parallel front end test suit, and include all known issues in the ui tests to ensure the stability of the features.

Then we will start the feature stabilization process to push parallel front end into the stable version.

In addition, there are already contributors working on improving the performance of the parallel front end, such as trying to parallelize the macro expansion process, analysing data contention in query calls, etc. (thank you very much). We will conduct more detailed analysis and implementation of these directions in the second half of the year.

In addition, we need to document and enhance the support of surrounding tools for parallel frontends, such as bootstrap, rustc-perf, and Cargo.

### The next 6 months

- Resolve the remaining issues in the issue [list](https://github.com/rust-lang/rust/issues?q=state%3Aopen%20label%3AA-parallel-compiler). Most of them have been solved, but are lack of ways to ensure stability.
- Land the parallel front end test [suite](https://github.com/rust-lang/rust/pull/132051) to ensure the robustness and prevent various issues from occurring again.
- Enable parallel front end in bootstrap.
- Continue to improve parallel compilation performance, via parallelize the macro expansion and reduce data competition.
- Enable parallel front end in Cargo.
- Add more test cases for parallel front end in rustc-perf

### The "shiny future" we are working towards

We will use measures to ensure the robustness of the parallel front end and push it into the stable version.

The current parallelization front end can already reduce the overall compilation time by 20~30+ percent, but we will continue to optimize it so that this number continues to grow.

## Design axioms

The parallel front end should be:
- safe: Ensure the safe and correct execution of the compilation process
- consistent: The compilation result should be consistent with that in single thread
- maintainable: The implementation should be easy to maintain and extend, and not cause confusion to developers who are not familiar with it.

[da]: ../about/design_axioms.md

## Ownership and team asks

**Owner:** @SparrowLii and Parallel Rustc WG own this goal

| Task                         | Owner(s) or team(s)  | Notes |
| ---------------------------- | -------------------- | ----- |
| Implementation               | @SparrowLii          |       |
| Author tests                 | @SparrowLii          |       |
| Discussion and moral support | ![Team][] [compiler] |       |

## Frequently asked questions


[ICE]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3AWG-compiler-parallel+ice
[deadlock]: https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3AWG-compiler-parallel+deadlock
[test]: https://github.com/rust-lang/rust/issues/118698
[issues]: https://github.com/rust-lang/rust/labels/WG-compiler-parallel
