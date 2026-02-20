# Control over Drop semantics

| Metadata         |                |
|:-----------------|----------------|
| Point of contact | @thunderseethe |
| Status           | Proposed       |
| Needs            | Contributor    |
| Needs            | Funding        |
| Tracking issue   |                |
| Zulip channel    | N/A            |

## Summary

Allow users to easily control the drop semantics of struct fields, letting them change drop order, and disable recursive destructors to make working with cross-language bindings easier.

- Allow cross-language bindings to expose struct fields without compatibility hazards, and
- Make all code that disables default destructor behavior more convenient to use in Rust.

**Needs contributor:** This goal needs a contributor to implement a lang experiment in rustc and write an RFC. The work is a focused compiler feature touching drop semantics and destructor codegen. Estimated time commitment: TBD.

## Motivation

### The status quo

Cross-language bindings for structs with destructors requires using `ManuallyDrop` today. For example, this struct in C++:

```c++
struct RingState {
    Uring ring;
    std::array<UringBuf, 16> buffers;
};
```

Must be bound in Rust like so:

```rust
#[repr(C)]
struct UringState {
    ring: ManuallyDrop<Uring>,
    buffers: ManuallyDrop<[UringBuf; 16]>,
}
impl Drop for UringState { /* call C++ destructor */ }
```

Otherwise, two things would happen:

1. The field destructors would be called in the opposite order from the source language. Most systems languages with destructors like C++, ObjC, and Swift drop their fields in reverse order of declaration. This is reversed in Rust. In our example there is a logical dependence between the ring and the buffers, and dropping them in the wrong order leads to errors.
2. It's technically undefined behavior, regardless of drop order. To destroy an object in C++, you need to call that type's destructor instead of recursing into the fields manually.

This is unergonomic to use and prevents normal construction:

```rust
let ring = Uring::new();
let buffers = register_buffers(&ring);

UringState {
    ring: ManuallyDrop::new(ring),
    buffers: ManuallyDrop::new(buffers),
}
// ...vs...
UringState { ring, buffers }
```

With two fields it isn't so bad, but the cost scales as the struct becomes larger.

One approach to easing this pain is to only use ManuallyDrop when binding fields whose types have destructors. But that presents a compatibility hazard: It would mean adding or removing a destructor from that C++ type becomes a breaking change to Rust code, when it is usually not a breaking change to C++ code.

### What we propose to do about it

#### Proposal 1: Add a `#[manually_drop]` attribute

Add an attribute `#[manually_drop]` to the language that disables drop glue on a struct's fields.

This allows us to correctly bind the `UringState` struct while exposing its fields safely:

```rust
#[repr(C)]
struct UringState {
    #[manually_drop]
    ring: Uring,
    #[manually_drop]
    buffers: [UringBuf; 16],
}

impl Drop for UringState {
    fn drop(&mut self) {
        // call C++ destructor
    }
}
```

#### Proposal 2: Add a `drop_in_place` method to the `Drop` trait

Add a `drop_in_place` method to the `Destruct` trait, which is called by the compiler instead of the normal drop glue when a type is dropped.

1. If the type has implemented `drop_in_place`, it is called instead of the normal drop glue. No other code is run on drop, in particular this does not recurse into the fields.

```rust
struct UringState {
    ring: Uring,
    buffers: [UringBuf; 16],
}

impl Drop for UringState {
    /// Does the full dropping of the value.
    /// If not overridden by the user, this is compiler-
    /// generated; the default wil call `Self::drop` then drop the fields.
    /// Use this to control the drop order of the fields, or emulate `ManuallyDrop`.
    unsafe fn drop_in_place(&mut self) {
        // call C++ destructor
    }
}
```

2. If the type does not override `drop_in_place`, we emit the drop glue as usual, which calls `Drop::drop` on the type (if implemented) then recursively drops the fields.

### Work items over the next year

| Task                        | Owner(s) | Notes                                                         |
| --------------------------- | -------- | ------------------------------------------------------------- |
| Implement a lang experiment |          | @thunderseethe to find an owner. @tmandry can act as champion |
| Write an RFC                |          | @thunderseethe to find an owner                               |

## Team asks

| Team       | Support level | Notes                     |
|------------|---------------|---------------------------|
| [compiler] | Medium        | Implementation reviews    |
| [lang]     | Medium        | Vibe check and RFC review |
| [opsem]    | Small         |                           |
| [types]    | Small         |                           |

## Frequently asked questions

### Why not a `#[reverse_drop_order]` attribute instead?

The original destructor of the type still needs to be called. In C++ that destructor in turn calls the destructors of every field, so dropping the fields from the Rust side would lead to double free.

If we had `#[reverse_drop_order]` it might be usable for structs which themselves don't define a destructor (only their fields have destructors), but technically it would still be UB to bypass the destructor defined by C++.
