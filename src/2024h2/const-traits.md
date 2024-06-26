# Const traits

> **Instructions:** Copy this template to a fresh file with a name based on your plan.
> Update the text. Feel free to replace any text with anything, but there are placeholders
> designed to help you get started. Also, while this template has received some iteration,
> it is not sacrosant. Feel free to change the titles of sections or make other changes that you think 
> will increase clarity.

| Metadata | |
| --- | --- |
| Owner(s) | feel1-dead |
| Teams | 
| Status | WIP |

## Motivation

Rust's compile time functionalities (`const fn`, `const`s, etc.) are greatly limited in terms of expressivity because const functions currently do not have access to generic trait bounds as runtime functions do. Developers want to write programs that do complex work in compile time, most of the times to offload work from runtime, and being able to have const `trait`s and const `impl`s will greatly reduce the difficulty of writing such compile time functions.

*Begin the motivation with a short (1 paragraph, ideally) summary of what the goal is trying to achieve and why it matters.*

### The status quo

People write a lot of code that will be run in compile time. They include procedural macros, build scripts ((42.8k hits)[build scripts] on GitHub for `build.rs`), and const functions/consts ((108k hits)[const fns] on GitHub for `const fn`). Not being able to write const functions with generic behavior is often cited as a pain point of Rust's compile time capabilities. Because of the limited expressiveness of `const fn`, people may decide to move some compile time logic to a build script, which could increase build times, or simply choose not to do it in compile time (even though it would have helped runtime performance).

There are also language features that require the use of traits, such as iterating with `for` and handling errors with `?`. Because the `Iterator` and `Try` traits currently cannot be used in constant contexts, people are unable to use `?` to handle results, nor use iterators e.g. `for x in 0..5`.
*Elaborate in more detail about the problem you are trying to solve. This section is making the case for why this particular problem is worth prioritizing with project bandwidth. A strong status quo section will (a) identify the target audience and (b) give specifics about the problems they are facing today. Sometimes it may be useful to start sketching out how you think those problems will be addressed by your change, as well, though it's not necessary.*

[build scripts]: https://github.com/search?q=path%3A**%2Fbuild.rs+NOT+is%3Afork&type=code
[const fns]: https://github.com/search?q=%22const+fn%22+language%3Arust+NOT+is%3Afork&type=code

### The next few steps

In 2024, we plan to:
* Finish experimenting with an effects-based desugaring for ensuring correctness of const code with trait bounds
* Land a relatively stable implementation of const traits
* Make all [UI tests] pass.
*Sketch out the specific things you are trying to achieve in 2024. This should be short and high-level -- we don't want to see the design!*

[UI tests]: https://github.com/rust-lang/rust/blob/master/tests/ui/rfcs/rfc-2632-const-trait-impl/

### The "shiny future" we are working towards

We're working towards enabling developers to do more things in general within a `const` context. Const traits is a blocker for many future possibilities (see also the const eval [feature skill tree]) including heap operations in const contexts. 

[feature skill tree]: https://rust-lang.github.io/const-eval/skill_tree.html

*If this goal is part of a larger plan that will extend beyond this goal period, sketch out the goal you are working towards. It may be worth adding some text about why these particular goals were chosen as the next logical step to focus on.*

## Design axioms

*Add your [design axioms][da] here. Design axioms clarify the constraints and tradeoffs you will use as you do your design work. These are most important for project goals where the route to the solution has significant ambiguity (e.g., designing a language feature or an API), as they communicate to your reader how you plan to approach the problem. If this goal is more aimed at implementation, then design axioms are less important. [Read more about design axioms][da].*

[da]: ../about/design_axioms.md

## Ownership and other resources

**Owner:** *Identify a specific person or small group of people if possible, else the group that will provide the owner*

*This section describes the resources that you the contributors are putting forward to address this goal. This includes people: you can list specific people or a number of people -- e.g., 2 experienced Rust engineers working 2 days/wk. Including details about experience level and background will help the reader to judge your ability to complete the work.*

*You can also include other resources as relevant, such as hardware, domain names, or whatever else.*

### Support needed from the project

*Identify which teams you need support from -- ideally reference the "menu" of support those teams provide. Some common considerations:*

* Will you be authoring RFCs? How many do you expect? Which team will be approving them?
    * Will you need design meetings along the way? And on what cadence?
* Will you be authoring code? If there is going to be a large number of PRs, or a very complex PR, it may be a good idea to talk to the compiler or other team about getting a dedicated reviewer.
* Will you want to use "Rust project resources"...?
    * Creating rust-lang repositories?
    * Issuing rust-lang-hosted libraries on crates.io?
    * Posting blog posts on the Rust blog? (The Inside Rust blog is always ok.)

## Outputs and milestones

### Outputs

*Final outputs that will be produced*

### Milestones

*Milestones you will reach along the way*

## Frequently asked questions

### What do I do with this space?

*This is a good place to elaborate on your reasoning above -- for example, why did you put the design axioms in the order that you did? It's also a good place to put the answers to any questions that come up during discussion. The expectation is that this FAQ section will grow as the goal is discussed and eventually should contain a complete summary of the points raised along the way.*