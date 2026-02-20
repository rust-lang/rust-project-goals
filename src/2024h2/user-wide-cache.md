# User-wide build cache

| Metadata         |                                    |
|------------------|------------------------------------|
| Point of contact | @epage                             |
| Status           | Accepted                           |
| Needs            | Contributor                        |
| Tracking issue   | [rust-lang/rust-project-goals#124] |
| Zulip channel    | N/A                                |

## Summary

Extend Cargo's caching of intermediate artifacts across a workspace to caching them across all workspaces of the user.

## Motivation

The primary goal of this effort is to improve build times by reusing builds across projects.

Secondary goals are
- Reduce disk usage
- More precise cross-job caching in CI

### The status quo

When Cargo performs a build, it will build the package you requested and all
dependencies individually, linking them in the end.
These build results (intermediate build artifacts) and the linked result (final
build artifact) are stored in the target-dir, which is per-workspace by
default.

Ways cargo will try to reuse builds today:
- On a subsequent build, Cargo tries to reuse these build results by
  "fingerprinting" the inputs to the prior build and checking if that
  fingerprint has changed.
- When dependencies are shared by host (`build.rs`, proc-macros) and
  platform-target and the platform-target is the host, Cargo will attempt to
  share host/target builds

Some users try to get extra cache reuse by assigning all workspaces to use the same target-dir.
- Cross-project conflicts occur because this shares both intermediate (generally unique) and final build artifacts (might not be unique)
- `cargo clean` will clear the entire cache for every project
- Rebuild churn from build inputs, like `RUSTFLAGS`, that cause a rebuild but aren't hashed into the file path

In CI, users generally have to declare what directory is should be cached between jobs.
This directory will be compressed and uploaded at the end of the job.
If the next job's cache key matches, the tarball will be downloaded and decompressed.
If too much is cached, the time for managing the cache can dwarf the benefits of the cache.
Some third-party projects exist to help manage cache size.

### The next 6 months

Add support for user-wide intermediate artifact caching
- Re-work target directory so each intermediate artifact is in a self-contained directory
  - Develop and implement transition path for tooling that accesses intermediate artifacts
- Adjust `cargo build` to
  - Hash all build inputs into a user-wide hash key
  - If hash key is present, use the artifacts straight from the cache, otherwise build it and put it in the cache
  - Limit this immutable packages ("non-local" in cargo terms, like Registry, git dependencies)
  - Limit this to idempotent packages (can't depend on proc-macro, can't have a `build.rs`)
  - Evaluate risks and determine how we will stabilize this (e.g. unstable to stable, opt-in to opt-out to only on)
- Track intermediate build artifacts for garbage collection
- Explore
  - Idempotence opt-ins for `build.rs` or proc-macros until sandboxing solutions can determine the level of idempotence.
  - a CLI interface for removing anything in the cache that isn't from this CI job's build, providing more automatic CI cache management without third-party tools.

Compared to pre-built binaries, this is adaptive to what people use
- feature flags
- RUSTFLAGS
- dependency versions

A risk is that this won't help as many people as they hope because being able
to reuse caches between projects will depend on the exact dependency tree for
every intermediate artifact.
For example, when building a proc-macro
- `unicode-ident` has few releases, so its likely this will get heavy reuse
- `proc-macro2` is has a lot of releases *and* depends on `unicode-ident`
- `quote` has a lot of releases *and* depends on `proc-macro2` and `unicode-ident`
- `syn` has a lot of releases *and* depends on `proc-macro2`, `unicode-ident`, and optionally on `quote`

With `syn` being a very heavy dependency, if it or any of its dependency versions are mismatched between projects,
the user won't get shared builds of `syn`.

See also [cargo#5931](https://github.com/rust-lang/cargo/issues/5931).

### The "shiny future" we are working towards

The cache lookup will be extended with plugins to read and/or write to different sources.
Open source projects and companies can have their CI read from and write to their cache.
Individuals who trust the CI can then configure their plugin to read from the CI cache.

A cooperating CI service could provide their own plugin that,
instead of caching everything used in the last job and unpacking it in the next,
their plugin could download only the entries that will be needed for the current build
(e.g. say a dependency changed)
and only upload the cache entries that were freshly built.
Fine-grained caching like this would save the CI service on bandwidth, storage,
and the compute time from copying, decompressing, and compressing the cache.
Users would have faster CI time and save money on their CI service, minus any
induced demand that faster builds creates.

On a different note, as sandboxing efforts improve, we'll have precise details
on the inputs for `build.rs` and proc-macros and can gauge when there is
idempotence (and verify the opt-in mentioned earlier).

## Design axioms

*This section is optional, but including [design axioms][da] can help you signal how you intend to balance constraints and tradeoffs (e.g., "prefer ease of use over performance" or vice versa). Teams should review the axioms and make sure they agree. [Read more about design axioms][da].*

[da]: ../about/design_axioms.md

## Ownership and team asks

**Owner:** *Identify a specific person or small group of people if possible, else the group that will provide the owner. GitHub user names are commonly used to remove ambiguity.*

*This section defines the specific work items that are planned and who is expected to do them. It should also include what will be needed from Rust teams. The table below shows some common sets of asks and work, but feel free to adjust it as needed. Every row in the table should either correspond to something done by a contributor or something asked of a team. For items done by a contributor, list the contributor, or ![Heap wanted][] if you don't yet know who will do it. For things asked of teams, list ![Team][] and the name of the team. The things typically asked of teams are defined in the [Definitions](#definitions) section below.*

| Task                   | Owner(s) or team(s) | Notes |
|------------------------|---------------------|-------|
| Implementation         | Goal owner          |       |
| Standard reviews       | ![Team][] [cargo]   |       |
| Mentoring and guidance | @epage              |       |
| Design meeting         | ![Team][] [cargo]   |       |

## Frequently asked questions

### Why not pre-built packages?

Pre-built packages requires guessing
- CPU Architecture
- Feature flags
- RUSTFLAGS
- Dependency versions

If there are any mismatches there, then the pre-built package can't be used.

A build cache can be populated with pre-built packages and react to the unique circumstances of the user.

### Why not sccache?

Tools like sccache try to infer inputs for hashing a cache key from command-line arguments.
This has us reusing the extra knowledge Cargo has to get more accurate cache key generation.

### If this is limited to immutable, idempotent packages, is this worth it?

In short, yes.

First, this includes an effort to allow packages to declare themselves as idempotent.
Longer term, we'll have sandboxing to help infer / verify idempotence.

### If subtle dependency changes prevent reuse across projects, is this worth it?

In short, yes.

This is a milestone on the way to remote caches.
Remote caches allows access to CI build caches for the same project you are developing on,
allowing full reuse at the cost of network access.