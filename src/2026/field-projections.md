# Field Projections

| Metadata         |                                                                                  |
| :--------------- | -------------------------------------------------------------------------------- |
| Point of contact | @BennoLossin                                                                     |
| Status           | Proposed                                                                         |
| Tracking issue   | [rust-lang/rust-project-goals#390]                                                         |
| Zulip channel    | https://rust-lang.zulipchat.com/#narrow/channel/522311-t-lang.2Fcustom-refs      |

## Summary

We aim to explore and refine the *virtual places* approach for field projections, document its design and interactions in the [beyond-refs wiki](https://rust-lang.github.io/beyond-refs/), implement it as an experiment in the compiler, and prepare RFCs based on the findings.

This is a continuing goal, see [the goal document of the previous period](https://rust-lang.github.io/rust-project-goals/2025h2/field-projections.html) for historical information.

## Motivation

Field projections aim to provide a uniform and ergonomic way to produce pointers to fields for custom pointer types. This mirrors the operation on built-in references available today: `&x.field` is of type `&Field` given that `x: &Struct`. This project goal aims to make this feature generally available for all custom types and implement it for `&mut MaybeUninit<T>`, `NonNull<T>` and many more.

This feature will reduce verbosity and increase ergonomics when working with custom pointer types. It also enhances expressiveness, as it provides native borrow checker integration and allows the simultaneous usage of pointers to disjoint fields of the same struct.

### The status quo

There are many examples for types that can take advantage of field projections. In its current form the design is a generalization of `Deref` that provides an umbrella abstraction for *pointers to virtual places*. As the name suggests, these places do not really need to exist, so `struct MyStruct<T>(PhantomData<T>)` is supported by our approach. Naturally any type that implements `Deref` is supported; but also types that cannot implement it, such as raw pointers, `NonNull<T>` and many more are covered.

The approach of *field projection via virtual places* puts places at the center. It adds a lot of operator traits for *place operations* such as `PlaceRead`, `PlaceWrite`, `PlaceMove`, and most importantly `PlaceBorrow`. We are missing some interactions, concrete details and a comprehensive document on this design, but the overall idea is solid.

In the previous goal period, we held a [design meeting](https://hackmd.io/@rust-lang-team/S1I1aEc_lx) about the general approach to designing a solution. In it we also extensively covered many use-cases. Here is a non-exhaustive list of types that would benefit from this feature:
- `&mut MaybeUninit<T>`,
- `cell::Ref[Mut]<'a, T>`
- `NonNull<T>` and `*{const,mut} T`
- `pyo3::pycell::PyRef[Mut]<'_, T>`
- `Cpp[Mut]Ref<T>`
- `&[mut] Cell<T>`
- `&Atomic<T>`
- `ArcRef<T>`
- `VolatilePtr<'_, T>`
- `&[mut] Untrusted<T>`
- `SeqLockRef<'_, T>`
- `UserPtr<T>`
- `Ptr<'_, T>`

We also want to note that another contributor separately (without being aware of the field projection effort) created an [ACP for `ArcRef<T>`](https://github.com/rust-lang/libs-team/issues/700). Another [wanted to introduce swift's keypath feature](https://rust-lang.zulipchat.com/#narrow/channel/213817-t-lang/topic/Fields.20projection.20with.20Keypaths/with/567490323) to Rust, which is very similar to a part of field projection.

The design axioms from last period still apply and are fulfilled by the virtual places approach: 

- **Effortless Syntax.** Using field projections in a non-generic context should look very similar to normal field accesses.
- **Broad Solution.** Field projections should be very general and solve complex projection problems such as pin-projections and [`RcuMutex<T>`](https://hackmd.io/@rust-lang-team/S1I1aEc_lx#RCU-Read-Copy-Update).

### The next 6 months

| Task        | Owner(s) | Notes |
| ----------- | -------- | ----- |
| Establish a working group for field projections | @tmandry | The group should start out with the members: @BennoLossin, @Nadrieril, @tmandry, @dingxiangfei2009. Any contributor is welcome to join if they intend to develop and design field projections. |
| Explore and map the solution space | field projection working group | |
| Document the design in the wiki | field projection working group | |
| Implement a compiler experiment | field projection working group | |
| Draft RFCs | field projection working group | |

TODO:
- a-mir-formality?

### The "shiny future" we are working towards

Field projections is part of a larger idea called *beyond references*. There should be no built-in types in the Rust language that a library could not recreate. For example, a user should be able to implement a `MyBox<T>` that allows moving in and out, supports unsizing, coercions and borrowing the contents using references. Field projections enable that last feature as well as the moving out support. Ultimately, using any library declared type should feel as if it was built into the language itself. In this future, no significant compiler magic exists for references and they can be fully implemented in `core`.

## Team asks

| Team       | Support level | Notes                                   |
| ---------- | ------------- | --------------------------------------- |
| [lang]     | Large         | Aiming for two design meetings; large language feature [^lang] |
| [compiler] | Medium        | Reviews of big changes needed; also looking for implementation help |
| [libs]     | Small         | Small reviews of RFC and/or lirary PRs (implementing FP for core & std types) |
| [types]    | Small         | Small reviews of RFC and/or compiler PRs |
| [opsem]    | Small         | Small reviews of RFC and/or compiler PRs |

[^lang]: Maybe this should be Medium, since the feature direction is pretty set and doesn't need too much lang-guidance. However, we could end up with an RFC review at the end of the year.

## Frequently asked questions
