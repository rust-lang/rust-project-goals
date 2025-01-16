# Next-generation trait solver

| Metadata       |                                    |
|----------------|------------------------------------|
| Point of contact | @lcnr                              |
| Teams | <!-- TEAMS WITH ASKS --> |
| Task owners      | <!-- TASK OWNERS --> |
| Status         | Proposed                           |
| Tracking issue | [rust-lang/rust-project-goals#113] |
| Zulip channel  | [#t-types/trait-system-refactor][channel] |

[channel]: https://rust-lang.zulipchat.com/#narrow/channel/364551-t-types.2Ftrait-system-refactor


## Summary

Continue work towards the stabilization of `-Znext-solver=globally`, collecting and resolving remaining blockers. Extend its use in lints and rustdoc.

## Motivation

The next-generation trait solver is intended to fully replace the existing type system components responsible for proving trait bounds, normalizing associated types, and much more. This should fix many long-standing (soundness) bugs, enable future type system improvements, and improve compile-times. It's tracking issue is [#107374](https://github.com/rust-lang/rust/issues/107374).

### The status quo

There are multiple type system unsoundnesses blocked on the next-generation trait solver: [project board][unsoundnesses]. Desirable features such as coinductive trait semantics and perfect derive, where-bounds on binders, and better handling of higher-ranked bounds and types are also stalled due to shortcomings of the existing implementation.

Fixing these issues in the existing implementation is prohibitively difficult as the required changes are interconnected and require major changes to the underlying structure of the trait solver. The Types Team therefore decided to rewrite the trait solver in-tree, and has been working on it since EOY 2022.

### The next six months

- resolve remaining issues affecting our compile-time benchmarks
    - fix the failing tests by 'properly' resolving the underlying issues
        - `wg-grammar`
        - `projection-caching`
        - `nalgebra-0.33.0`
    - improve performance
        - avoid exponential performance hits in all benchmarks
        - get most benchmarks to be neutral or improvements
- go through the most popular crates on crates.io and fix any encountered issues
- move additional lints and rustdoc to use the new solver by default
- publicly ask for testing of `-Znext-solver=globally` once that's useful


### The "shiny future" we are working towards

- we are able to remove the existing trait solver implementation and significantly cleanup the type system in general, e.g. removing most `normalize` in the caller by handling unnormalized types in the trait system
- all remaining type system unsoundnesses are fixed
- many future type system improvements are unblocked and get implemented
- the type system is more performant, resulting in better compile times

## Design axioms

In order of importance, the next-generation trait solver should be:
- sound: the new trait solver is sound and its design enables us to fix all known type system unsoundnesses
- backwards-compatible: the breakage caused by the switch to the new solver should be minimal
- maintainable: the implementation is maintainable, extensible, and approachable to new contributors 
- performant: the implementation is efficient, improving compile-times 

[da]: ../about/design_axioms.md

## Ownership and team asks

**Owner:** @lcnr

Add'l implementation work: @compiler-errors

| Task                         | Owner(s) or team(s)     | Notes                      |
|------------------------------|-------------------------|----------------------------|
| Discussion and moral support | ![Team][] [types]       |                            |
| Implementation               | @lcnr, @compiler-errors |                            |
| Standard reviews             | ![Team][] [types]       |                            |
| FCP decision(s)              | ![Team][] [types][]     | for necessary refactorings |

### Support needed from the project

* [Types] team
    * review design decisions
    * provide technical feedback and suggestion

## Outputs and milestones

See next few steps :3

### Outputs

### Milestones

## Frequently asked questions

### What do I do with this space?

*This is a good place to elaborate on your reasoning above -- for example, why did you put the design axioms in the order that you did? It's also a good place to put the answers to any questions that come up during discussion. The expectation is that this FAQ section will grow as the goal is discussed and eventually should contain a complete summary of the points raised along the way.*

[unsoundnesses]: https://github.com/orgs/rust-lang/projects/44