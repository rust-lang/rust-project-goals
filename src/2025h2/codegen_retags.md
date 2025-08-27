# Emit Retags in Codegen

| Metadata         |                                                                                  |
|:-----------------|----------------------------------------------------------------------------------|
| Point of contact | @icmccorm                                                                        |
| Status           | Proposed                                                                         |
| Tracking issue   |                                                                                  |
| Zulip channel    | N/A                                                                              |
| [compiler] champion | @RalfJung |
| [lang] champion | @tmandry |
| [opsem] champion | @RalfJung |

## Summary
Allow codegen backends to implement the MIR [`Retag`](https://doc.rust-lang.org/std/intrinsics/mir/fn.Retag.html) intrinsic, and add a similar intrinsic to the LLVM backend. 

## Motivation

Miri uses Rust's [`Retag`](https://doc.rust-lang.org/std/intrinsics/mir/fn.Retag.html) intrinsics to find aliasing bugs, but they are only available in the MIR. This limits the category of tools that can use this information to find Rust-specific forms of undefined behavior. We propose making these intrinsics available to codegen backends so that it can be possible to find these bugs using native instrumentation. We aim at having both an RFC and experimental nightly implementation in this period.

### The status quo

Miri has a unique role for the Rust community as the only tool that can find violations of Rust's evolving aliasing models. However, Miri is significantly slower than native execution, and it does not support finding aliasing violations triggered by foreign function calls. Lack of FFI support has prevented developers from finding aliasing bugs in real-world libraries—including one maintained by the Rust project ([`flate2-rs`](https://github.com/rust-lang/flate2-rs/issues/392))! We need a new approach for finding these bugs in the large-scale, multi-language projects where Rust is being adopted. 

We could provide both better performance and support for multilanguage applications by inserting native run-time checks into shared formats like LLVM IR. These checks could be implemented in several different ways, possibly as [an extension to Valgrind](https://github.com/pnkfelix/krabcake) or [a new LLVM sanitizer](https://borrowsanitizer.com/). However, before we can fully prototype any of these approaches, we need a way to take the type information that Miri uses to find aliasing violations and lower it into Rust's codegen backends, where it can be used to determine where and how to insert these run-time checks.

Luckily, there's already a mechanism that we can extend to provide everything that we need to support these tools. Miri configures the compiler to emit [`Retag`](https://doc.rust-lang.org/std/intrinsics/mir/fn.Retag.html) intrinsics in the MIR. These are used to update the permissions associated with pointers under each aliasing model (see [Tree Borrows](https://iris-project.org/pdfs/2025-pldi-treeborrows.pdf) for a deeper dive into the role of a retag). However, at the moment, retags are discarded once they reach codegen.

### The next 6 months

We are creating a proof-of-concept extension that allows codegen backends to implement custom behavior for retags. Our changes include a new `@llvm.retag` intrinsic within the LLVM backend. Dynamic instrumentation tools can replace this intrinsic with concrete run-time checks. We plan on taking our current implementation and refinining it over this development period with feedback from the compiler and opsem teams.

This is not quite as simple as emitting an LLVM intrinsic for each MIR intrinsic, though. Miri executes some retags implicitly as side-effects of interpreting other instructions, and it determines the effect of a retag at run-time using type information that we will not have access to at the LLVM level. We need to emit all of these implicit retags ahead of time, along with the permissions that they carry under the aliasing model that's being targeted.
 
### The "shiny future" we are working towards

These changes will be necessary to support BorrowSanitizer: an out-of-tree LLVM instrumentation tool for finding violations of Rust's aliasing model. BorrowSanitizer will be capable of finding aliasing violations, as well as other classic memory safety issues. Our tool is not fully functional yet, but we aspire for it to become a production-ready solution for developers who are integrating Rust components into large-scale C++ applications. You can read more about our goals at [our website](https://borrowsanitizer.com) or in [the abstract](https://borrowsanitizer.com/pdfs/rw2025.pdf) that we submitted for the 2025 Rust Verification Workshop. 

However, beyond BorrowSanitizer, we believe that the these changes will be necessary to support any kind of tool for finding aliasing bugs with native instrumentation. Model-checkers for Rust (see [SEABMC](https://arxiv.org/abs/2408.04043v3)) will also benefit from having additional ownership information from the frontend.

## Design axioms

### Avoid targeting any particular aliasing model

Rust's aliasing model is still evolving, and both Stacked and Tree Borrows require different types of retags in different places (e.g. Stacked Borrows retags raw pointers after they are cast from references, but Tree Borrows does not). @RalfJung [has indicated](https://www.reddit.com/r/rust/comments/1lv9y96/comment/n253qdu/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
) that there will still be significant changes coming—potentially as a part of a third aliasing model. In anticipation of these changes, we need make these backend retags configurable to support all of the features of Stacked and Tree Borrows. To make this easy to maintain, we should allow third-party compiler plugins to override the behavior of a retag at this level, so that tools can adapt quickly to changes in Rust's aliasing model while it remains unstable.

### Make changes as minimal as possible

We should avoid making changes that substantially impact any component of the compiler or Miri. We would benefit from having guidance from the Miri team, but we do not think that it will be necessary to make any changes to Miri.

## Ownership and team asks
| Task                           | Owner(s) or team(s) | Notes                                                                |
|--------------------------------|---------------------|-----------------------|
| Implementation                 | @icmccorm           |   [proof-of-concept](https://github.com/Borrowsanitizer/rust)        |
| Author RFC                     | @icmccorm           |      |
| Design meeting                 | ![Team][] [opsem], [compiler]    |  |
| Standard reviews | ![Team][] [opsem], [compiler] |     |
| Dedicated reviewer                 | ![Team][]  [opsem], [compiler] | Most of our changes are within `rustc_codegen_ssa`, but it would also be helpful to have feedback from someone familiar with how retags are handled within Miri's [`borrow_tracker`](https://doc.rust-lang.org/nightly/nightly-rustc/miri/borrow_tracker/index.html) module. |
| RFC decision                   | ![Team][] [opsem], [compiler]   |                       |
| Discussion and moral support   | ![Team][] [opsem], [compiler]    |                       |

### Definitions

* *Discussion and moral support* is the lowest level offering, basically committing the team to nothing but good vibes and general support for this endeavor.
* *Author RFC* and *Implementation* means actually writing the code, document, whatever.
* *Design meeting* means holding a synchronous meeting to review a proposal and provide feedback (no decision expected).
* *RFC decisions* means reviewing an RFC and deciding whether to accept.
* *Standard reviews* refers to reviews for PRs against the repository; these PRs are not expected to be unduly large or complicated.
* *Dedicated review* means identifying an individual (or group of individuals) who will review the changes, as they're expected to require significant context.

## Frequently asked questions

None yet.
