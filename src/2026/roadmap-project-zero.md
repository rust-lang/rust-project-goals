# Project Zero

| Metadata         |                                                                                          |
|:-----------------|------------------------------------------------------------------------------------------|
| Short title      | Project Zero                                                                             |
| What and why     | Fix all known type system unsoundnesses so Rust's safety guarantees are actually reliable |
| Point of contact | @lcnr                                                                                    |

## Summary

Fix every known type system soundness bug in Rust, so that `unsafe` is the only way to cause undefined behavior.

## Motivation

### The status quo

Rust's reputation for reliability rests on its *safety promise*, that users who write "safe Rust" code (to a first approximation, Rust that does not contain the `unsafe` keyword) cannot violate memory safety or trigger undefined behavior. The US government [strongly encourages](https://www.cisa.gov/resources-tools/resources/memory-safe-languages-reducing-vulnerabilities-modern-software-development) the use of memory safe languages like Rust, going so far as to call the use of memory unsafe languages a "threat to national security".

Despite its innocuous sounding name, bugs that trigger "undefined behavior" are among the worst kind of bugs, as they indicate a program that could do *anything*. Exploits based on undefined behavior typically give attackers complete control of the target machine, unlike other forms of vulnerabilities that frequently have more limited exposure. Many of the most infamous exploits in recent times, such as Heartbleed and Stuxnet, were caused by memory safety errors, and studies at Microsoft and elsewhere have found that fully 70% of vulnerabilities are triggered by memory safety errors.

Over the last few years, a number of [known soundness bugs][unsoundnesses] have been found in Rust's type system. These bugs are obscure and have rarely if ever affected production systems, but if they are not fixed, one of them is likely to be the basis for a Rust exploit. This is frightening both because it could mean exposed systems (never good) but it would also do damage to Rust's reputation as a secure language.


[unsoundnesses]: https://github.com/orgs/rust-lang/projects/61/views/1

### Design axioms

* **Zero means zero.** Not "fewer bugs" or "the important ones." The goal is to fix every known soundness issue, however obscure.

* **Migrate, don't break.** Breakage is acceptable to prevent unsound behavior but care should be taken to minimize the practical impact on users. Phasing in breaking changes gradually through future-compatibility warnings is preferred, particularly in cases where many crates affected.

### What we are shooting for

Our goal is simple: *If you stick to safe Rust, you don't have to worry about memory unsafety.* End of story. 

### How we get there

| Goal                             | Timespan | What and why |
|----------------------------------|----------|--------------|
| (((ROADMAP ROWS: Project Zero))) |
| Fix self-contained bugs and issues | 2026 - Future | There are a few minor unsoundnesses which don't interact with the rest of the type system. Fixing these can happen in parallel to other work |
| Remove higher-ranked subtyping and replace it with coercions | 2026 | Subtyping can currently impact trait selection, breaking library abstractions such as `Pin` |
| Fix overlap between builtin and user-written impls for trait objects | Future | Work on a design for `dyn`-trait which avoids [an existing unsoundness](https://github.com/rust-lang/rust/issues/57893) due to overlapping impls |
| Support emitting FCW for borrowck errors | 2026 | Support rerunning borrowck in case of errors, weakening the failure to future-compatability warnings to give users the time to fix their crates |  
| Check where-bounds when instantiating binders | Future | Checking where bounds when instantiating binders fixes most of our unsoundnesses around implied bounds. Blocked on supporting assumptions on binders |

[Open soundness bugs][unsoundnesses] remain open because naive fixes to them would also cause a number of safe Rust code patterns to break. Fixing these bugs in a way that ensures valid code continues to work requires extending Rust's type system implementation with new capabilities.

**Step 1: Stabilize the next-generation trait solver.** For the last few years the Rust types team has been developing a [next-generation trait solver](./next-solver.md) that will lift a number of limitations in the current trait system. This new trait solver started being used for coherence checking in Rust 1.84. In 2026 we plan to stabilize it for use across all of Rust. This step is needed to unblock virtually all of the following steps.

**Step 2: Fix self-contained issues and simplify implementation.** A number of soundness bugs can be fixed independently, without waiting for larger infrastructure changes. These range from straightforward fixes (closure return value checking, orphan check projection handling) to more involved work (replacing higher-ranked subtyping with coercions, fixing `Pin::new` soundness). Each fix ships with future-compatibility warnings first.

**Step 3: Land support for where-bounds on binders.** A cluster of soundness bugs — implied bounds on nested references, normalization skipping well-formedness checks, and several related issues — are blocked on the new solver *and* on [support for higher-ranked assumptions](./assumptions_on_binders.md). This work also requires the ability to emit future-compatibility warnings from the borrow checker.

**Step 4: Formalize trait system semantics.** To prevent future soundness bugs, we need a formal specification of what the trait solver *should* do. This catches bugs before they ship and provides a reference for correctness as the type system evolves.

## Frequently asked questions

### Will these fixes break my code?

They might, but only if your code was relying on unsound behavior and you'll get advance warning in that case. Soundness fixes causing non-trivial breakage land with future-compatibility warnings first, giving crate authors time to update before the old behavior is removed.

### How many soundness bugs are there?

The [tracking board][unsoundnesses] lists the known issues. Some are self-contained and can be fixed independently; others are blocked on the next-generation trait solver or require design work on where-bounds and binder semantics.
