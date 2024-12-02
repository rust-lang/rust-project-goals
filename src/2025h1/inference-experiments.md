# Inference experiments

| Metadata |                       |
| -------- | --------------------- |
| Owner(s) | @GolDDranks           |
| Teams    | [types], [compiler]   |
| Status   | Proposed              |

## Summary

Do experiments with type inference and trait solving to gather ideas for
resolving tension between having flexible, generic APIs and type inference
not working because of too much flexibility.

## Motivation

Comparing a `u32` and a `i8` is always having a meaningful result, yet `PartialOrd` with these
types is not defined. Indexing slices, vectors and arrays with `u16` should be fine as an perfectly
valid operation, but indexing is supported only by `usize`.

The alternative is to have more flexibility with these examples and other APIs.
Working as intended, this would make awkward and error-prone `as` coercions rarer, reduce visual
clutter and make the operations more ergonomic. However, the added flexibility would also make
the inferring the types and selecting traits harder, and would require – counterproductively –
more type annotations. Relaxing/generalizing API limitations is also a common source of backwards
incompatibility in the ecosystem, leading in the worst case, widespread inference breakages.

Resolving this tension would make Rust both more ergonomic and robust, and would improve
the API evolution story.

### The status quo

The tension with too little and too much API flexibility seems to be in the interaction
between the type inference algorithm, and the generics and traits system; the inference algorithm
uses results of trait solving to infer types that would be unknown in a generic context.

There is a Chalk-based experimental, external trait solver and a next-gen trait solver being introduced
in Rust, so experiments with inference and trait solving would likely to involve working with these
projects.

It is unknown to me if there is any significant existing attempts at solving the mentioned tension,
but I have witnessed some resistance towards complicating the inference further. This is why I think
the situation could benefit from experiments.

### The next 6 months

- Get up to speed with the type inference and trait solving in rustc
- Prepare and document test cases and motivating APIs that would benefit from resolving the tension
- Perform experiments with modified inference and trait solving algorithms (out of the tree at first?)
  - Gather ideas from well-informed stakeholders not to restrict to just mine
  - My current ideas:
    - For inference:
      - marking an impl for a "default" to resolve multiple-impls-apply-situations
      - tiered/ranked impl applicability
    - For backwards compatibility: inferred types lockfiles
- Gather ideas and feedback, and start writing an RFC if there are promising avenues

### The "shiny future" we are working towards

Having less "noisy" type conversions, and more API that "just work" as you expect.
To be sure, a goal of adding flexibility and ergonomics doesn't and shouldn't urge
us to accept APIs where inferring an arbitrary choice of a type would risk correctness or
would made code more opaque or hard to understand.

Ideally, we'd like to also get to a world where type inference stops being a backcompability
hazard.

## Design axioms

- Experimental. Not having a single preferred design, but trying many things to gather experience.
- Ergonomics. Using generic APIs could be simple and enjoyable; not only for writing code, but also for reading code.
- Correctness. Strive for enhancing writing correct code and reduce footguns.
- Clarity. If we end up introducing new features to the type inference or trait solving system,
  they should work in straightforward and unsurprising, predictable ways.
- Robustness. Strive to improve inference in a way that reduces backward compability hazards.

## Ownership and team asks

**Owner:** @GolDDranks

| Subgoal                                        | Owner(s) or team(s)            | Notes |
| ---------------------------------------------- | ------------------------------ | ----- |
| Discussion and moral support | ![Team][] [lang] [types]  |       |
| Gather ideas for motivating APIs | ![Team][] [libs-api]  |       |
| Gather ideas for experiments | ![Team][] [lang] [types]  |       |
| Start experimenting out of tree | GolDDranks  | Likely to need mentoring with rustc |
| Gather feedback for experiments | ![Team][] [lang] [types] [libs-api] |       |

## Frequently asked questions

### None yet?
