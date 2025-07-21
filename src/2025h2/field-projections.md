# Design a language feature to solve Field Projections

| Metadata         |                                                                                  |
|:-----------------|----------------------------------------------------------------------------------|
| Point of contact | @BennoLossin                                                                     |
| Teams            | <!-- TEAMS WITH ASKS -->                                                         |
| Task owners      | <!-- TASK OWNERS -->                                                             |
| Status           | Proposed                                                                         |
| Tracking issue   |                                                                                  |
| Zulip channel    | N/A                                                                              |

## Summary

Figure out the best design for field projections. Update the existing [Field Projections RFC] or
author a new one and implement it for use in nightly via a lang experiment.

[Field Projections RFC]: https://github.com/rust-lang/rfcs/pull/3735

## Motivation

Rust makes extensive use of smart pointers (`Box<T>`, `Rc<T>`, `Arc<T>`), modified references (`&mut
MaybeUninit<T>`, `Pin<&mut T>`) and custom pointer types (`NonNull<T>`).

Some of these types implement the `Deref[Mut]` trait(s) allowing one to access fields of the type
`T`. But not all of them can implement it due to various reasons. However, they often *can* support
operations that "index" into the fields of the type `T`. For example `&mut MaybeUninit<Struct>`
conceptually has fields of type `&mut MaybeUninit<Field>`.

### The status quo

Rust has a lot of container types that make it difficult to directly interact with fields of structs
that they wrap. For example:
- `MaybeUninit<T>`,
- `UnsafeCell<T>`,
- `Cell<T>`

It also has several pointer-like types that could support a natural pointer-to-field operation. For
example:
- `NonNull<T>`,
- `*const T` / `*mut T`,
- `cell::Ref<'_, T>` / `cell::RefMut<'_, T>`

Additionally, there is `Pin<&mut T>`, which already has a well-established name for this operation:
pin-projections. The ecosystem provides several crates to add this operation to the struct itself.

#### Custom types

A plethora of types making use of field projections are found in the context of Rust for Linux.
Therefore they might -- with high probability -- come up in other embedded projects too.

- `VolatilePtr<'a, T>` like `*mut T`, but with a lifetime & all accesses to the pointer are
  volatile.
- `Ptr<'a, T>` like a `&'a T` but without certain rust guarantees (most likely `&'a UnsafePinned<T>`
  under the hood).
- `RcuMutex<T>` a safe abstraction for RCU (a special synchronization primitive in the kernel)
  working together with a `Mutex<T>` to synchronize accesses to data (this requires complex
  projections, only allowing certain fields to be projected).
- `SeqLockRef<'_, T>`
- `AtomicPtr<T>` where `T` is a small enough type composed of integers.
- `UserPtr<T>` a pointer into userspace

Additionally, Rust for Linux could take advantage of field information present in the current
proposal. Essentially answering the question "does this type have a field of type X at offset Y?"
via traits.

Note that the projections listed above are also very important to Rust for Linux. Virtually all
types are pinned in the kernel, so `Pin<&mut T>` comes up a lot in drivers. We're also handling raw
pointers very often where we could use `NonNull<T>` instead if they had better field access.

#### Current proposals

In addition to [Field Projections RFC v2] already mentioned above, there is a [newer
proposal](https://hackmd.io/@BennoLossin/HkMBy6Hzlx) that improves upon it.

For historical context, there also is the [Field Projections RFC v1].

[Field Projections RFC v1]: https://github.com/rust-lang/rfcs/pull/3318
[Field Projections RFC v2]: https://github.com/rust-lang/rfcs/pull/3735

### The next 6 months

Have design meetings with the relevant parties & update the existing or write a new RFC.

### The "shiny future" we are working towards

Have field projections available in stable Rust.

## Design axioms

- **Effortless Syntax.** Using field projections in a non-generic context should look very similar
  to normal field accesses.
- **Broad Solution.** Field projections should be very general and solve complex projection problems
  such as pin-projections and `RcuMutex<T>`.

## Ownership and team asks

| Task                 | Owner(s) or team(s)                 | Notes                                                               |
|----------------------|-------------------------------------|---------------------------------------------------------------------|
| Design meeting       | ![Team][] [lang]                    | Possibly more than one required as well as discussions on zulip.    |
| Lang-team experiment | ![Team][] [lang]                    | @dingxiangfei2009, @BennoLossin                                     |
| Author RFC           | @BennoLossin                        |                                                                     |
| Lang-team champion   | ![Team][] [lang]                    | *Champion Needed*                                                   |
| RFC secondary review | ![Team][] [types]                   | might be a good idea?                                               |
| RFC decision         | ![Team][] [lang]                    |                                                                     |


## Frequently asked questions

