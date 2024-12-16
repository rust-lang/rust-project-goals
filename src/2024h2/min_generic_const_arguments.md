# "Stabilizable" prototype for expanded const generics

| Metadata       |                                    |
| ---            | ---                                |
| Owner(s)       | @BoxyUwU                           |
| Teams          | [types]                            |
| Status         | Accepted                           |
| Tracking issue | [rust-lang/rust-project-goals#100] |


## Summary

Experiment with a new `min_generic_const_args` implementation to address challenges found with the existing approach

## Motivation

`min_const_generics` was stabilized with the restriction that const-generic arguments may not use generic parameters other than a bare const parameter, e.g. `Foo<N>` is legal but not `Foo<{ T::ASSOC }>`. This restriction is lifted under `feature(generic_const_exprs)` however its design is fundamentally flawed and introduces significant complexity to the compiler. A ground up rewrite of the feature with a significantly limited scope (e.g. `min_generic_const_args`) would give a viable path to stabilization and result in large cleanups to the compiler.

### The status quo

A large amount of rust users run into the `min_const_generics` limitation that it is not legal to use generic parameters with const generics. It is generally a bad user experience to hit a wall where a feature is unfinished, and this limitation also prevents patterns that are highly desirable. We have always intended to lift this restriction since stabilizing `min_const_generics` but we did not know how.

It is possible to use generic parameters with const generics by using `feature(generic_const_exprs)`. Unfortunately this feature has a number of fundamental issues that are hard to solve and as a result is *very* broken. It being so broken results in two main issues:
- When users hit a wall with `min_const_generics` they cannot reach for the `generic_const_exprs` feature because it is either broken or has no path to stabilization.
- In the compiler, to work around the fundamental issues with `generic_const_exprs`, we have a number of hacks which negatively affect the quality of the codebase and the general experience of contributing to the type system.

### The next six months

We have a design for `min_generic_const_args` approach in mind but we want to validate it through implementation as const generics has a history of unforeseen issues showing up during implementation. Therefore we will pursue a prototype implementation in 2024.

As a stretch goal, we will attempt to review the design with the lang team in the form of a design meeting or RFC. Doing so will likely also involve authoring a design retrospective for `generic_const_exprs` in order to communicate why that design did not work out and why the constraints imposed by `min_generic_const_args` makes sense.

### The "shiny future" we are working towards

The larger goal here is to lift most of the restrictions that const generics currently have:
- Arbitrary types can be used in const generics instead of just: integers, floats, bool and char.
    - implemented under `feature(adt_const_params)` and is relatively close to stabilization
- Generic parameters are allowed to be used in const generic arguments (e.g. `Foo<{ <T as Trait>::ASSOC_CONST }>`).
- Users can specify `_` as the argument to a const generic, allowing inferring the value just like with types.
    - implemented under `feature(generic_arg_infer)` and is relatively close to stabilization
- Associated const items can introduce generic parameters to bring feature parity with type aliases
    - implemented under `feature(generic_const_items)`, needs a bit of work to finish it. Becomes significantly more important *after* implementing `min_generic_const_args`
- Introduce associated const equality bounds, e.g. `T: Trait<ASSOC = N>` to bring feature parity with associated types
    - implemented under `feature(associated_const_equality)`, blocked on allowing generic parameters in const generic arguments


Allowing generic parameters to be used in const generic arguments is the only part of const generics that requires significant amounts of work while also having significant benefit. Everything else is already relatively close to the point of stabilization. I chose to specify this goal to be for implementing `min_generic_const_args` over "stabilize the easy stuff" as I would like to know whether the implementation of `min_generic_const_args` will surface constraints on the other features that may not be possible to easily fix in a backwards compatible manner. Regardless I expect these features will still progress while `min_generic_const_args` is being implemented.


## Design axioms

- Do not block future extensions to const generics
- It should not feel worse to write type system logic with const generics compared to type generics
- Avoid post-monomorphization errors
- The "minimal" subset should not feel arbitrary

## Ownership and team asks

**Owner:** @BoxyUwU, project-const-generics lead, T-types member

This section defines the specific work items that are planned and who is expected to do them. It should also include what will be needed from Rust teams.

* Subgoal:
    * Describe the work to be done and use `↳` to mark "subitems".
* Owner(s) or team(s):
    * List the owner for this item (who will do the work) or ![Help wanted][] if an owner is needed.
    * If the item is a "team ask" (i.e., approve an RFC), put ![Team][] and the team name(s).
* Status:
    * List ![Help wanted][] if there is an owner but they need support, for example funding.
    * Other needs (e.g., complete, in FCP, etc) are also fine.

| Task                         | Owner(s) or team(s)      | Notes |
| ---------------------------- | ------------------------ | ----- |
| Discussion and moral support | ![Team][] [lang] [types] |       |
| Implementation and mentoring | @BoxyUwu                 |       |
| Implementation               | @camelid                 |       |
| Reviewer                     | @compiler-errors         |       |

## Outputs and milestones

### Outputs

- A sound, fully implemented `feature(min_generic_const_args)` available on nightly
- All issues with `generic_const_exprs`'s design have been comprehensively documented (stretch goal)
- RFC for `min_generic_const_args`'s design (stretch goal)

### Milestones

- Prerequisite refactorings for `min_generic_const_args` have taken place
- Initial implementation of `min_generic_const_args` lands and is useable on nightly
- All known issues are resolved with `min_generic_const_args`
- Document detailing `generic_const_exprs` issues
- RFC is written and filed for `min_generic_const_args`

## Frequently asked questions

### Do you expect `min_generic_const_args` to be stabilized by the end?

No. The feature should be fully implemented such that it does not need any more work to make it ready for stabilization, however I do not intend to actually set the goal of stabilizing it as it may wind up blocked on the new trait solver being stable first.