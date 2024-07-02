# Relaxing the Orphan Rule

| Metadata | |
| --- | --- |
| Owner(s) | |
| Teams | *lang* |
| Status | WIP |

## Summary

Experimental work to relax orphan rule

## Motivation

Relax the orphan rule, in limited circumstances, to allow crates to provide
implementations of third-party traits for third-party types. The orphan rule
averts one potential source of conflicts between Rust crates, but its presence
also creates scaling issues in the Rust community: it prevents providing a
third-party library that integrates two other libraries with each other, and
instead requires convincing the author of one of the two libraries to add
(optional) support for the other, or requires using a newtype wrapper. Relaxing
the orphan rule, carefully, would make it easier to integrate libraries with
each other, share those integrations, and make it easier for new libraries to
garner support from the ecosystem.

### The status quo

Suppose a Rust developer wants to work with two libraries: `lib_a` providing
trait `TraitA`, and `lib_b` providing type `TypeB`. Due to the orphan rule, if
they want to use the two together, they have the following options:

- Convince the maintainer of `lib_a` to provide `impl TraitA for TypeB`. This
  typically involves an optional dependency on `lib_b`. This usually only
  occurs if `lib_a` is substantially less popular than `lib_b`, or the
  maintainer of `lib_a` is convinced that others are likely to want to use the
  two together. This tends to feel "reversed" from the norm.

- Convince the maintainer of `lib_b` to provide `impl TraitA for TypeB`. This
  typically involves an optional dependency on `lib_a`. This is only likely to
  occur if `lib_a` is popular, and the maintainer of `lib_b` is convinced that
  others may want to use the two together. The difficulty in advocating this,
  scaled across the community, is one big reason why it's difficult to build
  new popular crates built around traits (e.g. competing
  serialization/deserialization libraries, or competing async I/O traits).

- Vendor either `lib_a` or `lib_b` into their own project. This is
  inconvenient, adds maintenance costs, and isn't typically an option for
  public projects intended for others to use.

- Create a newtype wrapper around `TypeB`, and implement `TraitA` for the
  wrapper type. This is less convenient, propagates throughout the crate (and
  through other crates if doing this in a library), and may require additional
  trait implementations for the wrapper that `TypeB` already implemented.

All of these solutions are suboptimal in some way, and inconvenient. In
particular, all of them are much more difficult than actually writing the trait
impl. All of them tend to take longer, as well, slowing down whatever goal
depended on having the trait impl.

### The next six months

As an initial experiment, try relaxing the orphan rule for binary crates, since
this cannot create library incompatibilities in the ecosystem. Allow binary
crates to implement third-party traits for third-party types, possibly
requiring a marker on either the trait or type or both. See how well this works
for users.

As a second experiment, try allowing library crates to provide third-party
impls as long as no implementations actually conflict. Perhaps require marking
traits and/or types that permit third-party impls, to ensure that crates can
always implement traits for their own types.

### The "shiny future" we are working towards

Long-term, we'll want a way to resolve conflicts between third-party trait
impls.

We should support a "standalone derive" mechanism, to derive a trait for a type
without attaching the derive to the type definition. We could save a simple
form of type information about a type, and define a standalone deriving
mechanism that consumes exclusively that information.

Given such a mechanism, we could then permit any crate to invoke the standalone
derive mechanism for a trait and type, and allow identical derivations no
matter where they appear in the dependency tree.

## Design axioms

- **Rustaceans should be able to easily integrate a third-party trait with a
  third-party type without requiring the cooperation of third-party crate
  maintainers.**

- **It should be possible to *publish* such integration as a new crate.** For
  instance, it should be possible to publish an `a_b` crate integrating `a`
  with `b`. This makes it easier to scale the ecosystem and get adoption for
  new libraries.

- **Crate authors should have some control over whether their types have
  third-party traits implemented.** This ensures that it isn't a breaking
  change to introdice first-party trait implementations.

[da]: ../about/design_axioms.md

## Ownership and other resources

**Owner:** TODO

### Support needed from the project

* Lang team:
    * Design meetings to discuss design changes
    * RFC reviews
* Blog post inviting testing, evaluation, and feedback

## Outputs and milestones

### Outputs

The output will be a pair of RFCs:
- A lang RFC proposing a very simple system for binaries to ignore the orphan rule.
- A lang RFC proposing a system with more careful safeguards, to relax the orphan rule for publishable library crates.

### Milestones

- Accepted RFCs.

## Frequently asked questions

### Won't this create incompatibilities between libraries that implement the same trait for the same type?

Yes! The orphan rule is a tradeoff. It was established to avert one source of
potential incompatibility between library crates, in order to help the
ecosystem grow, scale, and avoid conflicts. However, the presence of the orphan
rule creates a different set of scaling issues and conflicts. This project goal
proposes to adjust the balance, attempting to achieve some of the benefits of
both.
