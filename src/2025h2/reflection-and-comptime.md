# reflection and comptime

| Metadata         |                                                                                  |
|:-----------------|----------------------------------------------------------------------------------|
| Point of contact | @oli-obk                                                                         |
| Teams            | <!-- TEAMS WITH ASKS -->                                                         |
| Task owners      | <!-- TASK OWNERS -->                                                             |
| Status           | Proposed                                                                         |
| Other Tracking issue   | [rust-lang/rust#142577]                                  |
| Zulip channel    | N/A (an existing stream can be re-used or new streams can be created on request) |

## Summary

Design, implement and experimentally land a reflection scheme based on `const fn` that can only be called at compile time.

## Motivation & status quo

Creating new general purpose crates (like serialization crates, log/tracing crates, game engine state inspection crates) that should work with almost all other data structures is nontrivial today. You either need to locally implement your traits for other crates, or the other crates need to depend on you and implement your traits. This often hinders rollout and will never reach everything.

Reflection offers a way out of this dilemma, as you can write your logic for all types, by processing the type information at runtime (or even preprocess it at compile-time) without requiring trait bounds on your functions or trait impls anywhere.

### The next 6 months

* add an attribute for `const fn` that prevents them from being called from runtime code or `const fn` without the attribute
* add basic datastructures to libcore that represent common information about types and the APIs to obtain that information

### The "shiny future" we are working towards

Create basic building blocks that allow `facet`, `bevy-reflect` and `reflect` to process types without requiring derives or trait bounds.

## Design axioms

* Prefer procedural const-eval code over associated const based designs
    * We picked `const fn` in general evaluation over associated const based designs that are equally expressive but are essentially a DSL
* Ensure privacy is upheld, modulo things like `size_of` exposing whether new private fields have been added
* Avoid new semver hazards and document any if unavoidable.
    * e.g. do not expose private fields, methods, or types

> *This section is optional, but including [design axioms][da] can help you signal how you intend to balance constraints and tradeoffs (e.g., "prefer ease of use over performance" or vice versa). Teams should review the axioms and make sure they agree. [Read more about design axioms][da].*

[da]: ../about/design_axioms.md

## Ownership and team asks

> *This section lists out the work to be done and the asks from Rust teams. Every row in the table should either correspond to something done by a contributor or something asked of a team.*
>
> *For most goals, a single table will suffice, but you can also add subsections with `###`. We give several example subsections below that also demonstrate the most common kinds of goals. Remember that the items in the table only corresponds to what you plan to do over the next 6 months.*
>
> *For items done by a contributor, list the contributor, or ![Heap wanted][] if you don't yet know who will do it. The owner is ideally identified as a github username like `@ghost`.*
>
> *For items asked of teams, list ![Team][] and the name of the team, e.g. `![Team][] [compiler]` or `![Team][] [compiler], [lang]` (note the trailing `[]` in `![Team][]`, that is needed for markdown to parse correctly). For team asks, the "task" must be one of the tasks defined in [rust-project-goals.toml](../rust-project-goals.toml) or `cargo rpg check` will error.*

| Task                         | Owner(s) or team(s) | Notes |
|------------------------------|---------------------|-------|
| Discussion and moral support | ![Team][] [lang]    |       |
| Do the work                  | oli-obk             |       |

### Design language feature to solve problem

> *Some goals propose to design a feature to solve a problem. Typically the outcome from this goal is an draft or accepted RFC. If you would like to work on an experimental implementation in-tree before an RFC is accepted, you can create a [lang team experiment](https://lang-team.rust-lang.org/how_to/experiment.html), but note that a trusted contributor is required.*

| Task                 | Owner(s) or team(s)                | Notes                                                               |
|----------------------|------------------------------------|---------------------------------------------------------------------|
| Lang-team experiment | ![Team][] [lang], [libs]           | Needs libstd data structures (lang items) to make the specialization data available |
| Author RFC           |                                    | Not at that stage in the next 6 months                              |
| Lang-team champion   | ![Team][] [lang]                   | TBD |
| RFC decision         |                                    | Not at that stage in the next 6 months |

### Implement language feature

> *If there is an accepted RFC, or you are doing a [lang-team experiment](https://lang-team.rust-lang.org/how_to/experiment.html), you commonly need someone to write the code, support from the compiler to review your PRs, and possibly lang-team design meetings to review interesting design questions. Once implementation completes we recommend a call for testing blog post.*

| Task                              | Owner(s) or team(s)                | Notes |
|-----------------------------------|------------------------------------|-------|
| Implementation                    | oli-obk |       |
| Standard reviews                  | ![Team][] [compiler]               |       |
| Lang-team champion                | ![Team][] [lang]                   |       |
| Design meeting                    | ![Team][] [lang]                   |       |
| Author call for testing blog post |  | Likely will just experiment with bevy or facet, no general call for testing |

## Frequently asked questions

### Why do you need comptime in addition to reflection?

If we had a `bevy_reflect::Type` type in libcore and a

```rust
const fn type_of(id: TypeId) -> &'static Type;
```

function, that has the special requirement of, unlike every other `const fn`, not being callable at runtime, then we could work with type descriptions from normal procedural rust code. 

So for this experimental impl we would do

```rust
#[compile_time_only]
const fn type_of(id: TypeId) -> &'static Type;
```

These functions can't be run at runtime, because that would require there to be some global table somewhere that maps all `TypeId`s to their repr. This is an obvious no-go in my book.

an demonstration impl (absolutely not salvageable for anything that could be landed!) can be found [here](https://github.com/rust-lang/rust/compare/master...oli-obk:rust:compile-time-reflection)

### Why not continue where uwuflection left off?

See https://soasis.org/posts/a-mirror-for-rust-a-plan-for-generic-compile-time-introspection-in-rust/ for details on what uwuflection is


#### Structural processing

it makes procedural processing of type information very hard. E.g. to get the 3rd element of a tuple you need to 

```rust
<introwospect_type::<YourType> as FieldDescriptor<3>>
```

so to compute that index you need a constant. you can't just use a for loop to iterate over the indices.

we chose const fn over associated consts and generics which could compute the same thing as the const fn, just more expensively and mostly purely functional. so going to assoc consts again seems like it goes against that

This can somewhat be resolved by adding a compile-time for loop feature to the language, that will just expand the body N times in a macro/loop-unrolling kind of way.

#### generic const exprs

in order to use uwuflection in types in generic code you need to either write infallible code with min const generics or you need to add uwuflection bounds (Lots, think typenum) , which defeats the purpose.

### Why not go full zig-style comptime?

* the compiler is not set up to perform codegen while type information is already available. It possibly never will, and it would be an immense amount of work to get there. I'm doing lots of refactorings that would need to be done for sth like that anyway, even if the goal is just better incremental and general compilar architecture.
* there are too many open language questions about it that we haven't even started to discuss
* a hacky prototype that works for just tuples and that works with regular const eval exists right now, so pursueing the definitely possible implementation will pay off in a shorter term.
