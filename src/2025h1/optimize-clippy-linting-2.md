# Optimizing Clippy & linting
(a.k.a The Clippy Performance Project)

| Metadata         |                                    |
|------------------|------------------------------------|
| Point of contact | @blyxyas                           |
| Teams            | <!-- TEAMS WITH ASKS -->           |
| Task owners      | <!-- TASK OWNERS -->               |
| Status           | Proposed                           |
| Tracking issue   | [rust-lang/rust-project-goals#114] |
| Zulip channel    | N/A                                |
| Continuing goal  | [2024h2 project goal]              |

[2024h2 project goal]: https://rust-lang.github.io/rust-project-goals/2024h2/optimize-clippy.html

## Summary

This is the formalization and documentation of the Clippy Performance Project, a project first talked about on [Zulip, July 2023](https://rust-lang.zulipchat.com/#narrow/stream/257328-clippy/topic/Clippy's.20performance). As the project consists of several points and is ever-changing, this document also has a dynamic structure and the team can add points. 

In short, this is an effort to optimize Clippy, and Rust's linting infrastructure with a point of view of making Clippy faster (both on CI/CD pipelines, and on devs' machines)

## Motivation

Clippy can take up to 2.5 times the time that a normal `cargo check` takes, and it doesn't need to be! Taking so long is expensive both in development time, and in real money.

### The status quo

Based on some [informal][poll-urlo] [feedback][poll-reddit] [polls][poll-mastodon], it's clear that Clippy is used in lots of different contexts. Both in developer's IDEs and outside them.

The usage for IDEs is not as smooth as one may desire or expect when comparing to prior art like [Prettier][prettier], [Ruff][ruff], or other tools in the Rust ecosystem `rustfmt` and Rust-analyzer.

The other big use-case is as a test before committing or on CI. Optimizing Clippy for performance would fold the cost of these tests.

On GitHub Actions, this excessive time can equal the cost of running `cargo check` on a Linux x64 32-cores machine, instead of a Linux x64 2-cores machine. A 3.3x cost increase.


<!-- *Elaborate in more detail about the problem you are trying to solve. This section is making the case for why this particular problem is worth prioritizing with project bandwidth. A strong status quo section will (a) identify the target audience and (b) give specifics about the problems they are facing today. Sometimes it may be useful to start sketching out how you think those problems will be addressed by your change, as well, though it's not necessary.* -->

### The next 6 months

In order to achieve a better performance we want to:

- Have benchmarking software ready to run on the server.
- Optimize the collection of Minimum Safe Rust Version (MSRVs)
- Migrate applicable lints to use incremental compilation

Apart from these 3 clear goals, any open issue, open PR or merged PRs with the label [`performance-project`](https://github.com/rust-lang/rust-clippy/issues?q=sort%3Aupdated-desc+is%3Aopen+label%3Aperformance-project) are a great benefit.

### The "shiny future" we are working towards

The possible outcome would be a system that can be run on-save without being a hassle to the developer, and that has the minimum possible overhead over `cargo check` (which, would also be optimized as a side of a lot of a subset of the optimizations).

A developer shouldn't have to get a high-end machine to run a compiler swiftly; and a server should not spend more valuable seconds on linting than strictly necessary.

[da]: ../about/design_axioms.md

## Ownership and team asks

**Owner:** @blyxyas

| Task              | Owner(s) or team(s)  | Notes |
|-------------------|----------------------|-------|
| Implementation    | @blyxyas, @Alexendoo |       |
| Standard reviews  | ![Team][] [clippy]   |       |


[pr125116]: https://github.com/rust-lang/rust/pull/125116
[poll-urlo]: https://users.rust-lang.org/t/feedback-poll-where-and-how-do-you-use-clippy/114047?u=blyxyas
[poll-reddit]: https://www.reddit.com/r/rust/comments/1dxu43p/feedback_poll_where_how_do_you_use_clippy/
[poll-mastodon]: https://tech.lgbt/@blyxyas/112747808297589676
[prettier]: https://github.com/prettier/prettier
[ruff]: https://github.com/astral-sh/ruff
