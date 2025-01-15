# Experiment with relaxing the Orphan Rule

| Metadata |                     |
| -------- | ------------------- |
| Point of contact | @nikomatsakis    |
| Teams | <!-- TEAMS WITH ASKS --> |
| Task owners      | <!-- TASK OWNERS --> |
| Status   | Not accepted        |
| Zulip channel  | N/A                                |

## Summary

Experimental implementation and draft RFCs to relax the orphan rule

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

We propose to

- Experiment on nightly with alternate orphan rules
  - Idea 1. Try relaxing the orphan rule for binary crates, since
this cannot create library incompatibilities in the ecosystem. Allow binary
crates to implement third-party traits for third-party types, possibly
requiring a marker on either the trait or type or both. See how well this works
for users.
  - Idea 2. Try allowing library crates to provide third-party
impls as long as no implementations actually conflict. Perhaps require marking
traits and/or types that permit third-party impls, to ensure that crates can
always implement traits for their own types.
- Draft RFCs for features above, presuming experiments turn out well

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

## Ownership and team asks

**Owner:** ![Help wanted][]

This section defines the specific work items that are planned and who is expected to do them. It should also include what will be needed from Rust teams.

* Subgoal:
    * Describe the work to be done and use `↳` to mark "subitems".
* Owner(s) or team(s):
    * List the owner for this item (who will do the work) or ![Help wanted][] if an owner is needed.
    * If the item is a "team ask" (i.e., approve an RFC), put ![Team][] and the team name(s).
* Status:
    * List ![Help wanted][] if there is an owner but they need support, for example funding.
    * Other needs (e.g., complete, in FCP, etc) are also fine.

| Task                          | Owner(s) or team(s)      | Notes                      |
| ----------------------------- | ------------------------ | -------------------------- |
| Ownership and implementation  | ![Help wanted][]         |                            |
| RFC authoring                 | ![Help wanted][]         |                            |
| Design consultation/iteration | Josh Triplett            |                            |
| Design meeting                | ![Team][] [lang] [types] | Up to 1 meeting, if needed |

## Frequently asked questions

### Won't this create incompatibilities between libraries that implement the same trait for the same type?

Yes! The orphan rule is a tradeoff. It was established to avert one source of
potential incompatibility between library crates, in order to help the
ecosystem grow, scale, and avoid conflicts. However, the presence of the orphan
rule creates a different set of scaling issues and conflicts. This project goal
proposes to adjust the balance, attempting to achieve some of the benefits of
both.

### Why was this goal not approved for 2024H2?

Primarily for capacity reasons:

* lcnr [commented](https://github.com/rust-lang/rfcs/pull/3672/files/c73149a285c46d3f2d29a0226df6226bd8f3754f#r1679323797) that 
  there was no capacity on the types team for reviewing.
* tmandry [commented](https://github.com/rust-lang/rfcs/pull/3672/files/c73149a285c46d3f2d29a0226df6226bd8f3754f#r1679799818) that the
  goal as written was not necessarily focused on the right constraints (text quoted below).

> It strikes me as quite open ended and not obviously focused on the right constraints. (cc @joshtriplett as mentor)
>
> For example, we could choose to relax the orphan rule only within a restricted set of co-versioned crates that we treat as "one big crate" for coherence purposes. This would not meet the axioms listed in the goal, but I believe it would still improve things for a significant set of users.
>
> If we instead go with visibility restrictions on impls, that might work and solve a larger subset, but I think the design will have to be guided by someone close to the implementation to be viable.
>
> I would love to have a design meeting if a viable looking design emerges, but I want to make sure this feedback is taken into account before someone spends a lot of time on it.

These points can be considered and addressed at a later time.
