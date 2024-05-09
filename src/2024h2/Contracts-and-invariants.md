# Contracts and Invariants

| Metadata | |
| --- | --- |
| Owner(s) | pnkfelix |
| Teams | Lang, Libs, Compiler |
| Status | WIP |

## Motivation

We wish to extend Rust in three ways:
First, we want to extend the Rust language to enable Rust developers to write predicates, called "contracts", and attach them to specific points in their program. We intend for this feature to be available to all Rust code, including that of the standard library.
Second, we want to extend the Rust crate format such that the contracts for a crate can be embedded and then later extracted by third-party tools.
Third, we want to extend project-supported Rust compiler and interpreter tools, such as `rustc` and `miri`, to compile the code in a mode that dynamically checks the associated contracts (note that such dynamic checking might be forced to be incomplete for some contracts).
Examples of contracts we envision include: pre- and post-conditions on Rust methods; representation invariants for abstract data types; and loop invariants on `for` and `while` loops. 

Our motivation for this is that contracts are key for reasoning about software correctness. Formal verification tools such as [Creusot][], [Kani][], and [Verus][] have demonstrated that it is possible to write Rust code that is coupled with automated verification mechanisms. But furthermore, we assert that contracts can provide value even if we restrict our attention to dynamic checking: By having a dedicated construct for writing method specifications and invariants formally, we will give our tools new avenues for testing our programs in an automated fashion, similar to the benefits provdied by fuzzing.

[Creusot]: https://github.com/creusot-rs/creusot?tab=readme-ov-file#about
[Kani]: https://model-checking.github.io/kani/
[Verus]: https://verus-lang.github.io/verus/guide/

### The status quo

Currently, if you want to specify the behavior of a Rust method and check that the specification is correct, you can either attempt construct a test suite that covers the entirety of your specification, or you can manually embed contract-like predicates into your code. Embedding contract-like predicates is typically done via variations of either 1. `assert!`, 2. `debug_assert!`, or 3. similar `cfg`-guarded code sequences that abort/panic when a predicate fails to hold.

All of the existing options are limited.

First, most specifications rely on quantified forms, such as "for all X, P(X) implies Q(X)." The "for all" quantifier needs some language support to be expressed; it cannot be written as executable Rust code (except as a loop over the domain, which is potentially infinite).

Second, directly expressing contracts as an assertion mixes it in with the rest of the code, which makes it difficult or impossible for third-party verification tools to extract the contracts in order to reason about them.

As an example for why a tool might want to extract the contracts, the Kani model checker works by translating a whole program (including its calls to library code) into a form that is passed to an off-the-shelf model checker. Kani would like to use contracts as a way to divide-and-conquer the verification effort. The API for a method is abstracted by its associated contract. Instead of reasoning about the whole program, it now has two subproblems: Prove that the method on its own satisfies its associated contract, and in the rest of the program, replace calls to that method by the range of behaviors permitted by the contract.

Third: the Racket language [has demonstrated][findler-felleisen] that when you have dynamic dispatch (via higher-order functions or OOP), then assertions embedded in procedure bodies are a subpar way of expressing specifications. This is because when you compose software components, it is non-trivial to take an isolated assertion failure and map it to which module was actually *at fault*. Having a separate contract language might enable new tools to record enough metadata to do proper "blame tracking." But to get there, we first have to have a way to write contracts down in the first place.

[findler-felleisen]: https://www2.ccs.neu.edu/racket/pubs/icfp2002-ff.pdf

### The next few steps

1. Develop a contract predicate sublanguage, suitable for interpretation by rustc, miri, and third-party verification tools.

2. Extend Rust compiler to enable contract predicates to be attached to items and embedded in Rust crate rlib files.

3. Work with wg-formal-methods (aka the "Rust Formal Methods Interest Group") to ensure that the embedded predicates are compatible with their tools.

4. Work with Lang and Libs team on acceptable surface-level syntax for contracts. In particular, we want contracts to be *used* by the Rust standard library. (At the very least, for method pre- and post-conditions; I can readily imagine, however, also attaching contracts to `unsafe { ... }` blocks.)

5. Extend miri to evaluate contract predicates. Add primitives for querying memory model to contract language, to enable contracts that talk about provenance of pointers.


### The "shiny future" we are working towards

My shiny future is that the people "naturally" write Rust crates that can be combined with distinct dynamic-validation and verification tools. Today, if you want to use any verification tool, you usually have to pick one and orient your whole code base around using it. (E.g., the third-party verification tools often have their own (rewrite of a subset of the) Rust standard library, if only so that they can provide the contracts that our standard library is missing.)

## Design axioms

