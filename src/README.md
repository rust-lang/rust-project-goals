# Project goals

This repo tracks the effort to set and track goals for the Rust project.

## Current goal period (2025H1)

The 2025H1 goal period runs from Jan 1 to Jun 30. We have identified three flagship goals for 2025H1:

* Continue making Rust easier to use for network systems by [**bringing the Async Rust experience closer to parity with sync Rust**](./async.md). In 2025H1 we plan to:
    * tell a complete story for the use of async fn in traits, unblocking wide ecosystem adoption;
    * improve the ergonomics of `Pin`, which is frequently used in low-level async code; and
    * prepare to support asynchronous (and synchronous) generators in the language.
* Continue helping Rust support low-level projects by [**stabilizing compiler options and tooling used by the Rust-for-Linux project**](./rfl.md). In 2025H1 we plan to:
    * implement [RFC #3716] to allow stabilizing ABI-modifying compiler flags to control code generation, sanitizer integration, and so forth;
    * taking the first step towards stabilizing [`build-std`](https://rust-lang.github.io/rust-project-goals/2025h1/https://doc.rust-lang.org/cargo/reference/unstable.html#build-std) by [creating a stable way to rebuild core with specific compiler options](./build-std.html);
    * add rustdoc features to extract and customize rustdoc tests (`--extract-doctests`);
    * stabilize clippy configuration like `.clippy.toml` and `CLIPPY_CONF_DIR`;
    * stabilize compiler flags to extract dependency info (e.g., as via `-Zbinary-dep-depinfo=y`) and to configure no-std without requiring it in the source file (e.g., as via `-Zcrate-attr`);
* Address the biggest concerns raised by Rust maintainers, lack of face-to-face interaction, by [**organizing the Rust All-Hands 2025**](./all-hands.md). In 2025H1 we plan to:
    * convene Rust maintainers to celebrate Rust's tenth birthday at [RustWeek 2025](https://2025.rustweek.org) (co-organized with [RustNL](https://2025.rustweek.org/about/));
    * author a first draft for a [Rust vision doc](./rust-vision-doc.md) and gather feedback.

[The full list of 2025H1 goals is available here.](./2025h1/goals.md) We author monthly blog posts about our overall status, but you can also follow the tracking issue for a [particular goal](./2025h1/goals.md) to get updates specific to that goal.


## Next goal period (2025H2)

The next goal period will be 2025H2, running from July 1 to December 30. We are currently in the process of assembling goals. [Click here](./2025h2/goals.md) to see the current list. If you'd like to propose a goal, [instructions can be found here](./how_to/propose_a_goal.md).

## About the process

Want to learn more? Check out some of the following:

* [RFC #3614, which describes the overall goals and plan](https://github.com/rust-lang/rfcs/blob/master/text/3614-project-goals.md)
* The currently [proposed goals for 2024H2](./2024h2/slate.md)
* [How to propose a goal of your own](./how_to/propose_a_goal.md)
* [What it means to be a goal point of contact](./about/owners.md)
