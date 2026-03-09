# Beyond the `&`

| Metadata         |                                                                                                                                  |
|:-----------------|----------------------------------------------------------------------------------------------------------------------------------|
| Short title      | Beyond the `&`                                                                                                                   |
| What and why     | Smart pointers (`Arc`, `Pin`, FFI wrappers) get the same ergonomics as `&` and `&mut` — reborrowing, field access, in-place init |
| Point of contact | @tmandry                                                                                                                         |

## Summary

Smart pointers that feel as natural as `&` and `&mut`.

## Motivation

### The status quo

One of Rust's core value propositions is that it's a "library-based language"—libraries can build abstractions that feel built-in to the language even when they're not. Smart pointer types like `Rc` and `Arc` are prime examples, implemented purely in the standard library yet feeling like native language features.

However, Rust's built-in reference types (`&T` and `&mut T`) have special capabilities that user-defined smart pointers cannot replicate:

* **Reborrowing**: When you pass a `&mut T` to a function, Rust automatically reborrows it so you can use it again afterward. But `Option<&mut T>` or custom smart pointers require awkward `.as_deref_mut()` or `.reborrow()` calls—and values derived from these reborrows cannot be returned from the function.
* **Field projection**: Writing `&x.field` gives you `&Field`—the borrow checker tracks that you're only borrowing part of the struct. Custom pointer types like `NonNull<T>` or `ArcRef<T>` have no equivalent; you can't write `arc_ref.field` and get back an `ArcRef<Field>`.
* **In-place initialization**: Creating a value directly at its final memory location—crucial for unmovable types and large structs—requires complex macro-based solutions because the language has no native support.

This creates a "second-class citizen" problem: custom pointer types can never provide the same ergonomic experience as built-in references, limiting Rust's promise of zero-cost abstractions. The impact is felt across many domains: the Linux kernel uses custom smart pointer types (`Arc`, `ArcBorrow`, `Ref`) that would benefit dramatically from field projection and reborrowing; FFI wrappers like `PyRef<T>` or `CppMutRef<T>` should feel as natural as native references; types like `cell::Ref<T>` and `RefMut<T>` lose the ability to project into fields; and safely initializing structs field-by-field through `&mut MaybeUninit<T>` remains verbose and error-prone.

### Design axioms

* **Generalize and extend the basic operations of Rust.** Build on existing models of how the language works while relaxing restrictions. Fix expressiveness gaps in the base paradigm, rather than inventing new paradigms altogether.
* **Enable abstractions that are powerful, transparent, and ergonomic to use.** These abstractions should allow code to be clearer in its purpose and behavior than it is without them.
* **Ship iteratively.** Ship building blocks that unblock existing use cases and new experimentation. Don't block on full generality and ease of use when that can be done in the future.
* **Design holistically.** Look for ways to accommodate further generality in our designs, even if we don't expose that generality at first.

### What we are shooting for

User-defined smart pointers that are truly indistinguishable from built-in references in terms of syntax and ergonomics — automatic reborrowing for any pointer type that opts in, field projection with full borrow-checker integration, and language-level in-place initialization.

### How we get there

(((ROADMAP GOALS: Beyond the `&`)))

The goals are complementary but largely independent. **Reborrow traits** address automatic reborrowing, providing the general mechanism that types can opt into. **Field projections** provides the broader "place operations" framework that encompasses projection, borrowing, and reading/writing through custom pointer types. **In-place initialization** tackles a related but distinct problem: creating values that can't be moved.

## Frequently asked questions

### What types will benefit from this work?

A non-exhaustive list: `NonNull<T>`, `*const T` and `*mut T`, `&mut MaybeUninit<T>`, `cell::Ref` and `RefMut`, `ArcRef<T>`, FFI pointer wrappers like `PyRef<T>` or `CppMutRef<T>`, volatile pointers, and many more.

### Is this the same as the "custom receivers" feature?

Related but distinct. Custom receivers (RFC 3519) allow methods with `self: MyPtr<Self>`. The "Beyond the `&`" work goes further: making those custom pointer types *feel* like native references through automatic reborrowing, field access, and borrow checker integration.
