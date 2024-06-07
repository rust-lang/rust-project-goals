# The 2024H2 goal slate

This document explains the 2024H2 goal slate and how it was chosen. If you just want to see a table of goals, see the [all candidates](./candidates.md) page.

> *![Status: Experimental](https://img.shields.io/badge/Status-Experimental-yellow) This document is a draft. The reasoning and [goal slate](./slate.md) are still evolving. If you have thoughts or suggestions, please reach out to nikomatsakis on the [#project-goals-2024h2](https://rust-lang.zulipchat.com/#narrow/stream/435869-project-goals-2024h2) Zulip stream.*

## Rust’s mission

Rust’s mission is to *empower everyone to build reliable and efficient programs*. Overall our goal is to continue to grow adoption, but specifically in ways that will help us in driving this mission. 

## Axioms for Rust adoption

The slate was chosen according to the following axioms.

* **Slow and steady wins the race.** For this first round of goals, we want a small set that can be completed without undue stress. As the Rust open source org continues to grow, the set of goals can grow in size.
* **Rust must deliver on its promise of peak performance and high reliability.** Rust’s maximum advantage is in applications that require peak performance or low-level systems capabilities. We must continue to innovate and support those areas above all.
* **Rust must be “good enough” for much more.** If Rust is *only* worth using for those programs that truly need its advantages, that will not be enough value to justify its continued existence. Our goal is that Rust is productive enough to be worth using even when its full power is not required — and this in turn makes using Rust more pleasant for those critical applications.

## Areas where Rust is best suited and how it grows

Rust offers particular advantages in two areas:

* **Latency sensitive or high scale network services**, which benefit from Rust’s lack of garbage collection pauses (in comparison to GC’d languages).
* **Low-level systems applications**, like kernels and embedded development, benefit from Rust’s memory safety guarantees and high-level productivity (in comparison to C or C++).
* **Developer tooling** has proven to be an unexpected growth area, with tools ranging from IDEs to build systems being written in Rust.

### How Rust adoption grows

The typical pattern is that Rust adoption begins in a system where Rust offers particular advantage. For example, a company building network services may begin with a highly scaled service. In this setting, the need to learn Rust is justified by its advantage. 

Once users are past the initial learning curve, they find that Rust can be quite productive. They spend more time getting their program to build, but less time debugging; they are able to complete major refactorings successfully. They begin to use it for projects where Rust’s advantage is less and less — and in some cases in areas where other languages might have allowed for faster iteration, but Rust is “good enough”.

While it is not a goal for Rust to be used for all programs, Rust’s versatility — its ability to be “good enough” for a broad range of programs, even when other languages are strictly better — is a key part of making it a success. It both helps to make Rust adoption more practical (there are advantages to using one language for as many projects as possible) and the work to make Rust usage practical and ergonomic makes it more pleasant in all cases, making it even more effective when it is "in its element".

### How Rust adoption stalls

Anecdotally, the most commonly cited reasons to stop using Rust is a feeling that development is "too slow" or "too complex". There is not any one cause for this.

* **Language complexity:** Most users that get frustrated with Rust do not cite the borrow checker but rather the myriad workarounds needed to overcome various obstacles and inconsistencies. Often "idomatic Rust" involves a number of crates to cover gaps in core functionality (e.g., `anyhow` as a better error type, or `async_recursion` to permit recursive async functions). Language complexity is a particular problem
* **Picking crates:** Rust intentionally offeres a lean standard library, preferring instead to support a rich set of crates. But when getting started users are often overwhelmed by the options available and unsure which one would be best to use. Making matters worse, Rust documentation often doesn't show examples making use of these crates in an effort to avoid picking favorites, making it harder for users to learn how to do things.
* **Build times and slow iteration:** Being able to make a change and quickly see its effect makes learning and debugging effortless. Despite our best efforts, real-world Rust programs do still have bugs, and finding and resolving those can be frustratingly slow when every change requires waiting minutes and minutes for a build to complete.

### Additional concerns faced by companies

For larger users, such as companies, there are additional concerns:

* **Uneven support for cross-language invocations:** Most companies have large existing codebases in other languages. Rewriting those codebases from scratch is not an option. Sometimes it possible to integrate at a microservice or process boundary, but many would like a way to rewrite individual modules in Rust, passing data structures easily back and forth. Rust's support for this kind of interop is uneven and often requires knowing the right crate to use for any given language.
* **Spotty ecosystem support, especially for older things:** There are a number of amazing crates in the Rust ecosystem, but there are also a number of notable gaps, particularly for older technologies. Larger companies though often have to interact with legacy systems. Lacking quality libraries makes that harder.
* **Supply chain security:** Leaning on the ecosystem also means increased concerns about supply chain security and business continuity. In short, crates maintained by a few volunteers rather than being officially supported by Rust are a risk. 
* **Limited hiring pool:** Hiring developers skilled in Rust remains a challenge. Companies have to be ready to onboard new developers and to help them learn Rust. Although there are many strong Rust books available, as well as a number of well regarded Rust training organizations, companies must still pick and choose between them to create a "how to learn Rust" workflow, and many do not have the extra time or skills to do that.

## Flagship goals

Flagship goals are the most impactful, most ambitious goals that we will attempt to complete. They are often part of a larger program and effort that is expected to span over multiple years. For 2024h2, our flagship goals are listed below. Pursuant to our [selection axioms](#axioms-for-rust-adoption), we are focused primarily on closing gaps around async Rust and low-level systems programming (the Rust for Linux project, specifically) but we include some goals that target general productivity.

* [**Release the Rust 2024 edition**](./Rust-2024-Edition.md), accepted in [RFC ], [will contain](./Rust-2024-Edition.md#the-next-few-steps)
    * a change in how `impl Trait` capture bounds work ([RFC #3498](https://github.com/rust-lang/rfcs/pull/3498) and [RFC #3617](https://github.com/rust-lang/rfcs/pull/3617))
    * reserving the `gen` keyword to allow for generators ([RFC #3513](https://github.com/rust-lang/rfcs/pull/3513))
    * along with an [assortment of other possible changes](TODO)
    * never type fallback ([#123748](https://github.com/rust-lang/rust/issues/123748))
    * and a [number of other potential changes](https://github.com/rust-lang/rust/issues?q=label%3AC-tracking-issue+label%3AA-edition-2024+label%3AS-tracking-ready-to-stabilize%2CS-tracking-needs-documentation+-label%3AS-tracking-impl-incomplete%2CS-tracking-design-concerns) that may be included if they make enough progress
* [**Bringing the Async Rust experience closer to parity with sync Rust**](./async_fn_everywhere.md) [via](./async_fn_everywhere.md#the-next-few-steps):
    * stabilizing async closures, thus enabling richer, combinator APIs like sync Rust's [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html);
    * [resolving the "send bound problem"](./async_fn_everywhere.md#resolve-the-send-bound-problem), thus enabling foundational, generic traits like Tower's [`Service`]() trait;
    * [stabilizing a trait in libstd for async iteration](./async_fn_everywhere.md#stabilize-trait-for-async-iteration), thus enabling the ecosystem to build atop a stable foundation;
    * [authoring a draft RFC for async vision](./async_fn_everywhere.md#author-draft-rfc-for-async-vision), thus aligning the project around a coherent vision;
    * [completing the async drop experiments](./async_fn_everywhere.md#complete-async-drop-experiments) proposed in [MCP 727][], laying the groundwork for resolving the the next most
* [**Resolving the biggest blockers to Linux building on stable Rust**](./rfl_stable.md) [via](./rfl_stable.md#the-next-few-steps):
    * [stabilizing support for arbitrary `self` types and unsizeable smart pointers](./rfl_stable.md#stable-support-for-rfls-customized-arc-type), thus permitting ergonomic support for [in-place linked lists](https://rust-for-linux.com/arc-in-the-linux-kernel) on stable;
    * [stabilizing features for labeled goto in inline assembler and extended `offset_of!` support](./rfl_stable.md#labeled-goto-in-inline-assembler-and-extended-offset_of-support), needed for various bts of low-level coding;
    * [adding Rust For Linux project on Rust CI](./rfl_stable.md#rfl-on-rust-ci), thus ensuring we don't accidentally cause regressions for this highly visible project.
    *  ![Owner Needed][] We would also like to do the following but currently lack owners:
        * [stabilizing support for pointers to statics in constants](./rfl_stable.md#pointers-to-statics-in-constants), permitting the construction of vtables for kernel modules
        * [stabilize options for building core/alloc with fewer features](./rfl_stable.md#custom-builds-of-corealloc-with-specialized-configuration-options), allowing the kernel to forbid infallible allocation and other aspects of the standard libraries that it does not want;
        * [code-generation features and compiler options](./rfl_stable.md#code-generation-features-and-compiler-options), allowing Rust to match the compilers given to gcc/clang when building the kernel.

> **WIP:** There are several other [candidate flagship goals](./candidates.md#candidate-flagship-goals) and it is possible that this list may change to include more items or to replace one of the above with goals with something else.

[MCP 727]: https://github.com/rust-lang/compiler-team/issues/727

## Team goals

In addition to our flagship goals, we include a number of "team goals" that the various Rust teams would like to advertise. These goals tend to be smaller in scope and more "solution-oriented". They aren't generally the big deadlines that will grab peoples' attention. But don't be deceived, their impact on your daily coding can be as big or greater!

Accepted team goals include:

* [Preparing this slate of goals](./Project-goal-slate.md)



> **WIP:** There are several other [candidate team goals](./candidates.md#candidate-teams-goals) and it is likely that some of them will be added to this list.

[Owner Needed]: https://img.shields.io/badge/Owned%20Needed-blue