*Add your [design axioms][da] here. Design axioms clarify the constraints and tradeoffs you will use as you do your design work. These are most important for project goals where the route to the solution has significant ambiguity (e.g., designing a language feature or an API), as they communicate to your reader how you plan to approach the problem. If this goal is more aimed at implementation, then design axioms are less important. [Read more about design axioms][da].*

[da]: ../about/design_axioms.md

Felix [presented these axioms][rw2024] as "Shared Values" at the 2024 Rust Verification Workshop.

[rw2024]: http://pnkfx.org/presentations/contracts2-rw2024.pdf

### 1. Purpose: Specification Mechanism first, Verification Mechanism second

Contracts have proven useful under the "Design by Contract" philosophy, which usually focuses on pre + post + frame conditions (also known as "requires", "ensures", and "modifies" clauses in systems such as the Java Modelling Language). In other words, attaching predicates to the API boundaries of code, which makes contracts a *specification mechanism*.

There are other potential uses for attaching predicates to points in the code, largely for encoding formal correcttness arguments. My main examples of these are Representation Invariants, Loop Invariants, and Termination Measures (aka "decreasing functions").

In an ideal world, contracts would be useful for both purposes. But if we have to make choices about what to prioritize, we should focus on the things that make contracts useful as an API specifcation mechanism. In my opinion, API specification is the use-case that is going to benefit the broadest set of developers.

### 2. Contracts should be (semi-)useful out-of-the-box

This has two parts: 

