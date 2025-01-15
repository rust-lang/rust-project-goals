# Next-generation trait solver

| Metadata       |                                           |
|----------------|-------------------------------------------|
| Point of contact | @lcnr                                   |
| Teams | <!-- TEAMS WITH ASKS --> |
| Task owners      | <!-- TASK OWNERS --> |
| Status         | Accepted                                  |
| Tracking issue | [rust-lang/rust-project-goals#113]        |
| Zulip channel  | [#t-types/trait-system-refactor][channel] |

[channel]: https://rust-lang.zulipchat.com/#narrow/channel/364551-t-types.2Ftrait-system-refactor


## Summary

In the next 6 months we plan to extend the next-generation trait solver as follows:

- stabilize the use of the next-generation trait solver in coherence checking
- use the new implementation in rustdoc and lints where applicable 
- share the solver with rust-analyser
- successfully bootstrap the compiler when exclusively using the new implementation and run crater

## Motivation

The existing trait system implementation has many bugs, inefficiencies and rough corners which require major changes to its implementation. To fix existing unsound issues, accommodate future improvements, and to improve compile times, we are reimplementing the core trait solver to replace the existing implementations of `select` and `fulfill`.

### The status quo

There are multiple type system unsoundnesses blocked on the next-generation trait solver: [project board][unsoundnesses]. Desirable features such as coinductive trait semantics and perfect derive, where-bounds on binders, and better handling of higher-ranked bounds and types are also stalled due to shortcomings of the existing implementation.

Fixing these issues in the existing implementation is prohibitively difficult as the required changes are interconnected and require major changes to the underlying structure of the trait solver. The Types Team therefore decided to rewrite the trait solver in-tree, and has been working on it since EOY 2022.

### The next six months

- stabilize the use of the next-generation trait solver in coherence checking
- use the new implementation in rustdoc and lints where applicable 
- share the solver with rust-analyser
- successfully bootstrap the compiler when exclusively using the new implementation and run crater

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

| Task                         | Owner(s) or team(s) | Notes |
|------------------------------|---------------------|-------|
| Discussion and moral support | ![Team][] [types]   |       |

### Stabilize next-generation solver in coherence

| Task                   | Owner(s) or team(s)       | Notes |
|------------------------|---------------------------|-------|
| Implementation         | @lcnr, @compiler-errors   |       |
| Standard reviews       | ![Team][] [types]         |       |
| Standard reviews       | ![Team][] [rust-analyzer] |       |
| Stabilization decision | ![Team][] [types][]       |       |

### Support next-generation solver in rust-analyzer

| Task                                | Owner(s) or team(s)       | Notes |
|-------------------------------------|---------------------------|-------|
| Implementation (library side)       | owner and others          |       |
| Implementation (rust-analyzer side) | TBD                       |       |
| Standard reviews                    | ![Team][] [types]         |       |
| Standard reviews                    | ![Team][] [rust-analyzer] |       |


### Support needed from the project

* [Types] team
    * review design decisions
    * provide technical feedback and suggestion
* [rust-analyzer] team
    * contribute to integration in Rust Analyzer
    * provide technical feedback to the design of the API

## Outputs and milestones

See next few steps :3

### Outputs

### Milestones

## Frequently asked questions

### What do I do with this space?

*This is a good place to elaborate on your reasoning above -- for example, why did you put the design axioms in the order that you did? It's also a good place to put the answers to any questions that come up during discussion. The expectation is that this FAQ section will grow as the goal is discussed and eventually should contain a complete summary of the points raised along the way.*

[unsoundnesses]: https://github.com/orgs/rust-lang/projects/44