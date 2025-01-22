# Extend pubgrub to match cargo's dependency resolution

| Metadata         |                          |
|------------------|--------------------------|
| Point of contact | @eh2406                  |
| Teams            | <!-- TEAMS WITH ASKS --> |
| Task owners      | <!-- TASK OWNERS -->     |
| Status           | Proposed                 |
| Zulip channel    | N/A                      |
| Continuing goal  | [2024h2 project goal]    |

[2024h2 project goal]: https://rust-lang.github.io/rust-project-goals/2024h2/pubgrub-in-cargo.html

## Summary

Implement a standalone library based on pubgrub that model's cargo dependency resolution and bring it to a quality of code so that it can be maintained by the cargo team. This lays the groundwork for improved cargo error messages, extensions for hotly requested features (e.g., better MSRV support, CVE-aware resolution, etc), and support for a richer ecosystem of cargo extensions.

## Motivation


Cargo's dependency resolver is brittle and under-tested. Disentangling implementation details, performance optimizations, and user-facing functionality will require a rewrite. Making the resolver a standalone modular library will make it easier to test and maintain.


### The status quo

Big changes are required in cargo's resolver: there is lots of new functionality that will require changes to the resolver and the existing resolver's error messages are terrible. Cargo's dependency resolver solves the NP-Hard problem of taking a list of direct dependencies and an index of all available crates and returning an exact list of versions that should be built. This functionality is exposed in cargo's CLI interface as generating/updating a lock file. Nonetheless, any change to the current resolver in situ is extremely treacherous. Because the problem is NP-Hard it is not easy to tell what code changes break load-bearing performance or correctness guarantees. It is difficult to abstract and separate the existing resolver from the code base, because the current resolver relies on concrete datatypes from other modules in cargo to determine if a set of versions have any of the many ways two crate versions can be incompatible.

### The next six months

Develop a standalone library for doing dependency resolution with all the functionality already supported by cargo's resolver. Prepare for experimental use of this library inside cargo.

### The "shiny future" we are working towards

Eventually we should replace the existing entangled resolver in cargo with one based on separately maintained libraries. These libraries would provide simpler and isolated testing environments to ensure that correctness is maintained. Cargo plugins that want to control or understand what lock file cargo uses can interact with these libraries directly without interacting with the rest of cargo's internals.

## Design axioms

- **Correct**: The new resolver must perform dependency resolution correctly, which generally means matching the behavior of the existing resolver, and switching to it must not break Rust projects.
- **Complete output**: The output from the new resolver should be demonstrably correct. There should be enough information associated with the output to determine that it made the right decision.
- **Modular**: There should be a stack of abstractions, each one of which can be understood, tested, and improved on its own without requiring complete knowledge of the entire stack from the smallest implementation details to the largest overall use cases.
- **Fast**: The resolver can be a slow part of people's workflow. Overall performance must be a high priority and a focus.

## Ownership and team asks

**Owner:** @eh2406 will own and lead the effort.

I (@eh2406) will be working full time on this effort. I am a member of the Cargo Team and a maintainer of pubgrub-rs.

Integrating the new resolver into Cargo and reaching the shiny future will require extensive collaboration and review from the Cargo Team. The next milestones involve independent work polishing various projects for publication. Review support from the cargo team, identifying what about the code needs to be documented and improved will be invaluable. However, there is plenty of work clearly available to do. If team members are not available progress will continue.

| Task                                   | Owner(s) or team(s) | Notes |
|----------------------------------------|---------------------|-------|
| Discussion and moral support           | ![Team] [cargo]     |       |
| Implementation work on pubgrub library | eh2046              |       |

### Outputs

Standalone crates for independent components of cargo's resolver. We have already developed https://github.com/pubgrub-rs/pubgrub for solving the core of dependency resolution and https://github.com/pubgrub-rs/semver-pubgrub for doing mathematical set operations on Semver requirements. The shiny future will involve several more crates, although their exact borders have not yet been determined. Eventually we will also be delivering a `-Z pubgrub` for testing the new resolver in cargo itself.

### Milestones

#### For all crate versions on crates.io the performance is acceptable.

There are some crates where pubgrub takes a long time to do resolution, and more where pubgrub takes longer than cargo's existing resolver. Investigate each of these cases and figure out if performance can be improved either by improvements to the underlying pubgrub algorithm or the way the problem is presented to pubgrub.

#### Make a new release of pubgrub with the features developed for the prototype.

The prototype testing tool has relied on pubgrub as a git dependency. This has allowed rapid feedback on proposed changes. Before cargo can depend on PubGrub these changes need to be polished and documented to a quality appropriate for publication on crates.io.

#### Determine what portion of the prototype can be maintained as a standalone library.

One of the goals of this effort is to have large portions of resolution be maintained as separate packages, allowing for their use and testing without depending on all of cargo. Figure out which parts of the prototype can be separate packages and which parts should be part of cargo.

#### Get cargo team review of code developed in the prototype.

Much of the prototypes code has only ever been understood by me. Before it becomes a critical dependency of cargo or part of cargo, it needs to be polished and documented so that other members the cargo team would be comfortable maintaining it.

## Frequently asked questions

### If the existing resolver defines correct behavior then how does a rewrite help?

Unless we find critical bugs with the existing resolver, the new resolver and cargo's resolver should be 100% compatible. This means that any observable behavior from the existing resolver will need to be matched in the new resolver. The benefits of this work will come not from changes in behavior, but from a more flexible, reusable, testable, and maintainable code base. For example: the base `pubgrub` crate solves a simpler version of the dependency resolution problem. This allows for a more structured internal algorithm which enables complete error messages. It's also general enough not only to be used in cargo but also in other package managers. We already have contributions from the maintainers of [`uv`](https://pypi.org/project/uv/) who **are** using the library in production.