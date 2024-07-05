# Sandboxed build scripts

| Metadata |             |
| ---------| ----------- |
| Owner(s) | [weihanglo] |
| Teams    | [Cargo]     |
| Status   | WIP         |

[weihanglo]: https://github.com/weihanglo
[Cargo]: https://www.rust-lang.org/governance/teams/dev-tools#team-cargo

## Summary

Explore different strategies for sandboxing build script executions in Cargo.

## Motivation

Cargo users can opt-in to run [build scripts] in a sandboxed environment,
limiting their access to OS resources like the file system and network.
By providing a sandboxed environment for build script executions,
fewer repetitive code scrutinies are needed.
The execution of a build script also becomes more deterministic,
helping the caching story for the ecosystem in the long run.

[build scripts]: https://doc.rust-lang.org/cargo/reference/build-scripts.html

### The status quo

Build scripts in Cargo can do literally anything from network requests to executing arbitrary binaries.
This isn't deemed a security issue as it is "by design".
Unfortunately, this "by design" virtue relies on trust among developers within the community.
When trust is broken by some incidents,
even just once,
the community has no choice but to intensively review build scripts in their dependencies.

Although there are collaborative code review tools like [cargo-vet] and [cargo-crev] to help build trust,
comprehensive review is still impractical,
especially considering the pace of new version releases.
In Rust, the `unsafe` keyword helps reviewers identify code sections that require extra scrutiny.
However, an unsandboxed build script is effectively an enormous `unsafe` block,
making comprehensive review impractical for the community.

Besides the security and trust issues,
in an unsandboxed build script,
random network or file system access may occur and fail.
These kinds of "side effects" are notoriously non-deterministic,
and usually cause retries and rebuilds in build pipelines.
Because the build is not deterministic,
reproducibility cannot be easily achieved,
making programs harder to trace and debug.

There is one 2024 GSoC project ["Sandboxed and Deterministic Proc Macro using Wasm"][GSoC]
experimenting with the possibility of using WebAssembly to sandbox procedural macros.
While build scripts and proc-macros are different concepts at different levels,
they share the same flaw — arbitrary code execution.
Given that we already have experiments on the proc-macros side,
it's better we start some groundwork on build scripts in parallel,
and discuss the potential common interface for Cargo to configure them.

[cargo-vet]: https://crates.io/crates/cargo-vet
[cargo-crev]: https://crates.io/crates/cargo-crev
[GSoC]: https://summerofcode.withgoogle.com/programs/2024/projects/kXG0mZoj

### The next 6 months

* Look at prior art in this domain, especially for potential blockers and challenges.
* Prototype on sandboxing build scripts.
  Currently looking at [WebAssembly System Interface (WASI)](WASI) and [Cackle].
* Provide a way to opt-in sandboxed build scripts for Cargo packages,
  and design a configurable interface to grant permissions to each crate.
* Based on the results of those experiments,
  consider whether the implementation should be a third-party Cargo plugin first,
  or make it into Cargo as an unstable feature (with a proper RFC).

[wasi]: https://wasi.dev/
[Cackle]: https://github.com/cackle-rs/cackle

### The "shiny future" we are working towards

These could become future goals if this one succeeds:

* The sandboxed build script feature will be opted-in at first when stabilized.
  By the next Edition, sandboxed build scripts will be on by default,
  hardening the supply chain security.
* Cargo users only need to learn one interface for both sandboxed proc-macros and build scripts.
  The configuration for build scripts will also cover the needs for sandboxed proc-macros,
* Crates.io and the [`cargo info`] command display the permission requirements of a crate,
  helping developers choose packages based on different security level needs.
* The runtime of the sandbox environment is swappable,
  enabling the potential support of remote execution without waiting for a first-party solution.
  It also opens a door to hermetic builds.

[`cargo info`]: https://crates.io/crates/cargo-information

## Design axioms

In order of importance, a sandboxed build script feature should provide the following properties:

* **Restrict runtime file system and network access**, unless allowed explicitly.
* **Cross-platform supports.**
  Cargo is guaranteed to work on [tier 1] platforms.
  This is not a must have for experiments,
  but is a requirement for stabilization.
* **Easy to configure for using system (C) libraries.**
  Probing and building from system libraries is the major use case of build scripts.
  We should support it as a first-class citizen.
* **Declarative configuration interface to grant permissions to packages.**
  A declarative configuration helps us analyze permissions granted more easily,
  without running the actual code.
* **Don't block the build when the sandboxed feature is off.**
  The crates.io ecosystem shouldn't rely on the interface to **successfully** build things.
  That would hurt the integration with other external build systems.
  It should work as if it is an extra layer of security scanning.
* **Room for supporting different sandbox runtimes and strategies.**
  This is for easier integration into external build systems,
  as well as faster iteration for experimenting with new ideas.

