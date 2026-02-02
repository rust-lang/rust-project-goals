# Finish the libtest json output experiment

| Metadata         |                                                                                  |
| :--------------- | -------------------------------------------------------------------------------- |
| Point of contact | @epage                                   |
| Status           | Proposed                                                                         |
| Flagship         | Building blocks                                                                  |
| Tracking issue     | [rust-lang/rust-project-goals#255] |
| Zulip channel    | N/A (an existing stream can be re-used or new streams can be created on request) |

## Summary

Finish the [libtest json experiment](https://rust-lang.github.io/rfcs/3558-libtest-json.html).

## Motivation

[libtest](https://github.com/rust-lang/rust/tree/master/library/test)
is the test harness used by default for tests in cargo projects.
It provides the CLI that cargo calls into and enumerates and runs the tests discovered in that binary.
It ships with rustup and has the same compatibility guarantees as the standard library.

Before 1.70, anyone could pass `--format json` despite it being unstable.
When this was fixed to require nightly,
this helped show [how much people have come to rely on programmatic output](https://www.reddit.com/r/rust/comments/13xqhbm/announcing_rust_1700/jmji422/).

Cargo could also benefit from programmatic test output to improve user interactions, including
- [Wanting to run test binaries in parallel](https://github.com/rust-lang/cargo/issues/5609), like `cargo nextest`
- [Lack of summary across all binaries](https://github.com/rust-lang/cargo/issues/4324)
- [Noisy test output](https://github.com/rust-lang/cargo/issues/2832) (see also [#5089](https://github.com/rust-lang/cargo/issues/5089))
- [Confusing command-line interactions](https://github.com/rust-lang/cargo/issues/1983) (see also [#8903](https://github.com/rust-lang/cargo/issues/8903), [#10392](https://github.com/rust-lang/cargo/issues/10392))
- [Poor messaging when a filter doesn't match](https://github.com/rust-lang/cargo/issues/6151)
- [Smarter test execution order](https://github.com/rust-lang/cargo/issues/6266) (see also [#8685](https://github.com/rust-lang/cargo/issues/8685), [#10673](https://github.com/rust-lang/cargo/issues/10673))
- [JUnit output is incorrect when running multiple test binaries](https://github.com/rust-lang/rust/issues/85563)
- [Lack of failure when test binaries exit unexpectedly](https://github.com/rust-lang/rust/issues/87323)

Most of that involves shifting responsibilities from the test harness to the test runner which has the side effects of:
- Allowing more powerful experiments with custom test runners (e.g. [`cargo nextest`](https://crates.io/crates/cargo-nextest)) as they'll have more information to operate on
- Lowering the barrier for custom test harnesses (like [`libtest-mimic`](https://crates.io/crates/libtest-mimic)) as UI responsibilities are shifted to the test runner (`cargo test`)

### The status quo

### The next 6 months

| Task        | Owner(s) | Notes |
| ----------- | -------- | ----- |
| Experiment with potential test harness features | *owner*  |       |
| Experiment with test reporting moving to Cargo | *owner*  |       |
| Putting forward a proposal for approval | *owner*  |       |

### The "shiny future" we are working towards

- Reporting shifts from test harnesses to Cargo
- We run test harnesses in parallel

## Team asks

| Team       | Support level | Notes                                   |
| ---------- | ------------- | --------------------------------------- |
| [cargo]    | Vibes         |                                         |
| [libs]     | Vibes         |                                         |
| [testing-devex] | Small    | Design discussions and review           |

## Frequently asked questions
