# Survey tools suitability for Std safety verification

| Metadata |           |
| -------- |-----------|
| Owner(s) | @celinval |
| Teams    | [Libs]    |
| Status   | Proposed  |

## Summary

Instrument a [fork of the standard library](https://github.com/model-checking/verify-rust-std) with safety contracts,
and employ existing verification tools to verify the standard library.

## Motivation

The Rust Standard Library is the foundation of portable Rust software. It provides efficient implementations 
and safe abstractions for the most common general purpose programming data structures and operations.
For doing so, they perform unsafe operations internally.

Despite being constantly battle-tested, the implementation of the standard library has not been formally verified
or proven safe. A safety issue in the Standard Library may affect almost all Rust applications, and this effort is
the first step to enhance the safety guarantees of the Standard Library, hence, the Rust ecosystem.

### The status quo

Rust has a very active and diverse formal methods community that has been developing modular and mostly automated
verification tools.

TODO: Finish this

#### Repository Configuration

#### Verification Target

### The next 6 months

First, we will instrument some unsafe functions of the forked Rust Standard Library with function contracts,
and safe abstractions with safety type invariants.

Then we will employ existing verification tools to verify that the annotated unsafe functions are in-fact safe as long
as its contract pre-conditions are preserved. And we will also check that any post condition is respected.
With that, we will work on proving that safe abstractions do not violate any safety contract, and that it does not
leak any unsafe value through its interface.

Type invariants will be employed to verify that unsafe value encapsulation is strong enough to guarantee the safety
of the type interface. Any safe method should be able to assume the type invariant, and it should also preserve the type
invariant. Unsafe methods contract must be enough to guarantee that the type invariant is also preserved at the end
of the call.

Finally, we hope to contribute upstream contracts and type invariants added to this fork using the [experimental contract
support proposed in this MCP](https://github.com/rust-lang/compiler-team/issues/759).

This is open source and very much open to contributions of tools/techniques/solutions.
We introduce problems (currently phrased as challenges) that we believe are important to the Rust and verification
communities. These problems can be solved by anyone.

### The "shiny future" we are working towards

We are working towards the enhancement of Rust verification tools, so it can eventually be incorporated as part of
regular Rust development cycle for code that require the usage of unsafe Rust.

The Rust Standard Library is the perfect candidate given its blast radios and its extensive usage of unsafe Rust
to provide performant abstractions.

## Design axioms

- **No runtime penalty**: Instrumentation must not affect the standard library runtime behavior, including performance.
- **Automated Verification**: Our goal is to verify the standard library implementation. Given how quickly the standard 
library code evolves, automated verification is needed to ensure new changes preserve the properties previously verified.
- **Contract as code**: Keeping the contract language and specification as close as possible to Rust syntax and 
semantics will lower the barrier for users to understand and be able to write their own contracts.

## Ownership and team asks

**Owner:** @celinval

| Subgoal                            | Owner(s) or team(s)  | Notes                                                                 |
|------------------------------------|----------------------|-----------------------------------------------------------------------|
| Discussion and moral support       | ![Team][] [libs][]   |                                                                       |
| Standard review                    | ![Team][] [libs][]   | We would like to contribute upstream the contracts added to the fork. |
| Problem proposals                  | Help Wanted          |                                                                       |
| Fork maintenance                   | @celinval, @jaisnan  |                                                                       |
| Fork PR Reviews                    | [Own Committee]      | We are gathering a few contributors with expertise knowledge.         |
| Instrumentation and verification   | Help Wanted          |                                                                       |

[Own Committee]: https://github.com/model-checking/verify-rust-std/blob/main/.github/pull_requests.toml#L4

### Definitions

Definitions for terms used above:

* *Discussion and moral support* is the lowest level offering, basically committing the team to nothing but good vibes and general support for this endeavor.
* *Standard reviews* refers to reviews for PRs against the Rust repository; these PRs are not expected to be unduly large or complicated.
* *Problem proposals* refers to creating a scoped task that involves verifying a chunk of the standard library. 
* *Fork PR reviews* means a group of individuals who will review the changes made to the fork, as they're expected to require significant context.
Besides contracts, these changes may include extra harnesses, lemmas, ghost-code.
* *Fork maintenance* means configuring CI, performing periodic fork update from upstream, tool integration.
* *Instrumentation and verification* is the work of specifying contracts, invariants, and verifying a specific part of
the standard library.

## Frequently asked questions

