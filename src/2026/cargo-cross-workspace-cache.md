# Cargo cross workspace cache

| Metadata         |                                                                                  |
| :--------------- | -------------------------------------------------------------------------------- |
| Point of contact | @ranger-ross                                                                     |
| Status           | Proposed                                                                         |
| Tracking issue   |                                                                                  |
| Zulip channel    | N/A                                                                              |
| [cargo] champion | @epage                                                                           |

## Summary

Work towards a build cache that is shared across workspaces to build times and reduce disk usage.

## Motivation

### The status quo

Currently Cargo stores build artifacts in `build-dir` (which defaults to `target` in root of the workspace).
These artifacts are local to the current workspace. This is not ideal as it requires Cargo to rebuild build units when we could simply reuse the artifacts that have previously been built for other workspaces. Not sharing artifacts across workspaces also results in higher disc usage as files are duplicated.

### What we propose to do about it

In 2025, we split `target-dir` into 2 directories (`artifact-dir` and `build-dir`) as well as began re-organizing the file layout to be grouped by build unit. With this preliminary work complete, we can begin working towards creating a shared cache that shares build units across workspaces.

A shared cache would:
* Skip compilation for commonly used crates
* Reduce disk usage as we only store build artifacts for a given build unit once
* Make it possible to share build artifacts between profiles (e.g. `debug`, `release`)
* Provide a central location to cleanup unneeded build artifacts (potentially automatically by Cargo)
* Could be extended in the future to be pre-populated from a remote cache for CI usecases.

In 2026, we will design and implement this cache in Cargo, making it available on nightly for users to begin experimenting with. (tracked in [cargo#5931](https://github.com/rust-lang/cargo/issues/5931))
As part of implementing the cache we will stabilize the `build-dir` [layout rework](https://github.com/rust-lang/cargo/issues/15010) that was done in 2025.
In the beginning, the cache would be fairly conservative in what is cached but would be expanded over time.
At the end of the year, we should have an understanding of the benefits and tradeoffs of the design we pick as well as a rough path towards stablization.

### Work items over the next year


| Task        | Owner(s) | Notes |
| ----------- | -------- | ----- |
| Design the cache | @ranger-ross  | In collaboration with T-cargo |
| Implement in Cargo | @ranger-ross  |  |

## Team asks

| Team       | Support level | Notes                                   |
| ---------- | ------------- | --------------------------------------- |
| [cargo]    | Medium         | Design and code reviews                 |

## Frequently asked questions

### What about shared CARGO_TARGET_DIR?

Having a global `CARGO_TARGET_DIR` is a common pattern, but comes with some downsides like `cargo clean` removing all artifacts for all workspaces as well not having the output binaries in the workspace directory.
Users also need to be aware of this pattern and configure it themselves.
It would be great if we could get the benefits of a shared `CARGO_TARGET_DIR` out of the box no configuration.


### What about tools like sccache?

Tools like sccache try to infer inputs for hashing a cache key from command-line arguments.
In Cargo, we have much more knowledge about the dependency graph and crate metadata, which could allow us to be more aggressive in what we choose to cache.
