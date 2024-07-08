# Extend pubgrub to match cargo's dependency resolution

| Metadata |        |
| -------- | ------ |
| Owner(s) | eh2406 |
| Teams    | Cargo  |
| Status   | WIP    |

## Summary

Implement a standalone library based on pubgrub that model's cargo dependency resolution and validate its accurate with testing against crates found on crates.io. This lays the groundwork for improved cargo error messages, extensions for hotly requested features (e.g., better MSRV support, CVE-aware resolution, etc), and support for a richer ecosystem of cargo extensions.

## Motivation


Cargo's dependency resolver is brittle and under-tested. Disentangling implementation details, performance optimizations, and user-facing functionality will require a rewrite. Making the resolver a standalone modular library will make it easier to test and maintain.


### The status quo

Big changes are required in cargo's resolver: there is lots of new functionality that will require changes to the resolver and the existing resolver's error messages are terrible. Cargo's dependency resolver solves the NP-Hard problem of taking a list of direct dependencies and an index of all available crates and returning an exact list of versions that should be built. This functionality is exposed in cargo's CLI interface as generating/updating a lock file. Nonetheless, any change to the current resolver in situ is extremely treacherous. Because the problem is NP-Hard it is not easy to tell what code changes break load-bearing performance or correctness guarantees. It is difficult to abstract and separate the existing resolver from the code base, because the current resolver relies on concrete datatypes from other modules in cargo to determine if a set of versions have any of the many ways two crate versions can be incompatible.

### The next six months

Develop a standalone library for doing dependency resolution with all the functionality already supported by cargo's resolver. Extensively test this library to ensure maximum compatibility with existing behavior. Prepare for experimental use of this library inside cargo.

### The "shiny future" we are working towards

Eventually we should replace the existing entangled resolver in cargo with one based on separately maintained libraries. These libraries would provide simpler and isolated testing environments to ensure that correctness is maintained. Cargo plugins that want to control or understand what lock file cargo uses can interact with these libraries directly without interacting with the rest of cargo's internals.

## Design axioms

- **Correct**: The new resolver must perform dependency resolution correctly, which generally means matching the behavior of the existing resolver, and switching to it must not break Rust projects.
- **Complete output**: The output from the new resolver should be demonstrably correct. There should be enough information associated with the output to determine that it made the right decision.
- **Modular**: There should be a stack of abstractions, each one of which can be understood, tested, and improved on its own without requiring complete knowledge of the entire stack from the smallest implementation details to the largest overall use cases.
- **Fast**: The resolver can be a slow part of people's workflow. Overall performance must be a high priority and a focus.

## Ownership and other resources

**Owner:** eh2406 will own and lead the effort.

I (eh2406) will be working full time on this effort. I am a member of the Cargo Team and a maintainer of pubgrub-rs.

### Support needed from the project

Integrating the new resolver into Cargo and reaching the shiny future will require extensive collaboration and review from the Cargo Team. However, the next milestones involve independent work exhaustively searching for differences in behavior between the new and old resolvers and fixing them. So only occasional consultation-level conversations will be needed during this proposal.

## Outputs and milestones

| Subgoal                                | Owner(s) or team(s) | Notes |
| -------------------------------------- | ------------------- | ----- |
| Implementation work on pubgrub library | eh2046              |       |
| Discussion and moral support           | ![Team]             |       |

### Outputs

Standalone crates for independent components of cargo's resolver. We have already developed https://github.com/pubgrub-rs/pubgrub for solving the core of dependency resolution and https://github.com/pubgrub-rs/semver-pubgrub for doing mathematical set operations on Semver requirements. The shiny future will involve several more crates, although their exact borders have not yet been determined. Eventually we will also be delivering a `-Z pubgrub` for testing the new resolver in cargo itself.

### Milestones

#### For all crate versions on crates.io the two resolvers agree about whether there is a solution.

Build a tool that will look at the index from crates.io and for each version of each crate, make a resolution problem out of resolving the dependencies. This tool will save off an independent test case for each time pubgrub and cargo disagree about whether there *is* a solution. This will not check if the resulting lock files are the same or even compatible, just whether they agree that a lock file *is* possible. Even this crude comparison will find many bugs in how the problem is presented to pubgrub. This is known for sure, because this milestone has already been achieved.

#### For all crate versions on crates.io the two resolvers accept the other one's solution.

The tool from previous milestone will be extended to make sure that the lock file generated by pubgrub can be accepted by cargo's resolver and vice versa. How long will this take? What will this find? No way to know. To quote [FractalFir](https://www.reddit.com/r/rust/comments/1doh929/comment/la9oo2i/) "If I knew where / how many bugs there are, I would have fixed them already. So, providing any concrete timeline is difficult." 

#### For all crate versions on crates.io the performance is acceptable.

There are some crates where pubgrub takes a long time to do resolution, and many more where pubgrub takes longer than cargo's existing resolver. Investigate each of these cases and figure out if performance can be improved either by improvements to the underlying pubgrub algorithm or the way the problem is presented to pubgrub.

## Frequently asked questions

### If the existing resolver defines correct behavior then how does a rewrite help?

Unless we find critical bugs with the existing resolver, the new resolver and cargo's resolver should be 100% compatible. This means that any observable behavior from the existing resolver will need to be matched in the new resolver. The benefits of this work will come not from changes in behavior, but from a more flexible, reusable, testable, and maintainable code base. For example: the base `pubgrub` crate solves a simpler version of the dependency resolution problem. This allows for a more structured internal algorithm which enables complete error messages. It's also general enough not only to be used in cargo but also in other package managers. We already have contributions from the maintainers of [`uv`](https://pypi.org/project/uv/) who **are** using the library in production.
