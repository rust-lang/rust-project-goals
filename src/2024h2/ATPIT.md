# Associated type position impl trait

| Metadata       |                                    |
|----------------|------------------------------------|
| Owner(s)       | @oli-obk                           |
| Teams          | [types], [lang]                    |
| Status         | Accepted                           |
| Tracking issue | [rust-lang/rust-project-goals#103] |


## Summary

Stable support for `impl Trait` in the values of associated types (aka "associated type position impl trait" or ATPIT)

## Motivation

Rust has been on a long-term quest to support `impl Trait` in more and more locations ("impl Trait everywhere"). The next step in that journey is supporting `impl Trait` in the values of associated types (aka "associated type position impl trait" or ATPIT), which allows impls to provide more complex types as the value of an associated type, particularly anonymous types like closures and futures. It also allows impls to hide the precise type they are using for an associated type, leaving room for future changes. This is the latest step towards the overall vision of support `impl Trait` notation in various parts of the Rust language.

### The status quo

Impls today must provide a precise and explicit value for each associated type. For some associated types, this can be tedious, but for others it is impossible, as the proper type involves a closure or other aspect which cannot be named. Once a type is specified, impls are also unable to change that type without potentially breaking clients that may have hardcoded it.

Rust's answer to these sorts of problems is `impl Trait` notation, which is used in a number of places within Rust to indicate "some type that implements `Trait`":

* Argument position impl Trait ("APIT"), in inherent/item/trait functions, in which `impl Trait` desugars to an anonymous method type parameter (sometimes called "universal" impl Trait);
* Return type position in inherent/item functions ("RPIT") and in trait ("RPITIT") functions, in which `impl Trait` desugars to a fresh opaque type whose value is inferred by the compiler.

ATPIT follows the second pattern, creating a new opaque type.

### The next six months

The plan for 2024 is to stabilize Associated Type Position Impl Trait (ATPIT). The design has been finalized from the lang team perspective for some time, but the types team is still working out final details. In particular, the types team is trying to ensure that whatever programs are accepted will also be accepted by the [next generation trait solver](./next-solver.md), which handles opaque types in a new and simplified way.

### The "shiny future" we are working towards

This goal is part of the ["impl Trait everywhere"](https://rust-lang.github.io/impl-trait-initiative/) effort, which aims to support `impl Trait` in any position where it makes sense. With the completion of this goal we will support `impl Trait` in

* the type of a function argument ("APIT") in inherent/item/trait functions;
* return types in functions, both inherent/item functions ("RPIT") and trait functions ("RPITIT");
* the value of an associated type in an impl (ATPIT).

Planned extensions for the future include:

* allowing `impl Trait` in type aliases ("TAIT"), like `type I = impl Iterator<Item = u32>`;
* allowing `impl Trait` in let bindings ("LIT"), like `let x: impl Future = foo()`;
* dyn safety for traits that make use of RTPIT and async functions.

Other possible future extensions are:

* allowing `impl Trait` in where-clauses ("WCIT"), like `where T: Foo<impl Bar>`;
* allowing `impl Trait` in struct fields, like `struct Foo { x: impl Display }`;

See also: the [explainer](https://rust-lang.github.io/impl-trait-initiative/explainer.html) here for a "user's guide" style introduction, though it's not been recently updated and may be wrong in the details (especially around TAIT).

## Design axioms

None.

## Ownership and team asks

**Owner:** oli-obk owns this goal.

| Task                   | Owner(s) or team(s)      | Notes |
|------------------------|--------------------------|-------|
| Implementation         | @oli-obk                 |       |
| FCP decision(s)           | ![Team][] [types]        |       |
| Stabilization decision | ![Team][] [types] [lang] |       |

## Frequently asked questions

None yet.