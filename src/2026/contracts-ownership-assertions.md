# Contracts: primitive ownership assertions

| Metadata         |                                                                                  |
| :--------------- | -------------------------------------------------------------------------------- |
| What and why     | Implement [MCP #942](https://github.com/rust-lang/compiler-team/issues/942): experiment with Fulminate-like ownership primitives in contracts                |
| Point of contact | @dawidl022                                   |
| Status           | Proposed                                                                         |
| Tracking issue   |      |
| Zulip channel    | N/A (an existing stream can be re-used or new streams can be created on request) |

## Summary

This project aims to implement the `owned` and `alloc_block` separation logic
assertions in the Rust compiler, as described in [MCP
#942](https://github.com/rust-lang/compiler-team/issues/942), to enable the
formal specification and verification of unsafe memory manipulation. The effort
is split into two main phases:

- Implementation: Building the core compiler intrinsics and higher-level wrappers, alongside integrating them with Miri for dynamic runtime testing.

- Documentation: Creating extensive reference materials and real-world tutorials to teach developers how to apply these separation logic concepts to verify libraries using unsafe Rust, e.g. the Rust standard library.

## Motivation

### The status quo

Safe Rust code depends on lower-level unsafe Rust libraries. The correctness of
these low-level libraries directly impacts the memory safety guarantees of safe
Rust code. By giving developers and formal methods specialists the language
necessary to express the correctness of unsafe Rust code, we advance a step
closer to closing the gap between the widely praised hypothetical safety of Rust
as a language, to tangible proofs of such being the case.

There is currently an [experiment](https://github.com/rust-lang/rust/issues/128044) inside rustc to add special annotations that describe pre/post-conditions of functions. While built to be extensible, today this can only represent rather simple invariants. Unsafe code typically needs to reason with a more powerful logic, for example about memory ownership.

We want this specification language to be able to specify
[the whole of the standard library](../2025h1/std-contracts.md).
For this purpose, we propose to introduce ownership primitives, as
described in detail in [MCP #942](https://github.com/rust-lang/compiler-team/issues/942).
These primitives are based on the standard and well-understood theory of separation logic,
and modeled after [Fulminate](https://dl.acm.org/doi/10.1145/3704879),
which is a practical, runtime-checkable, and non-expert-legible take on writing
separation logic predicates, which looks to be a great fit for Rust.

### What we propose to do about it

Discussed in depth in [MCP
#942](https://github.com/rust-lang/compiler-team/issues/942).

TL;DR: The state of Rust after the implementation would be that we have
ownership assertions suitable for specifying most raw pointer manipulating code,
and a fully-functional way of dynamically testing those assertions through Miri.

Successfully implementing this change will allow for more functions in the
standard library to be given contract specifications, bringing us further
towards the goal of [specifying and proving correct the entire Rust standard
library](../2024h2/std-verification.md). This addition specifically targets code
using `unsafe` Rust, which is known to have subtle correctness arguments, and
serves as the low level building blocks for many fundamental data structures in
the standard library.

Since many developers may be unfamiliar with Rust contracts and separation
logic, we will also put together documentation and a tutorial as part of this
goal. The benefits of doing this are two-fold: 1. we can directly evaluate the applicability of `owned` and `alloc_block` in real-world code (stdlib) and 2. it walks developers through non-trivial examples, allowing them to better understand the concepts and be able to apply them well to their own code, and be able to flag shortcomings in the current approach. This would be similar in spirit to the Nomicon (https://doc.rust-lang.org/nomicon/) and "Learn Rust With Entirely Too Many Linked Lists" (https://rust-unofficial.github.io/too-many-lists/index.html).

### Work items over the next year

#### Implementation

| Task        | Owner(s) | Notes |
| ----------- | -------- | ----- |
| Low-level ownership assertion intrinsics available for use in contracts. | @dawidl022  |       |
| Higher-level wrapper assertions for typical use cases, including alignment, arrays etc. | @dawidl022 | |
| Prototype integration with Miri - runtime ownership checks should work with the prototype Miri runtime ownership tracker as [implemented by Johannes Hostert](https://github.com/JoJoDeveloping/miri/tree/fulminate).|@dawidl022 | |
| Full Miri runtime ownership support | |

#### Documentation

| Task        | Owner(s) | Notes |
| ----------- | -------- | ----- |
| Reference level documentation: explaining the semantics of the ownership primitives, along with basic examples. | @dawidl022 | |
| A tutorial: selecting one of the challenges in https://model-checking.github.io/verify-rust-std/challenges.html and walking the developer through what it means to write the specifications, and testing them using the Miri runtime. | @dawidl022 | |

## Team asks

| Team       | Support level | Notes                                   |
| ---------- | ------------- | --------------------------------------- |
| [compiler] | Small         | Code reviews                            |
| [miri]     | Medium        | Guidance on integration                 |

## Funding

| Purpose | Cost | Funded | Sponsor(s) |
|---------|------|--------|------------|
| Contributor (12 months) | Ask | No | |

## Frequently asked questions
