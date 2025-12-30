# Incremental Systems Rethought

| Metadata         |                                                                                  |
| :--------------- | -------------------------------------------------------------------------------- |
| Point of contact | @blyxyas                                                                         |
| Status           | Proposed                                                                         |
| Tracking issue   |                                                                                  |
| Zulip channel    | N/A                                                                              |

## Summary

Design, propose, and implement a redesign of the incremental system to allow for shared common bases between different Rustc invocations with different arguments (i.e. `cargo build` and `cargo check`)

There's already an [on-going RFC], based on [this blog post and talk] by the same author (and this project goal's point of contact)
As my other project goals, this is a formalization of an already existing effort.

## Motivation

The incremental system (i.e. the system that takes care of reusing the dep-graph and keeping track of changes on
compiler invocations) does not take into account inter-activity communication. So, `cargo build` and `cargo check`,
while they share a lot of common meaning and thus should share much in a perfect incremental system, in reality
trigger a recompilation if performed one after the other.

This project goal will design, propose and implement the necessary changes for `check`, `build` and possibly some
3rd-parties such as `clippy` to share a common incremental ground when performed in a compatible way.

So, when a user runs `cargo check` and then `cargo build`, `cargo build` would reuse much of the work done by `cargo check`.
`cargo check` and `cargo clippy` (if implemented) would also reuse a lot of work. `cargo build` and `cargo test` could also
reuse some e.g. parsing information

### The status quo

Quoting from the earlier mentioned RFC:

The current model for incremental recompilations doesn't share progress between compiler activities, leading to unnecessary
rebuilds. Users notice redundant compilations, as "Changes in workspaces trigger unnecessary rebuilds" was submitted as
[a big complaint in the compiler performance][perf-survey].

---

This affects all users that use a typical `cargo check` -> `cargo build` -> `cargo test` workflow. Even on codebases
that don't use e.g. `cfg(test)` or similar profile-based `cfg` attributes, it will rebuild the workspace 3 times.

Summing to the already slow compilation times, having to rebuild the workspace on every different command is a pain
point discovered in the performance survey.

### The next 6 months

> *Sketch out the specific things you are trying to achieve in this goal period. This should be short and high-level -- we don't want to see the design!*

| Task            | Owner(s) | Notes |
| --------------- | -------- | ----- |
| Design, propose | @blyxyas |       |
| Implementation  | @blyxyas |       |
| ...             |          |       |

### The "shiny future" we are working towards

The end goal is to allow for smoother work-reutilization between compiler invocations with little to no overhead. With dynamic dependencies declaring which compiler flags are sensitive for the current codebase recompiling, and keeping an accurate track of macro invocations and a real and specific sense of which functions from dependencies the current crate depends on.

The shiny future, and the end goal of the whole roadmap is to avoid the following scenarios:

- Full recompilations on `cargo check` -> `cargo build` on all scenarios. (Some will need it due to e.g. build scripts, but the majority don't)
- Recompilations at all on `cargo build` -> `cargo check`
- Recompilations due to changes in unused functions from dependencies.
- Recompilations due to CLI flags that don't affect the current invocation (e.g. `-Clto` on `cargo check`)

The first step is completing the RFC and implementing it in an unstable stage.

## Team asks

| Team       | Support level | Notes                                                        |
| ---------- | ------------- | ------------------------------------------------------------ |
| [compiler] | Medium        | I'd need to find a champion, but I don't know where to start |

## Frequently asked questions

[on-going RFC]: https://github.com/rust-lang/rfcs/pull/3881/
[this blog post and talk]: https://blog.goose.love/posts/improving-the-incremental-system-in-the-rust-compiler/
[perf-survey]: https://blog.rust-lang.org/2025/09/10/rust-compiler-performance-survey-2025-results/#incremental-rebuilds
