# Ergonomic ref-counting: RFC decision and preview

| Metadata         |                                    |
|:-----------------|------------------------------------|
| Point of contact | @nikomatsakis                      |
| Teams            | <!-- TEAMS WITH ASKS -->           |
| Task owners      | <!-- TASK OWNERS -->               |
| Status           | Proposed                           |
| Tracking issue   | [rust-lang/rust-project-goals#107] |
| Zulip channel    | N/A                                |

## Summary

We propose to write an alternative RFC for ergonomic ref-counting that makes lightweight cloning automatic and hold design meetings so the lang team can compare both approaches. This work builds on RFC #3680, which proposed a new keyword, `use`, that could be used with closures (`use || ...`) and expressions like `x.use` to help address a longstanding problem: working with ref-counted data structures like `Arc<T>` is verbose and confusing.

The 2025H1 work delivered `.use` syntax that works technically, but community feedback on the RFC pointed out a tension: if we're trying to improve ergonomics, why add more required syntax? We'll write an RFC exploring the automatic approach, complete the implementation work needed to support it, and run design meetings to help the lang team decide. This will tell us about the impact on user experience and performance, how well we can catch problematic cases, and which approach works better for Rust developers working with ref-counted data.

## Motivation: Reference counting is a common pain point across many kinds of Rust applications

### The status quo

Working with ref-counted data structures creates verbose and confusing patterns that affect most non-trivial Rust applications. The friction appears not just with user-facing `Arc<T>` and `Rc<T>`, but throughout Rust's ecosystem where ref-counting is pervasive but often hidden.

Consider this common pattern when spawning async tasks:

```rust
// Today: verbose and repetitive
let config = Arc::new(load_config());
let database = Arc::new(connect_db());
let metrics = Arc::new(MetricsCollector::new());

let config_clone = config.clone();
let database_clone = database.clone(); 
let metrics_clone = metrics.clone();
spawn(async move {
    process_request(config_clone, database_clone, metrics_clone).await;
});
```

This pattern appears throughout Rust codebases, particularly in async and concurrent code. The repetitive cloning obscures the actual logic and creates maintenance burden when adding or removing shared data.

#### Applies to high- and low-level code

Explicit cloning friction affects all kinds of Rust projects. This is most apparent in higher-level applications, where the need for explicit cloning (particularly in closures) emerges as one of the largest pain points, but it applies to lower-level development too, particularly async network services. Ref-counting is also applicable beyond `Rc` or `Arc`, as numerous types (e.g., the channel channel handles found in libstd and tokio) are ref-counted under the hood. Improving the ergonomics of working with ref-counted data is one of those rare cases to simultaneously improve the lives of developers using interop crates like PyO3, GUI crates like Sycamore and dioxus, and tokio simultaneously. As the blog post from [Dioxus labs](https://dioxus.notion.site/Dioxus-Labs-High-level-Rust-5fe1f1c9c8334815ad488410d948f05e) put it:

> While working at Cloudflare, I had to work with a struct with nearly 30 fields of Arced data. Spawning tokio tasks looked like:
>
> ```rust
> // listen for dns connections
> let _some_a = self.some_a.clone();
> let _some_b = self.some_b.clone();
> let _some_c = self.some_c.clone();
> let _some_d = self.some_d.clone();
> let _some_e = self.some_e.clone();
> let _some_f = self.some_f.clone();
> let _some_g = self.some_g.clone();
> let _some_h = self.some_h.clone();
> let _some_i = self.some_i.clone();
> let _some_j = self.some_j.clone();
> tokio::task::spawn(async move {
>   	// do something with all the values
> });
>```
>
> Working on this codebase was demoralizing. We could think of no better way to architect things - we needed listeners for basically everything that filtered their updates based on the state of the app. You could say “lol get gud,” but the engineers on this team were the sharpest people I’ve ever worked with. Cloudflare is all-in on Rust. They’re willing to throw money at codebases like this. Nuclear fusion won’t be solved with Rust if this is how sharing state works.

#### Projects go great lengths to avoid the need for clone

Projects today structure their APIs in part to avoid the need for users to call clone. Dioxus's [0.5.0](https://dioxuslabs.com/blog/release-050/) release pivoted their framework around implicit runtime arenas; Sycamore did the same in [0.9](https://sycamore.dev/post/announcing-v0-9-0) and Leptos uses a similar approach. In the compiler, where the use of arenas allows most types to be `Copy`, we've encountered resistance to extracting libraries out for wider reuse due to the need to add `clone` calls which obscure the logi, and projects like MiniRust and a-mir-formality have built elaborate facades primarily to avoid having to call `clone`.

#### At the same time, there is a time and a place for explicit ref-counting

There are applications where explicit ref-counting is important. Methods like `Rc::make_ref` or `Arc::make_ref`, for example, have very different cost depending on whether the ref-count is 1 or 2, so knowing when ref-counts are incremented or decremented can be relevant. Naive and widespread use of atomic reference counting can have measurable impact on optimized applications, so avoiding reference-counts when not needed (or preferring non-atomic reference counting) is worthwhile.

## The story so far

This work builds on RFC #3680 and the 2025H1 ergonomic ref-counting goal, which delivered a working nightly implementation of `.use` syntax and generated extensive community feedback that revealed an important design question.

### RFC #3680 and experimental implementation

RFC #3680, titled "Simplify lightweight clones, including into closures and async blocks," proposed a feature to simplify performing lightweight clones (such as of `Arc`/`Rc`), particularly cloning them into closures or async blocks, while still keeping such cloning visible and explicit.

The RFC identified common friction patterns where developers need to clone `Arc<T>` reference-counted objects into async blocks or tasks, showing examples of the verbose workarounds currently required. The proposal aimed to "minimize the syntactic weight of lightweight-cloning objects, particularly cloning objects into a closure or async block, **while still keeping an indication of this operation**."

The core approach was to introduce a `Use` trait that types like `Arc<T>` and `Rc<T>` could implement to opt into lightweight cloning behavior, along with new syntax (`x.use` and `use || { ... }`) to make this cloning more ergonomic while keeping it explicit.

### Experimental implementation available on Nightly

Nightly Rust now has an experimental implementation of RFC #3680 (modulo the caveats in the [FAQ](#what-is-the-current-status-of-the-implementation)):

```rust
let config = Arc::new(load_config());
let database = Arc::new(connect_db());
let metrics = Arc::new(MetricsCollector::new());

spawn(use async {
    // config, database, metrics are automatically cloned into the block
    process_request(config, database, metrics).await;
});
```

### RFC feedback: positive on the idea, but concerns on the details

RFC #3680 generated extensive discussion with over 80 comments of feedback ([summarized here](https://github.com/rust-lang/rfcs/pull/3680#issuecomment-2625526944)) that showed both support for addressing the ergonomics problem and concerns about the specific solution. Contributors from projects like PyO3 highlighted the real-world impact of the current friction, particularly in domains that make heavy use of reference counting.

The discussion covered several themes: concerns about defining what constitutes "cheap" cloning, questions about compiler optimizations that could lead to behavioral differences, debates about keyword overloading with `use`, and various alternative syntax proposals. However, one critique stood out as particularly compelling and worth deeper exploration. Outside of the RFC thread, concerns were raised about compilation time impact, since, at least if naively implemented, this new approach involves more use of the trait system than is currently the case.

### Key design question: will this make Rust easier?

Among the various concerns raised, the most hard-hitting critique questioned whether explicit `.use` syntax actually serves the stated ergonomic goals. [Diggsey's comment](https://github.com/rust-lang/rfcs/pull/3680#issuecomment-2301770660) articulated this view:

> Having to navigate a codebase filled with this strange .use keyword, which is hard to explain (it copies the value.. except when it doesn't, and it can be used on Use types, but not Clone types, but Use is not Copy, it's different... When is something Use? What makes something lightweight? Uh... read this essay. Why do I need to make this closure use but this one doesn't need to be? Ahahaha, yeah this is going to take a while to explain...) is more of a blocker than clone-into-closure ever was.

This critique suggests that if the goal is ergonomics, requiring more explicit syntax may be counterproductive. We propose to explore an alternative approach that makes lightweight cloning automatic by default, with optional warnings for non-trivial cases. This would allow the lang team to evaluate both the maximally explicit approach (current RFC) and the seamlessly integrated approach before deciding on Rust's direction for ergonomic improvements.

### The next 6 months

To explore the automatic alternative, we will:

* Author an alternative RFC that avoids new keywords and includes a lint fo help catch potentially incorrect usage.
* Implement the new design (feature-gated, natch) to support experimentation and measure compilation overhead.
* Estimate the prevalence of code that benefits or is complicated by this change using Crater runs or other experiments.
* Conduct design reviews with the lang team to compare/contrast the two approaches.

### The "shiny future" we are working towards

Ergonomic ref-counting represents a friction point in multiple domains. Async Rust programs frequently have context shared across multiple tasks (e.g., server state) that is managed via reference counting. GUI applications also have callbacks and data patterns that do not correlate well to the stack. In these domains, the existence of `.clone()` calls (or `.use` notation) represents syntactic noise that distracts from the essential data flow patterns. 

**Removing barriers in existing domains:** In network services and async applications where Rust already excels, eliminating explicit cloning friction will make complex architectures more maintainable and readable. Developers won't need to choose between ergonomic APIs and performance, or resort to arena allocation patterns primarily for ergonomic reasons.

**Opening new application domains:** More significantly, this change opens up domains where Rust would otherwise be an excellent choice but where cloning friction currently tips the cost-benefit analysis against adoption. GUI applications, reactive programming, data processing pipelines, and other high-level domains become viable when the ergonomic barrier is removed.

**Enabling natural architectural patterns:** Projects like Dioxus won't need to build custom unsafe abstractions to avoid ref-counting friction. Mathematical formalism projects won't need preprocessors to eliminate distracting `.clone()` calls. The Rust compiler and other complex applications can use ref-counting patterns more naturally without relying primarily on arenas for ergonomic relief.

This isn't about making Rust easier for beginners (though it will) - it's about removing a pervasive friction point that affects most non-trivial Rust applications and currently blocks adoption in domains where Rust's performance and safety would otherwise be compelling. The lang team's decision between explicit and seamless approaches will establish important precedent for how Rust balances ergonomics with explicitness in future language evolution.

## Design axioms

The design axioms for this alternative RFC are as follows:

* **Clarity of purpose.** Although experienced users have learned to live with it, we believe that the current copy/clone introduces noise that distracts from being able to read the code (and, along the way, blocks new user adoption). Our top goal is to encourage Rust code that is clearer of purpose, no matters its domain. 
* **Competitive or improved performance.** Using this feature should lead to code which is either as efficient or, in some cases, more efficient that you get today.

## Ownership and team asks

| Task                              | Owner(s) or team(s)                | Notes |
|-----------------------------------|------------------------------------|-------|
| Author alternative RFC            | @nikomatsakis                      | Seamlessly integrated approach |
| Complete seamless implementation  | @spastorino                        | Make `x` equivalent to `x.use` with optional linting |
| Standard reviews                  | ![Team][] [compiler]               |       |
| Lang-team champion                | @nikomatsakis                      |       |
| Design meetings                   | ![Team][] [lang]                   | Two meetings to evaluate both approaches |
| RFC decision                      | ![Team][] [lang]                   | Choose between maximally additive vs seamlessly integrated |

### Definitions

Definitions for terms used above:

* *Discussion and moral support* is the lowest level offering, basically committing the team to nothing but good vibes and general support for this endeavor.
* *Author RFC* and *Implementation* means actually writing the code, document, whatever.
* *Design meeting* means holding a synchronous meeting to review a proposal and provide feedback (no decision expected).
* *RFC decisions* means reviewing an RFC and deciding whether to accept.
* *Org decisions* means reaching a decision on an organizational or policy matter.
* *Secondary review* of an RFC means that the team is "tangentially" involved in the RFC and should be expected to briefly review.
* *Stabilizations* means reviewing a stabilization and report and deciding whether to stabilize.
* *Standard reviews* refers to reviews for PRs against the repository; these PRs are not expected to be unduly large or complicated.
* *Prioritized nominations* refers to prioritized lang-team response to nominated issues, with the expectation that there will be *some* response from the next weekly triage meeting.
* *Dedicated review* means identifying an individual (or group of individuals) who will review the changes, as they're expected to require significant context.
* Other kinds of decisions:
    * [Lang team experiments](https://lang-team.rust-lang.org/how_to/experiment.html) are used to add nightly features that do not yet have an RFC. They are limited to trusted contributors and are used to resolve design details such that an RFC can be written.
    * Compiler [Major Change Proposal (MCP)](https://forge.rust-lang.org/compiler/mcp.html) is used to propose a 'larger than average' change and get feedback from the compiler team.
    * Library [API Change Proposal (ACP)](https://std-dev-guide.rust-lang.org/development/feature-lifecycle.html) describes a change to the standard library.

## Frequently asked questions

### What is the current status of the implementation?

The implementation plan for the feature gate `ergonomic_clones` (experimental version of RFC #3680) is as follows. The following steps are considered required for the basic feature functionality. A step is checked if it is present in nightly.

* [x] Introduce a `UseCloned` type implemented for `Rc` and `Arc` and possibly other types
    * The RFC called this trait `Use`; the names will have to be reconciled. We deviated because `x.use` can be applies to values of any type, including things that do not implement this trait, so the previous name felt confusing. The intention of `UseCloned` is "when a value of this type is `use`d, it will (if necessary) be cloned".
* [x] Introduce the `use` keyword as an operator on places, e.g., `some_place.use`
* [x] Introduce `use || /* body */` closures. These are equivalent to `move` closures except that, where each captured place `place` is stored into a move closure with an initializer like `f: place`, `use` closures contain fields initialized with `f: place.use`.
* [x] In MIR Build, `place.use` is compiled as a call to `clone` with a [`call_source`](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/mir/enum.TerminatorKind.html#variant.Call.field.call_source) of [`CallSource::Use`](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/mir/enum.CallSource.html#variant.Use):
    * As a compile-time optimization, the MIR build for `place.use` depends on the traits implemented by the type `T` of `place`:
        * If the type `T` is known to implement `Copy` (modulo regions), then we compile `place.use` to a copy like `place`.
        * If the type `T` is known to implement `UseCloned` (modulo regions), then we compile it to a call with `call_source` as described above.
        * Otherwise, `place.use` is compiled to a move.
        * This is a compile-time optimization because, if we didn't do this, it would be optimized later by the monorphization-time optimization, but we would spend more effort in the meantime.
    * If the type of `x` is NOT known to implement `UseCloned`, then `x` will be compiled as a move.
* [x] Integrate `some_place.use` into borrow check (fairly trivial)
* [ ] Identity candidates for "last-use" optimization (`some_place.use` is a *last use* if `some_place` is never used again)
* [x] At code generation time, the semantics of `some_place.use` depends on the type `T` of `some_place`:
    * If `T` implements `Copy`, then `some_place.use` is a copy
    * If `T` implements `UseCloned` (and is not a last-use), then `some_place.use` is compiled as a call to `some_place.clone()`
    * Otherwise, `some_place.use` is a move.

The following features are not planned for implementation until the future; they are a "nice to have":

* [ ] Implement `use`-elision and inter-procedural optimization
    * If `some_place.use` never escapes the current stack frame and no mutation is occurring, we can forego the call to `clone()`.

### How does this goal relate to stabilization?

This goal is explicitly *not* about stabilization - it's about getting to a clear decision point. The chosen approach and its implementation will represent a step toward eventual stabilization, but the focus here is on resolving the fundamental design question that emerged from 2025H1.

Once the lang team chooses a direction, subsequent work can focus on refinement, community testing, and the stabilization process without the uncertainty of fundamental design questions.

### What is the relationship to the 2025H1 ergonomic ref-counting goal?

This goal builds on the 2025H1 implementation work and the feedback from the RFC. The current implementation already includes most of the infrastructure needed for both approaches - the remaining technical work is primarily about completing the seamless integration option.

