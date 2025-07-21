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
This proposal is solely for producing const eval values, not for putting types back into the type system.
That will be a follow-up once this proposal has a merged MVP.

## Motivation & status quo

Creating new general purpose crates (like serialization crates, log/tracing crates, game engine state inspection crates) that should work with almost all other data structures is nontrivial today.
You either need to locally implement your new traits for other (common) crates, or the other crates need to depend on you and implement your traits.
This often hinders rollout and will never reach every crate. Most crate maintainers do not want to depend on 2+ serialization crates, and 3+ logging crates, so they will instead pick one,
causing everyone to either pick the large popular crates or be limited in what they can serializer/log. This is a hindrance to innovation and will (imo) long term cause the ecosystem to
stop evolving even when an objectively better solution to a problem is found.

Reflection offers a way out of this dilemma, as you can write your logic for all types.
You would be processing the type information at runtime (or even preprocess it at compile-time, e.g. in const blocks) without requiring trait bounds on your functions or trait impls anywhere.
This means no one but consumers of your serialization/logging/game-engine will need to know about your crate, and you do not need the entire crates.io ecosystem to add derives for your traits.
Your consumers immediately are able to interoperate with tuples of any size, arbitrary structs and enums from arbitrary crates that neither depend on yours nor you on theirs.

If this experiment is successful, crates like `bevy` will be able to "just work" with arbitrary types instead of requiring authors to `#[derive(Component)]`, `#[derive(Bundle)]`, or `#[derive(Resource)]` their types
just to get the `bevy_reflect` information built at compile-time. Crates like `bevy_reflect` and `facet` will still exist, but only as different libraries with different goals and methods for exposing reflection information.

Furthermore it opens up new possibilities of reflection-like behaviour by
* specializing serialization on specific formats (e.g. serde won't support changing serialization depending on the serializer  https://github.com/serde-rs/serde/issues/2877),
* specializing trait impl method bodies to have more performant code paths for specific types, groups of types or shapes (e.g. based on the layout) of types.

I consider reflection orthogonal to derives as they solve similar problems from different directions. Reflection lets you write the logic that processes your types in a way very similar to dynamic languages, by inspecting values' types during the execution of the reflection code, while derives generate the code that processes types ahead of time. Proc macros derives have historically been shown to be fairly hard to debug and bootstrap from scratch (we should totally also improve proc macro workflows). While reflection can get similarly complex fast, it allows for a more dynamic approach where you can easily debug the state your are in, as you do not have to pair the derive logic with the consumer logic (e.g. a serializer) and are instead directly writing just the consumer logic.

Reflection often is not as efficient as derives, as the derives can generate the ideal code ahead of time, but once a fully functioning reflection system has been written for a use case, and performance becomes a problem, it should be significantly easier to now write a derive for the performance critical cases than to have started doing so from the start. 

### The next 6 months

* add an attribute for `const fn` that prevents them from being called from runtime code or `const fn` without the attribute
    * See the FAQ for why we need `#[rustc_comptime] const fn() {}` declarations
* add basic datastructures to libcore that represent common information about types and the APIs to obtain that information

### The "shiny future" we are working towards

Create basic building blocks that allow `facet`, `bevy-reflect` and `reflect` to process types without requiring derives or trait bounds.

## Design axioms

* Prefer procedural const-eval code over associated const based designs (see also "why not uwuflection" in the FAQ).
    * We picked `const fn` in general evaluation over associated const based designs that are equally expressive but are essentially a DSL
* Ensure privacy is upheld, modulo things like `size_of` exposing whether new private fields have been added
    * This is important to ensure that we cannot break abstractions. We will experiment with allowing const items in the same module to access private fields even if the access is in a comptime fn defined in another crate. Or with a comptime fn defined in the same module of a private field accessing that private field even if called in a const item outside of it.
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
| Lang-team champion   | ![Team][] [lang]                   | TBD |

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

zig's approach to comptime from a very high level is effectively

* generate AST for all source files
* pick the `main` function and start compiling it and looking for what it needs to be compiled
* if a comptime function call is found, look only for what code that needs to compile, compile it and produce the resulting code of the comptime function
* continue the main compilation, which may now invoke the generated code and start compiling that

we do not experiment with this approach at this time, because the compiler is not set up in a way to permit proc macros from accessing type information from the current crate.
While there are ongoing refactorings that go into the direction of potentially allowing more of that in the future, that future seems to be more than 5 years away at my best guess.

* the compiler is not set up to add AST nodes while type information is already available. It possibly never will, and it would be an immense amount of work to get there. I'm doing lots of refactorings that would need to be done for sth like that anyway, even if the goal is just better incremental and general compilar architecture.
* there are too many open language questions about it that we haven't even started to discuss
* a hacky comptime reflection prototype that works for just tuples and that works with regular const eval exists right now, so pursueing the definitely possible implementation will pay off in a shorter term.
