# Instrument the Rust standard library with safety contracts

| Metadata         |                                    |
|:-----------------|------------------------------------|
| Point of contact | @celinval                          |
| Status           | Accepted                           |
| Tracking issue   | [rust-lang/rust-project-goals#126] |
| Zulip channel    | N/A                                |
## Summary

Finish the implementation of the contract attributes proposed in the compiler [MCP-759],
and port safety contracts from the [verify-rust-std] fork to the Rust standard library.

## Motivation

Safety contracts serve as formal specifications that define the preconditions, postconditions, and invariants,
that must be maintained for functions and data structures to operate correctly and safely.
Currently, the Rust standard library already contains safety pre- and postconditions specified in the unsafe functions'
documentation.

Contract attributes will enable developers to define safety requirements and behavioral specifications through programmatic contracts,
which can be automatically converted into runtime checks when needed.
These contracts can also express conditions that are verifiable through static analysis tools,
and also provide foundation for formal verification of the standard library implementation, and other Rust code.

### The status quo

Safety conditions are already well documented, and the Rust standard library is also instrumented using
`check_library_ub` and `check_language_ub` in many different places for conditions that are checkable at runtime.

The compiler team has also accepted @pnkfelix's proposal [MCP-759] to add experimental contracts attributes, and
the initial implementation is currently [under review](https://github.com/rust-lang/rust/pull/128045).

Finally, we have annotated and verified around 200 functions in the [verify-rust-std] fork with safety contracts using
contract attributes similar to the ones proposed in [MCP-759].

### The next 6 months

First, we will keep working with the compiler team to finish the implementation of contract attributes.
We'll add support to `#[contracts::requires]` and `#[contracts::ensures]` attributes as described in [MCP-759],
as well type invariant specification.

This will allow users to convert contracts into runtime checks, as well as, provide compiler interface
for external tools, such as verification tools, to retrieve the annotated contracts.

Once that has been merged to the compiler, we will work with the library to annotate functions of the standard library
with their safety contracts.

### The "shiny future" we are working towards

All unsafe functions in Rust should have their safety conditions specified using contracts, and verified that those
conditions are enough to guarantee absence of undefined behavior.

Rust users should be able to check that their code do not violate the safety contracts of unsafe functions, which
would rule out the possibility that their applications could have a safety bug.

## Design axioms

- **No runtime penalty**: Instrumentation must not affect the standard library runtime behavior, including performance, 
unless users opt-in for contract runtime checks.
- **Formal Verification**: Enable the verification of the standard library implementation.
- **Contract as code**: Keeping the contract language and specification as close as possible to Rust syntax and
  semantics will lower the barrier for users to understand and be able to write their own contracts.

## Ownership and team asks

**Owner:** @celinval and @tautschnig

| Task                         | Owner(s) or team(s) | Notes |
|------------------------------|---------------------|-------|
| Discussion and moral support | ![Team][] [libs]    |       |

### Experimental Contract attributes

| Task             | Owner(s) or team(s)  | Notes                                   |
|------------------|----------------------|-----------------------------------------|
| Author MCP       |                      | ![Complete][] Done already by @pnkfelix |
| Implementation   | @celinval            | In progress.                            |
| Standard reviews | ![Team][] [compiler] |                                         |
| Design meeting   | ![Team][] [compiler] |                                         |

### Standard Library Contracts

| Task                       | Owner(s) or team(s)    | Notes |
|----------------------------|------------------------|-------|
| Standard Library Contracts | @celinval, @tautschnig |       |
| Writing new contracts      | ![Help wanted][]       |       |
| Standard reviews           | ![Team][] [libs]       |       |
### Definitions

Definitions for terms used above:

* *Discussion and moral support* is the lowest level offering, basically committing the team to nothing but good vibes and general support for this endeavor.
* *Author MCP* and *Implementation* means actually writing the code, document, whatever.
* *Design meeting* means holding a synchronous meeting to review a proposal and provide feedback (no decision expected).
* *Standard reviews* refers to reviews for PRs against the repository; these PRs are not expected to be unduly large or complicated.

## Frequently asked questions

[verify-rust-std]: https://github.com/model-checking/verify-rust-std
[MCP-759]: https://github.com/rust-lang/compiler-team/issues/759