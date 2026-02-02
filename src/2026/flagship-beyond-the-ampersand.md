# Beyond the `&`

## Summary

Smart pointers that feel as natural as `&` and `&mut`.

## Motivation

### The status quo

One of Rust's core value propositions is that it's a "library-based language"—libraries can build abstractions that feel built-in to the language even when they're not. Smart pointer types like `Rc` and `Arc` are prime examples, implemented purely in the standard library yet feeling like native language features.

However, Rust's built-in reference types (`&T` and `&mut T`) have special capabilities that user-defined smart pointers cannot replicate:

* **Reborrowing**: When you pass a `&mut T` to a function, Rust automatically reborrows it so you can use it again afterward. But `Option<&mut T>` or custom smart pointers require awkward `.as_deref_mut()` or `.reborrow()` calls—and values derived from these reborrows cannot be returned from the function.
* **Field projection**: Writing `&x.field` gives you `&Field`—the borrow checker tracks that you're only borrowing part of the struct. Custom pointer types like `NonNull<T>` or `ArcRef<T>` have no equivalent; you can't write `arc_ref.field` and get back an `ArcRef<Field>`.
* **In-place initialization**: Creating a value directly at its final memory location—crucial for unmovable types and large structs—requires complex macro-based solutions because the language has no native support.

This creates a "second-class citizen" problem: custom pointer types can never provide the same ergonomic experience as built-in references, limiting Rust's promise of zero-cost abstractions.

### What we are shooting for

We want user-defined smart pointers to be truly indistinguishable from built-in references in terms of syntax and ergonomics. When you use an `ArcRef<T>` or `Pin<&mut T>`, it should feel just like using `&T` or `&mut T`:

* **Automatic reborrowing** for any pointer type that opts in, with derived values returnable from functions
* **Field projection** that lets you write `ptr.field` and get back a pointer to the field, with full borrow-checker integration
* **Language-level in-place initialization** that's more ergonomic than what macros can achieve

### Key use cases

* **Rust for Linux**: The kernel uses custom smart pointer types (`Arc`, `ArcBorrow`, `Ref`) to express ownership and borrowing patterns that don't map directly to Rust's built-in references. Field projection and reborrowing would make these types dramatically more ergonomic.
* **Cross-language interop**: FFI wrappers like `PyRef<T>` (Python) or `CppMutRef<T>` (C++) represent references into foreign runtimes. These should feel as natural as native Rust references when accessing fields or passing to functions.
* **`MaybeUninit` initialization**: Safely initializing structs field-by-field through `&mut MaybeUninit<T>` is currently verbose and error-prone. Field projection would let you write `uninit.field` and get `&mut MaybeUninit<Field>`.
* **Interior mutability**: Types like `cell::Ref<T>` and `RefMut<T>` wrap borrowed data but lose the ability to project into fields. You can't write `ref_mut.field` to get a `RefMut<Field>`.
* **Volatile and atomic access**: Low-level code working with memory-mapped I/O needs pointer types like `VolatilePtr<T>` that preserve access semantics through field projection.

### Design axioms

* **Generalize and extend the basic operations of Rust.** Build on existing models of how the language works while relaxing restrictions. Fix expressiveness gaps in the base paradigm, rather than inventing new paradigms altogether.
* **Enable abstractions that are powerful, transparent, and ergonomic to use.** These abstractions should allow code to be clearer in its purpose and behavior than it is without them.
* **Ship iteratively.** Ship building blocks that unblock existing use cases and new experimentation. Don't block on full generality and ease of use when that can be done in the future.
* **Design holistically.** Look for ways to accommodate further generality in our designs, even if we don't expose that generality at first.

## 2026 goals

(((FLAGSHIP GOALS: Beyond the `&`)))

## Frequently asked questions

### How do these goals relate to each other?

The goals are complementary but largely independent:

* **Reborrow traits** address the "automatic reborrowing" problem, providing the general mechanism that types can opt into.
* **Field projections** provides the broader "place operations" framework that encompasses projection, borrowing, and reading/writing through custom pointer types.
* **In-place initialization** tackles a related but distinct problem: creating values that can't be moved.

### What types will benefit from this work?

A non-exhaustive list: `NonNull<T>`, `*const T` and `*mut T`, `&mut MaybeUninit<T>`, `cell::Ref` and `RefMut`, `ArcRef<T>`, FFI pointer wrappers like `PyRef<T>` or `CppMutRef<T>`, volatile pointers, and many more.

### Is this the same as the "custom receivers" feature?

Related but distinct. Custom receivers (RFC 3519) allow methods with `self: MyPtr<Self>`. The "Beyond the `&`" work goes further: making those custom pointer types *feel* like native references through automatic reborrowing, field access, and borrow checker integration.