Anyone can eat: I want any Rust developer to be able to turn on "contract checking" in some form without having to change toolchain nor install 3rd-party tool, and get *some* utility from the result. (Its entirely possible that contracts become *even more* useful when used in concert with a suitable 3rd-party tool; that's a separate matter.)

Anyone can cook: Any Rust developer can also *add contracts* to their own code, without having to change their toolchain.

### 3. Contracts are not just assertions

Contracts are meant to enable modular reasoning.

Contracts have both a dynamic semantics and a static semantics.

In an ideal dynamic semantics, a broken contract will identify *which component* is at fault for breaking the contract. (We do acknowledge that precise blame assignment becomes non-trivial with dynamic dispatch.)

In an ideal static semantics, contracts enable theorem provers to choose, instead of reasoining about `F(G)`, to instead allow independent correctnes proofs for `F(...)` and ``... G ...`.

### 4. Balance accessibility over power

For accessibilty to the developer community, Rust contracts should strive for a syntax that is, or closely matches, the syntax of Rust code itself. Deviations from existing syntax *or semantics* must meet a high bar to be accepted for contract language.

But some deviations should be possible, if justified by their necessity for correct expression of specifications. Contracts may *need* forms that are not valid Rust code. For example, for-all quantifiers will presumably need to be supported, and will likely have a dynamic semantics that is necessarily incomplete compared to an idealized static semantics used by a verification tool. (Note that middle grounds exist here, such as adding a `forall(|x: Type| { pred(x) })` intrinsic that is fine from a syntax point of view and is only troublesome in terms of what semantics to assign to it.)

Some expressive forms might be intentionally unavailable to normal object code. For example, some contracts may want to query provenance information on pointers, which would make such contracts unevaluatable in `rustc` object code (and then one would be expected to use `miri` or similar tools to get checking of such contracts).

### 5. Accept incompleteness

Not all properties of interest can be checked/6 at runtime; similarly, not all statements can be proven true or false.

Full functional correctness specfifications are often not economically feasiable to develop and maintain.

We must accept limitations on both dynamic validation and static verification strategies, and must choose our approximations accordingly.

An impoverished contract system may still be useful for specifying a coarser range of properties (such as invariant maintenance, memory safety, panic-freedom).

### 6. Embrace tool diversity

Different static verification systems require or support differing levels of expresiveness. And the same is true for dynamic validation tools! (E.g. consider injecting assertions into code via `rustc`, vs interpreters like `miri` or binary instrumentation via `valgrind`).

An ideal contract system needs to deal with this diversity in some manner. For example, we may need to allow third-party tools to swap in different contracts (and then also have to meet some added proof obligation to justify the swap).

### 7. Verification cannot be bolted on, ... but validation can

In general, code must be written with verification in mind as one of its design criteria.

We cannot expect to add contracts to arbitrary code and be able to get it to pass a static verifier.

This does not imply that contracts must be useless for arbitrary code. Dynamic contract checks have proven useful for the Racket community.

Racket development style: add more contracts to the code when debugging (including, but not limited to, contract failures)

A validation mechanism can be bolted-on after the fact.


## Ownership and other resources

**Owner:** pnkfelix

pnkfelix has been working on the Rust compiler since before Rust 1.0; he was co-lead of the Rust compiler team from 2019 until 2023. pnkfelix has taken on this work as part of a broader interest in enforcing safety and correctness properties for Rust code.

celinval is also assisting. celinval is part of the Amazon team producing the Kani model checker for Rust. The Kani team has been using contracts as an unstable feature in order to enable *modular verification*; you can read more details about it on [Kani's blog post](https://model-checking.github.io/kani-verifier-blog/2024/01/29/function-contracts.html).

### Support needed from the project

* Compiler: We expect to be authoring three kinds of code: 1. unstable surface syntax for expressing contract forms, 2. new compiler intrinsics that do contract checks, and 3. extensions to the rlib format to allow embedding of contracts in a readily extractable manner. We expect that we can acquire review capacity via AWS-employed members of compiler-contributors; the main issue is ensuring that the compiler team (and project as a whole) is on board for the extensions as envisioned.

* Libs-impl: We will need libs-impl team engagement to ensure we design a contract language that the standard library implementors are willing to use. To put it simply: If we land support for contracts without uptake within the Rust standard library, then the effort will have failed.

* Lang: We need approval for a lang team experiment to design the contract surface language. However, we do not expect this surface language to be stabilized in 2024, and therefore the language team involvement can be restricted to "whomever is interested in the effort."


* WG-formal-methdos: We need engagement with the formal-methods community to ensure our contract system is serving their needs.

* Stable-MIR: if we deliver a contract system that leverages Stable-MIR, it may serve as a useful "carrot" to encourage 3rd party tools to invest in adopting Stable-MIR for their own tools.

* Miri: We would like assistance from the miri developers on the right way to extend miri to have configurable contract-checking (i.e. to equip `miri` with enhanced contract checking that is not available in normal object code).

## Outputs and milestones

### Outputs

Rust standard library ships with contracts in the rlib (but not built into the default object code).

Rustc has unstable support for embedding dynamic contract-checks into Rust object code.

Some dynamic tool (e.g. miri or valgrind) that can dynamically check contracts whose bodies are *not* embedded into the object code.

Some static verification tool (e.g. Kani) leverages contracts shipped with Rust standard library.


### Milestones

Unstable syntactic support for contracts in Rust programs (at API boundaries at bare minimum, but hopefully also at other natural points in a code base.)

Support for extracting contracts for a given item from an rlib.


## Frequently asked questions

### Q: How do contracts help static verification tools?

Answer: Once we have a contract language built into rustc, we can include its expressions as part of the compilation pipeline, turning them into HIR, THIR, MIR, et cetera.

For example, we could add contract-specific intrinsics that map to new MIR instructions. Then tools can decide to interpret those instructions. rustc, on its own, can decide whether it wants to map them to LLVM, or into valgrind calls, et cetera.
(Or compiler could throw them away; but: unused = untested = unmaintained) 

### Q: Why do you differentiate between the semantics for dynamic validation vs static verification?

See next question for an answer to this.

### Q: How are you planning to dynamically check arbitrary contracts?

A dynamic check of the construct `forall(|x:T| { … })` sounds problematic for most T of interest


pnkfelix'x expectation here is that we would *not* actually expect to support `forall(|x:T| ...)` in a dynamic context, not in the general case of arbitrary `T`.

pnkfelix's current favorite solution for cracking this nut: a new form, `forall!(|x:T| suchas: [x_expr1, x_expr2, …] { … })`,
where the semantics is "this is saying that the predicate must hold for all T, but in particular, we are hinting to the dynamic semantics that it can draw from the given sample population denoted by `x_expr1`, `x_expr2`, etc.

Static tools can ignore the sample population, while dynamic tools can use the sample population directly, or feed them into a fuzzer, etc.

### Q: Doesn't formal specifications need stuff like unbounded arithmetic?

Some specifications benefit from using constructs like unbounded integers, or sequences, or sets. (Especially important for devising abstraction functions/relations to describe the meaning of a given type.)

Is this in conflict with “Balance accessibility over power”?

pnkfelix sees two main problems here: 1. Dynamic interpretation may incur unacceptably high overhead. 2. Freely copying terms (i.e. ignoring ownership) is sometimes useful.

Maybe the answer is that some contract forms *simply cannot* be interpreted via the Rust abstract machine. And to be clear: That is **not** a failure! If some forms can only be checked in the context of `miri` or some other third-party tool, so be it.

### Q: What about unsafe code?

pnkfelix does not know the complete answer here.

Some dynamic checks would benefit from access to memory model internals. 

But in general, checking the correctness of an unsafe abstraction needs type-specific ghost state (to model permissions, etc). We are leaving this for future work, it may or may not get resolved this year.




