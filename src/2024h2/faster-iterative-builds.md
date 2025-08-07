# Faster build experience

| Metadata         |                             |
|------------------|-----------------------------|
| Point of contact | @jkelleyrtp                 |
| Status           | Not accepted                |
| Zulip channel  | N/A                                |

## Summary

Improvements to make iterative builds faster.

## Motivation

For 2024H2, we propose to create better caching to speed up build times, particularly in iterative, local development. Build time affects all Rust users, but it can be a particular blocker for Rust users in "higher-level" domains like app/game/web development, data science, and scientific computing. These developers are often coming from interpreted languages like Python and are accustomed to making small, quick changes and seeing immediate feedback. In those domains, Rust has picked up momentum as a language for building underlying frameworks and libraries thanks to its lower-level nature. The motivation of this project goal is to make Rust a better choice for higher level programming subfields by improving the build experience (see also the partner goal related to [language papercuts](./ergonomics-initiative.md)). 

### The status quo

Rust has recently seen tremendous adoption in a number of high-profile projects. These include but are not limited to: Firecracker, Pingora, Zed, Datafusion, Candle, Gecko, Turbopack, React Compiler, Deno, Tauri, InfluxDB, SWC, Ruff, Polars, SurrealDB, NPM and more. These projects tend to power a particular field of development: SWC, React, and Turbopack powering web development, Candle powering AI/ML, Ruff powering Python, InfluxDB and Datafusion powering data science etc. Projects in this space devote significant time to improving build times and the iterative experience, often requiring significant low-level hackery. See e.g. [Bevy's page on fast compiles](https://bevyengine.org/learn/quick-start/getting-started/setup/#enable-fast-compiles-optional). Seeing the importance of compilation time, other languages like Zig and Go have made fast compilation times a top priority, and developers often cite build time as one of their favorite things about the experience of using those languages. We don't expect Rust to match the compilation experience of Go or Zig -- at least not yet! -- but some targeted improvements could make a big difference.

### The next six months

The key areas we've identified as avenues to speed up iterative development include:

- Speeding up or caching proc macro expansion
- A per-user cache for compiled artifacts
- A remote cache for compiled artifacts integrated into Cargo itself

There are other longer term projects that would be interesting to pursue but don't necessarily fit in the 2024 goals:

- Partial compilation of invalid Rust programs that might not pass "cargo check"
- Hotreloading for Rust programs
- A JIT backend for Rust programs
- An incremental linker to speed up test/example/benchmark compilation for workspaces

#### Procedural macro expansion caching or speedup

Today, the Rust compiler does not necessarily cache the tokens from procedural macro expansion. On every `cargo check`, and `cargo build`, Rust will run procedural macros to expand code for the compiler. The vast majority of procedural macros in Rust are idempotent: their output tokens are simply a deterministic function of their input tokens. If we assumed a procedural macro was free of side-effects, then we would only need to re-run procedural macros when the input tokens change. This has been shown in prototypes to drastically improve incremental compile times (30% speedup), especially for codebases that employ lots of derives (Debug, Clone, PartialEq, Hash, serde::Serialize).

A solution here could either be manual or automatic: macro authors could opt-in to caching or the compiler could automatically cache macros it knows are side-effect free.
#### Faster fresh builds and higher default optimization levels

A "higher level Rust" would be a Rust where a programmer would be able to start a new project, add several large dependencies, and get to work quickly without waiting minutes for a fresh compile. A web developer would be able to jump into a Tokio/Axum heavy project, a game developer into a Bevy/WGPU heavy project, or a data scientist into a Polars project and start working without incurring a 2-5 minute penalty. In today's world, an incoming developer interested in using Rust for their next project immediately runs into a compile wall. In reality, Rust's incremental compile times are rather good, but Rust's perception is invariably shaped by the "new to Rust" experience which is almost always a long upfront compile time.

Cargo's current compilation model involves downloading and compiling dependencies on a per-project basis. Workspaces allow you to share a set of dependency compilation artifacts across several related crates at once, deduplicating compilation time and reducing disk space usage.

A "higher level Rust" might employ some form of caching - either per-user, per-machine, per-organization, per-library, or otherwise - such that fresh builds are just as fast as incremental builds. If the caching was sufficiently capable, it could even cache dependency artifacts at higher optimization levels. This is particularly important for game development, data science, and procedural macros where debug builds of dependencies run *significantly* slower than their release variant. Projects like Bevy and WGPU explicitly guide developers to manually increase the optimization level of dependencies since the default is unusably slow for game and graphics development.

Generally, a "high level Rust" would be fast-to-compile and maximally performant by default. The tweaks here do not require language changes and are generally a question of engineering effort rather than design consensus.
### The "shiny future" we are working towards

A "high level Rust" would be a Rust that has a strong focus on iteration speed. Developers would benefit from Rust's performance, safety, and reliability guarantees without the current status quo of long compile times, verbose code, and program architecture limitations.

A "high level" Rust would:
- Compile quickly, even for fresh builds
- Be terse in the common case
- Produce performant programs even in debug mode
- Provide language shortcuts to get to running code faster

In our "shiny future," an aspiring genomics researcher would:
- be able to quickly jump into a new project
- add powerful dependencies with little compile-time cost
- use various procedural macros with little compile-time cost
- cleanly migrate their existing program architecture to Rust with few lifetime issues
- employ various shortcuts like unwrap to get to running code quicker
## Design axioms[da]

- Preference for minimally invasive changes that have the greatest potential benefit
- No or less syntax is preferable to more syntax for the same goal
- Prototype code should receive similar affordances as production code
- Attention to the end-to-end experience of a Rust developer
- Willingness to make appropriate tradeoffs in favor of implementation speed and intuitiveness

[da]: ../about/design_axioms.md

## Ownership and team asks

The work here is proposed by Jonathan Kelley on behalf of Dioxus Labs. We have funding for 1-2 engineers depending on the scope of work. Dioxus Labs is willing to take ownership and commit funding to solve these problems.
| Task                         | Owner(s) or team(s) | Notes |
|------------------------------|---------------------|-------|
| proc macro expansion caching | [jkelleyrtp] + tbd  |       |
| global dependency cache      | [jkelleyrtp] + tbd  |       |

* The ![Team][] badge indicates a requirement where Team support is needed.

[Not funded]: https://img.shields.io/badge/Not%20yet%20funded-red
[Approved]: https://img.shields.io/badge/Approved-green
[Not approved]: https://img.shields.io/badge/Not%20yet%20approved-red
[Complete]: https://img.shields.io/badge/Complete-green
[TBD]: https://img.shields.io/badge/TBD-red
[Team]: https://img.shields.io/badge/Team%20ask-red

### Support needed from the project

- We are happy to author RFCs and/or work with other experienced RFC authors.
- We are happy to host design meetings, facilitate work streams, logistics, and any other administration required to execute. Some subgoals proposed might be contentious or take longer than this goals period, and we're committed to timelines beyond six months.
- We are happy to author code or fund the work for an experienced Rustlang contributor to do the implementation. For the language goals, we expect more design required than actual implementation. For cargo-related goals, we expected more engineering required than design. We are also happy to back any existing efforts as there is ongoing work in cargo itself to add various types of caching.
- We would be excited to write blog posts about this effort. This goals program is a great avenue for us to get more corporate support and see more Rust adoption for higher-level paradigms. Having a blog post talking about this work would be a significant step in changing the perception of Rust for use in high-level codebases.

## Outputs and milestones

### Outputs

*Final outputs that will be produced*

### Milestones

*Milestones you will reach along the way*

## Frequently asked questions

### What do I do with this space?

*This is a good place to elaborate on your reasoning above -- for example, why did you put the design axioms in the order that you did? It's also a good place to put the answers to any questions that come up during discussion. The expectation is that this FAQ section will grow as the goal is discussed and eventually should contain a complete summary of the points raised along the way.*

[jkelleyrtp]: https://github.com/jkelleyrtp
