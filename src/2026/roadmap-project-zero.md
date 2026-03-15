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

Rust's defining promise is that safe code cannot cause undefined behavior. Users depend on this guarantee to build critical systems - if the type system says it's safe, it should be safe.

In practice, Rust's type system has accumulated [known soundness bugs][unsoundnesses] over the years. These bugs are obscure and have rarely if ever affected production systems, but they nonetheless reduce Rust's value proposition.

Knowing that their program will never violate memory safety gives users an important guardrail, allowing them to hack without fear.

[unsoundnesses]: https://github.com/orgs/rust-lang/projects/61/views/1

### Design axioms

* **Zero means zero.** Not "fewer bugs" or "the important ones." The goal is to fix every known soundness issue, however obscure.

* **Migrate, don't break.** Breakage is acceptable only if it is necessary to prevent the unsound behavior, and even then the transition should be gradual. If it causes non-trivial breakage, fixes must use future-compatibility warnings to give the ecosystem time to adapt. 

### What we are shooting for

If you don't write `unsafe`, you don't get memory unsafety. No caveats, no asterisks, no "except for these obscure edge cases." Rust's safety guarantee holds completely.

### How we get there

| Goal                             | Timespan | What and why |
|----------------------------------|----------|--------------|
| (((ROADMAP ROWS: Project Zero))) |
| Fix self-contained bugs and issues | 2026 - Future | There are a few minor unsoundnesses which don't interact with the rest of the type system. Fixing these can happen in parallel to other work |
| Remove higher-ranked subtyping and replace it with coercions | 2026 | Subtyping can currently impact trait selection, breaking library abstractions such as `Pin` |
| Fix overlap between builtin and user-written impls for trait objects | Future | Work on a design for `dyn`-trait which avoids [an existing unsoundness](https://github.com/rust-lang/rust/issues/57893) due to overlapping impls |
| Support emitting FCW for borrowck errors | 2026 | Support rerunning borrowck in case of errors, weakening the failure to future-compatability warnings to give users the time to fix their crates |  
| Check where-bounds when instantiating binders | Future | Checking where bounds when instantiating binders fixes most of our unsoundnesses around implied bounds. Blocked on supporting assumptions on binders |

The [known soundness bugs][unsoundnesses] aren't independent — many share common blockers, and fixing them requires a specific sequencing of infrastructure work.

**Step 1: Stabilize the next-generation trait solver.** The current trait solver has accumulated technical debt and limitations that make correct fixes impossible without breaking its assumptions. The [next-generation trait solver](./next-solver.md), already stable for coherence checking since Rust 1.84, provides the sound foundation needed to fix bugs that were previously intractable. Stabilizing it everywhere is the single highest-leverage step.

**Step 2: Fix self-contained issues and simplify implementation.** A number of soundness bugs can be fixed independently, without waiting for larger infrastructure changes. These range from straightforward fixes (closure return value checking, orphan check projection handling) to more involved work (replacing higher-ranked subtyping with coercions, fixing `Pin::new` soundness). Each fix ships with future-compatibility warnings first.

**Step 3: Land support for where-bounds on binders.** A cluster of soundness bugs — implied bounds on nested references, normalization skipping well-formedness checks, and several related issues — are blocked on the new solver *and* on [support for higher-ranked assumptions](./assumptions_on_binders.md). This work also requires the ability to emit future-compatibility warnings from the borrow checker.

**Step 4: Formalize trait system semantics.** To prevent future soundness bugs, we need a formal specification of what the trait solver *should* do. This catches bugs before they ship and provides a reference for correctness as the type system evolves.

## Frequently asked questions

### Will these fixes break my code?

They might, but only if your code was relying on unsound behavior and you'll get advance warning in that case. Soundness fixes causing non-trivial breakage land with future-compatibility warnings first, giving crate authors time to update before the old behavior is removed.

### How many soundness bugs are there?

The [tracking board][unsoundnesses] lists the known issues. Some are self-contained and can be fixed independently; others are blocked on the next-generation trait solver or require design work on where-bounds and binder semantics.