Currently out of scope:

* Terminal user interface.
* Pre-built build script binaries.
* Hermetic builds, though this extension should be considered.
* Support for all tier 2 with-host-tools platforms.
  As an experiment, we follow what the chosen sandbox runtime provides us.
* On-par build times.
  The build time is expected to be impacted because build script artifacts are going to build for the sandbox runtime.
  This prevents an optimization that when "host" and "target" platforms are the same,
  Cargo tries to share artifacts between build scripts and applications.

[tier 1]: https://doc.rust-lang.org/rustc/platform-support.html#tier-1-with-host-tools

## Ownership and other resources

**Owner:** [weihanglo],
though I also welcome someone else to take ownership of it.
I would be happy to support them as a Cargo maintainer.

### Support needed from the project

| Subgoal                                        | Owner(s) or team(s)  | Status |
| ---------------------------------------------- | -------------------- | ------ |
| Experiments for sandboxed build script         |                      |        |
| ↳ design                                       | ![TBD][]             |        |
| ↳ general design review                        | ![Team][] [Cargo]    |        |
| ↳ security reviews                             | ![Help wanted][]     |        |
| ↳ WASI runtime design review (if chosen)       | ![Help wanted][]     |        |
| ↳ code reviews                                 | ![Team][] [Cargo]    |        |
| ↳ collaboration with [GSoC] proc-macro project | ![Team][] [Compiler] |        |
| ↳ sandbox runtime distribution (if needed)     | ![Team][] [Infra]    |        |
| ↳ Summary of experiments or RFC                | ![TBD][]             |        |
  
[Help wanted]: https://img.shields.io/badge/Help%20wanted-yellow
[Complete]: https://img.shields.io/badge/Complete-green
[TBD]: https://img.shields.io/badge/TBD-red
[Team]: https://img.shields.io/badge/Team%20ask-red

[Compiler]: https://www.rust-lang.org/governance/teams/compiler
[Infra]: https://www.rust-lang.org/governance/teams/infra

## Outputs and milestones

### Outputs

As the work here is mostly experiments and prototyping,
based on the results,
the outputs could be:

* A report about why these methods have failed to provide a proper sandboxed environment for build scripts in Cargo,
  plus some other areas worth exploring in the future.
* A configurable sandboxed environment for build scripts landed as an unstable feature in Cargo,
  or provided via crates.io as a third-party plugin for faster experimenting iteration.
* An RFC proposing a sandboxed build script design to the Rust project.

### Milestones

| Milestone                                               | Expected Date |
| ------------------------------------------------------- | ------------- |
| Summarize the prior art for sandbox strategies          | 2024-07       |
| Prototype a basic sandboxed build script implementation | 2024-08       |
| Draft a configurable interface in Cargo.toml            | 2024-10       |
| Integrate the configurable interface with the prototype | 2024-12       |
| Ask some security experts for reviewing the design      | TBD           |
| Write an RFC summary for the entire prototyping process | TBD           |

## Frequently asked questions

### Q: Why can't build script be removed?

The Rust crates.io ecosystem depends heavily on build scripts.
Some foundational packages use build scripts for essential tasks,
such as linking to C dependencies.
If we shut down this option without providing an alternative,
half of the ecosystem would collapse.

That is to say, build script is a feature included in the stability guarantee that we cannot simply remove,
just like the results of the dependency resolution Cargo produces.

### Q: Why are community tools like `cargo-vet` and `cargo-crev` not good enough?

They are all excellent tools.
A sandboxed build script isn't meant to replace any of them.
However, as aforementioned, those tools still require intensive human reviews,
which are difficult to achieve
(cargo-crev has [103 reviewers and 1910 reviews](https://web.crev.dev/rust-reviews/) at the time of writing).

A sandboxed build script is a supplement to them.
Crate reviews are easier to complete when crates explicitly specify the permissions granted.

### Q: What is the difference between sandboxed builds and deterministic builds?

Sandboxing is a strategy that isolates the running process from other resources on a system,
such as file system access.

A deterministic build always produces the same output given the same input.

Sandboxing is not a requirement for deterministic builds, and vice versa.
However, sandboxing can help deterministic builds because hidden dependencies often come from the system via network or file system access.

### Q: Why do we need to build our own solution versus other existing solutions?

External build systems are aware of this situation.
For example, Bazel provides a [sandboxing feature](https://bazel.build/docs/sandboxing).
Nix also has a [sandbox build](https://nix.dev/manual/nix/2.23/command-ref/conf-file.html#conf-sandbox) via a different approach.
Yet, migrating to existing solutions will be a long-term effort for the entire community.
It requires extensive exploration and discussions from both social and technical aspects.
At this moment, I don't think the Cargo team and the Rust community are ready for a migration.
