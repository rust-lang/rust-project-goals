# Impl trait everywhere

| Metadata | |
| --- | --- |
| Owner(s) | [oli-obk] |
| Teams | [Lang], [Types] |
| Status | WIP |

[oli-obk]: https://github.com/oli-obk

[Lang]: https://www.rust-lang.org/governance/teams/lang
[Types]: https://www.rust-lang.org/governance/teams/compiler#team-types

## Motivation

The `impl Trait` syntax is a key enabler for Rust libraries.
It not only permits shorter, more meaningful type signatures and documentation,
it also enables developers to name types (like futures and closures) that are otherwise anonymous.

### The status quo

Impl trait is supported in the following positions:

* Argument position impl Trait ("APIT"), in inherent/item/trait functions 
* Return type position in inherent/item functions ("RPIT") and in trait ("RPITIT") functions

### The next few steps

The plan for 2024 is to

* Stabilize new capture rules for the [Rust 2024 edition](./Rust-2024-Edition.md).
    * RFC proposing `use<T>` syntax was recently opened.
* Stabilize Associated Type Position Impl Trait (ATPIT):
    * Permits `impl Foo for Bar { type Baz = impl Trait; }`.
    * This is useful for associated types whose value is a future, iterator, closure, or simpler type.
* Stabilize Type Alias Impl Trait (TAIT) -- stretch goal.
    * Permits `type Foo = impl Trait;`
    * This is a core feature that many other uses of impl Trait effectively desugar to, and thus can be used to close gaps where those features don't quite do what is needed.
    * It allows the inferred types to be referenced in struct fields or other unusual places; it also allows for those types to be part of a module's public API.

### The "shiny future" we are working towards

Long-term, we wish to enable impl trait syntax "everywhere", meaning in any position where it makes sense.
We are nearing the end of this journey.
Some of the remaining extensions that could be considered in the future:

* dyn safety for traits that make use of RTPIT and async functions;
* in where-clauses in functions or other locations;
* in struct fields.

See also: the [explainer](https://rust-lang.github.io/impl-trait-initiative/explainer.html) here for a "user's guide" style introduction, though it's not been recently updated and may be wrong in the details (especially around TAIT).

## Design axioms

TODO

## Ownership and other resources

**Owner:** oli-obk owns this goal, work sponsored by Amazon.

### Support needed from the project

* Lang team:
    * Design meetings to discuss design changes
    * RFC reviews
* Types team:
    * Design reviews to ensure compatibility with upcoming scheduler

## Outputs and milestones

### Outputs

* Stable version of ATPIT
* Stable version of TAIT

### Milestones

* ATPIT
* 

## Frequently asked questions

None yet.