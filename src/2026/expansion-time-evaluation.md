# Architectural groundwork for expansion-time evaluation

| Metadata         |                            |
| :--------------- | -------------------------- |
| Point of contact | @tmandry                   |
| Status           | Proposed                   |
| Flagship         | Constify all the things    |
| [types] champion | @oli-obk |
| [compiler] champion | @petrochenkov |
| Tracking issue   |                            |
| Zulip channel    | N/A                        |
| Needs            | Funding                    |

## Summary

Lay the architectural groundwork for functions that can be evaluated during macro expansion. This speculative work focuses on "queryifying" the resolver and implementing a restricted trait solver mode, enabling future language features like `macro fn` and compiler-integrated interop tools.

**Needs funding:** This goal needs funding to proceed.

## Motivation

### The status quo

The current compiler architecture has two major warts that create limitations for both language evolution and tooling:

1. Monolithic name resolution: Macro expansion needs to resolve names, but name resolution requires expansion to be finished to see all possible items. Because these happen in a single pass, it is currently impossible to invoke queries (like MIR construction or const evaluation) on code within the same crate during expansion.
2. Global trait solving: Trait solving in the current crate normally considers all implementations in that crate. This creates a dependency on every file being parsed and expanded before any trait-based logic can run.

For example, we might want a macro that generates code based on types defined in the same crate, but the compiler cannot yet "pause" expansion to compile and run a function that inspects those types.

### What we propose to do about it

We propose two major architectural changes to unblock expansion-time logic. This work is speculative and may hit unexpected roadblocks, but will involve some refactoring work that is needed anyway.

These changes are purely architectural and do not introduce new stable language features. Instead, they provide the "plumbing" for future possibilities:

* Future language features: Provides a path for `macro fn` or other same-crate procedural logic.
* Compiler integration for interop: Current tools for cross-language interop (e.g., `bindgen`, `cxx`, `crubit`) must run as external build steps. This work would enable the creation of "interop plugins" that run during expansion to generate safe, automated bindings for C++ and other languages, leveraging type information from the current crate.

#### 1. A Restricted Solver Mode
We will implement an experimental attribute (e.g., `#[rustc_expansion_time]`) that restricts a function's body to only see trait implementations from *upstream* crates. By opting out of seeing the current crate's implementations, these functions can be safely compiled and evaluated before the rest of the current crate has finished expanding.

```rust
#[rustc_expansion_time]
fn generate_binding(t: Type) -> TokenStream {
    // This function can only use traits/types from other crates,
    // allowing the compiler to run it during expansion.
    ...
}
```

#### 2. Queryifying the Resolver
We will prototype decoupling macro expansion from the monolithic resolver. This involves making the expansion loop capable of invoking queries on specific items. This is a prerequisite for any future "comptime" features, as it allows the compiler to treat certain functions as "ready to run" even while the rest of the crate is still being parsed.

### Work items over the next year

| Task | Owner(s) | Notes |
| ---- | -------- | ----- |
| Implement restricted solver mode attribute | | @oli-obk to champion; possible owner |
| Decouple expansion from monolithic resolver | | @oli-obk to champion; possible owner |
| Prototype expansion-time function invocation | | |

## Team asks

| Team | Support level | Notes |
| ---- | ------------- | ----- |
| [compiler] | Large | Significant refactoring of the resolver, reviews from @petrochenkov |
| [types] | Medium | Support for the restricted solver mode in the new solver |

## Frequently asked questions

### Does this implement `macro fn`?
No. This work is strictly about the compiler architecture needed to make such a feature *possible* to implement in the future.

### Why do we need a restricted solver mode?
If a function evaluated during expansion could see trait impls in the current crate, adding a new impl anywhere in the crate could change the output of that function, which in turn could generate new impls. This creates a cycle. Restricting the solver to upstream crates breaks this cycle.

### How does this help interop?
It allows interop tools to move from "external code generators" to "compiler-integrated components." An interop macro could run during expansion, use the restricted solver to process foreign types, and generate Rust code directly within the compiler's pipeline.
