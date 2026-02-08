# Standardizing Unsafe Code Usage in the Rust Standard Library

| Metadata         |                                                                                  |
| :--------------- | -------------------------------------------------------------------------------- |
| Point of contact | @hxuhack                                   |
| Status           | Proposed                                                                         |
| Tracking issue   |      |
| Zulip channel    | N/A  |

## Summary

To establish explicit conventions for unsafe code usage and documentation by reviewing existing practices in the Rust standard library, and to audit the library against these conventions to identify and correct inconsistencies.

## Motivation

### The status quo

The Rust standard library does not currently provide systematic guidelines for unsafe code usage and documentation. 
While existing [Unsafe Code Guidelines](https://rust-lang.github.io/unsafe-code-guidelines/glossary.html#soundness-of-code--of-a-library) establishes an essential baseline, it is not sufficiently detailed to guide the practical adoption of unsafe code, particularly with respect to visibility-related soundness choices and the documentation of safety invariants.

In practice, the opsem team and standard library team have developed a shared understanding and informal conventions about unsafe code usage through ongoing development. 
These conventions are important, but they have not yet been systematically documented.
As the standard library continues to grow in size and complexity, documenting these conventions becomes increasingly important for guiding contributors in writing and reviewing unsafe code correctly.

In addition, Rust for Linux has a similar requirement and has already drafted a [document](https://lore.kernel.org/rust-for-linux/20240717221133.459589-1-benno.lossin@proton.me/) to guide unsafe code usage in the project.

### What we propose to do about it

- Document the conventions about unsafe code usage as a Rust Safety Standard in collaboration with the opsem team and the standard library team, and iteratively refine the standard based on feedback and experience (see a draft [here](https://internals.rust-lang.org/t/pre-rfc-rust-safety-standard/23963)). The standard focuses on the following two aspects:

  * **Code safety criteria**: criteria for marking functions or other program components as safe or unsafe.
  
  * **Safety documentation**: the information that should be provided at unsafe code definition sites, use sites, and struct definition sites where type invariants are required.

- Review unsafe code usage in the Rust standard library as well as Rust for Linux, and identify and correct inconsistencies with the standard.


### Work items over the next year

| Task        | Owner(s) | Notes |
| ----------- | -------- | ----- |
| Draft and refine the safety standard  | @hxuhack |       |
| Audit standard library | @hxuhack, @DiuDiu777 |       |
| Audit Rust for Linux | @hxuhack, @yilin0518   |       |

### The "shiny future"

- The standard can also be adopted by downstream Rust crates, helping to promote consistent and sound unsafe code practices across the ecosystem.
- It can serve as a basis for future attribute-based safety documentation and checking, as proposed in [RFC 3842](https://github.com/rust-lang/rfcs/pull/3842). 

## Team asks

| Team       | Support level | Notes                                   |
| ---------- | ------------- | --------------------------------------- |
| [libs]     | Small         | Review the standard document and provide feedback, examine any issues identified, and review related pull requests |
| [opsem]    | Small         | Review the standard document and provide feedback, examine any issues identified, and review related pull requests |
