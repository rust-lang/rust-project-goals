# Improving Unsafe Code Documentation in the Rust Standard Library

| Metadata         |                                                                                  |
| :--------------- | -------------------------------------------------------------------------------- |
| Point of contact | @hxuhack                                   |
| Status           | Proposed                                                                         |
| Tracking issue   |      |
| Zulip channel    | N/A  |

## Summary

Review unsafe code documentation in the Rust standard library, identify inconsistencies, and fix them. Based on the lessons learned during this review process, we also aim to establish explicit conventions for code safety reasoning and unsafe code documentation.

## Motivation

### The status quo

The Rust standard library provides many unsafe APIs accompanied by safety documentation. 
While most unsafe APIs include safety sections, the quality of these documents does not always match the rigor of the code itself. 
Inconsistencies exist, such as missing safety sections (e.g., [#138309](https://github.com/rust-lang/rust/pull/138309), [#146925](https://github.com/rust-lang/rust/pull/146925)) and incomplete safety properties (e.g., [#134953](https://github.com/rust-lang/rust/pull/134953), [#134496](https://github.com/rust-lang/rust/pull/134496), [#135009](https://github.com/rust-lang/rust/pull/135009)). A systematic review is therefore necessary to uphold the quality of this flagship component of Rust.

Additionally, the Rust standard library lacks systematic guidelines for safety reasoning and unsafe code documentation. 
As the library continues to grow in size and complexity, maintaining unsafe code documentation quality becomes increasingly challenging without such guidelines.
Rust for Linux has already drafted a [document](https://lore.kernel.org/rust-for-linux/20240717221133.459589-1-benno.lossin@proton.me/) to guide unsafe code usage in that project. However, the standard is still incomplete and not directly applicable to the Rust standard library.
In practice, the standard library team and the opsem team have developed a shared understanding and informal conventions around unsafe code usage through ongoing development. However, these conventions have not yet been systematically documented.
While reviewing the Rust standard library, we aim to document these conventions, which are increasingly important for guiding contributors in writing and reviewing unsafe code correctly.

Finnaly, such a guideline is also needed by the community. For example, there have recently been several discussion threads about code safety reasoning and documentation, such as [Conditions for unsafe code to rely on correctness](https://internals.rust-lang.org/t/conditions-for-unsafe-code-to-rely-on-correctness/23995) and [Private unsafe fields are a poorly motivated feature](https://internals.rust-lang.org/t/private-unsafe-fields-are-a-poorly-motivated-feature/23976). This suggests that there is ongoing uncertainty and divergence in how people reason about code safety. While existing [Unsafe Code Guidelines](https://rust-lang.github.io/unsafe-code-guidelines/glossary.html#soundness-of-code--of-a-library) establishes an essential baseline, it is not sufficiently detailed to guide the practical adoption of unsafe code, particularly with respect to visibility-related soundness choices and the documentation of safety invariants. We believe that even an experimental standard could help reduce such divergence by providing a clearer and more structured framework for discussion.

### What we propose to do about it

- **Review unsafe code documentation in the Rust standard library, identify and resolve inconsistencies (missing safety sections and missing safety properties), following the approach taken in [#138309](https://github.com/rust-lang/rust/pull/138309), [#146925](https://github.com/rust-lang/rust/pull/146925), [#134953](https://github.com/rust-lang/rust/pull/134953), [#134496](https://github.com/rust-lang/rust/pull/134496), and [#135009](https://github.com/rust-lang/rust/pull/135009).** This effort will require continued collaboration between the opsem team and the standard library team, as in previous reviews.

  The inconsistency checking will be primarily based on the following mechanisms and will not introduce hard semantic-related issues.
  - **i. Missing safety sections:**
  For example, an unsafe API without the safety section in its doc.
  
  - **ii. Cross-checking API naming and parameters:**
  API names and parameters often encode specific safety properties. For instance, the following four APIs (`from_raw(raw: *mut T)`, `from_raw_in(raw: *mut T, alloc: A)`, `from_non_null(ptr: NonNull<T>)`, `from_non_null_in(raw: NonNull<T>, alloc: A)`) of `Box` illustrate a naming convention. The substring `_in` indicates that the pointer must refer to a block of memory allocated by the provided allocator `alloc`. By contrast, APIs without the `_in` suffix imply that the pointer must refer to a block of memory allocated by the global allocator. Such naming patterns also exist in other structs like `Weak` and `Arc`. It is not difficult to detect and confirm inconsistencies among them. See [#135009](https://github.com/rust-lang/rust/pull/135009) and [#135805](https://github.com/rust-lang/rust/pull/135805) for more details.

  - **iii. Inference based on unsafe callees:**
  If an unsafe callee has safety requirements that are not enforced or discharged by the caller, this may indicate that a safety requirement is missing from the caller’s documentation.
  In the following example, `foo` is an unsafe API with two safety requirements. At the callsite in `bar`, only one requirement `valid for reads` is discharged. The other requirement `aligned for u32` must therefore be documented as a safety requirement of the caller.
    ```rust
    /// # Safety:
    /// - `p` must be valid for reads.
    /// - `p` must be properly aligned for `u32`.
    pub unsafe fn foo(p: *const u32) -> u32 {
        let r = p;
        unsafe { *r }
    }
    
    /// # Safety:
    /// - `T` must have size at least 4.
    /// - `T` must not have padding in its first 4 bytes.
    /// - (missing: `x` must be properly aligned for `u32`.)
    pub unsafe fn bar<T>(x: T) {
        let p: *const u32 = &x as *const T as *const u32;
    
        // Safety: 
        // - `p` is valid for reads because it points to `x`.
        unsafe { foo(p as *mut u32); }
    }
    ```
    In a word, all prospective inconsistencies detected via such mechanisms should be straightforward to check and confirm.

- **Formulate an experimental standard based on the lessons learned from the standard library to guide the future development of library features involving unsafe code (see a draft [here](https://internals.rust-lang.org/t/pre-rfc-rust-safety-standard/23963)).** The proposed standard focuses on two aspects:
  * **Code safety reasoning**: principles and criteria for determining when functions or other program components should be marked as safe or unsafe.
  * **Unsafe code documentation**: the information that should be provided at unsafe code definition sites, use sites, and struct definition sites where type invariants are required.

  For example, the community has had a fierce [dicussion](https://internals.rust-lang.org/t/private-unsafe-fields-are-a-poorly-motivated-feature/23976) on whether the function, `new_unchecked`, in the following example, should be declared safe.
  While surveying the Rust standard library, we observe that declaring a function as unsafe means that it may violate certain safety invariants if misused. Therefore, these invariants must be identified and established first. From this, we can summarize the guiding principle: "safety invariants go before unsafe."
  
  In the following example, whether `new_unchecked` should be safe or unsafe depends on whether the type explicitly declares a safety invariant that `x` must be even.
  If such an invariant is part of the type’s contract, then `new_unchecked` must be declared unsafe, since incorrect usage can violate the invariant.
  If such invariant does not exist, then `new_unchecked` is safe, as misuse does not violate any safety invariant.
  Therefore, both versions below are sound, but they correspond to different design choices regarding the type’s invariants.
  
  ```rust
  /// # Safety
  /// ## Struct invariant
  /// - `x` must be even.
  pub struct EvenNumber {
    val: u32,
  }
  
  impl EvenNumber {
      /// # Safety
      /// - `x` must be even.
      pub unsafe fn new_unchecked(x: u32) -> Self {
          EvenNumber{x}
      }
  }
  ```

### Work items over the next year

| Task        | Owner(s) | Notes |
| ----------- | -------- | ----- |
| Audit the Rust standard library | @hxuhack, @DiuDiu777 |       |
| Draft and refine the safety standard  | @hxuhack |       |

### The "shiny future"

- The standard can also be adopted by downstream Rust crates, promoting more consistent and sound unsafe code practices across the ecosystem. A particularly important beneficiary would be Rust for Linux, which places significant emphasis on the correct handling of unsafe code.
- It can serve as a basis for future attribute-based safety documentation and checking, as proposed in [RFC 3842](https://github.com/rust-lang/rfcs/pull/3842). 

## Team asks

| Team       | Support level | Notes                                   |
| ---------- | ------------- | --------------------------------------- |
| [libs]     | Small         | Review pull requests; |
| [opsem]    | Small         | Review pull requests; answer questions on Zulip when there are different openions about specific rules  |

## Frequently asked questions

### What is the difference from goal [486](https://github.com/rust-lang/rust-project-goals/pull/486).

[#486](https://github.com/rust-lang/rust-project-goals/pull/486) will start with Eclipse iceoryx2.
> We’ll apply the zerocopy model systematically, starting with [Eclipse iceoryx2](https://github.com/eclipse-iceoryx/iceoryx2), a zero-copy IPC framework with ~3,300 unsafe usages.

It focuses more on summarizing and documenting practical usage patterns of unsafe code and the specific contracts involved, particularly across different application scenarios. 
> Priority areas include cross-process synchronization, memory-mapped regions, cross-process atomics, and UnsafeCell in shared memory—patterns common in systems programming but underdocumented.

This goal is neither concerned with specific contracts nor with particular application scenarios. It starts from the Rust standard library and summarizes general rules to guide further development of library features involving unsafe code.

