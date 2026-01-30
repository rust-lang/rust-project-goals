# Experiment and RFC for `#[manually_drop]`

| Metadata         |                         |
| :--------------- | ----------------------- |
| Point of contact | @tmandry                |
| Status           | Proposed for mentorship |
| Tracking issue   |                         |
| Zulip channel    | N/A                     |

## Summary

Add a `#[manually_drop]` attribute to

* Allow cross-language bindings to expose struct fields without compatibility hazards, and  
* Make all code that disables default destructor behavior more convenient to use in Rust.

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

### Work items over the next year

| Task                        | Owner(s) | Notes                     |
| --------------------------- | -------- | ------------------------- |
| Implement a lang experiment |          | @tmandry to find an owner |
| Write an RFC                |          | @tmandry to find an owner |

## Team asks

| Team       | Support level | Notes                     |
| ---------- | ------------- | ------------------------- |
| [compiler] | Small         | Implementation reviews    |
| [lang]     | Medium        | Vibe check and RFC review |
| [opsem]    | Vibes         |                           |
| [types]    | Vibes         |                           |

## Frequently asked questions

### Why not a `#[reverse_drop_order]` attribute instead?

The original destructor of the type still needs to be called. In C++ that destructor in turn calls the destructors of every field, so dropping the fields from the Rust side would lead to double free.

If we had `#[reverse_drop_order]` it might be usable for structs which themselves don't define a destructor (only their fields have destructors), but technically it would still be UB to bypass the destructor defined by C++.
