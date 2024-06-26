# a-mir-formality modeling the borrow checker, coherence

| Metadata | |
| --- | --- |
| Owner(s) | [nikomatsakis] |
| Teams | [Types] |
| Status | WIP |

[nikomatsakis]: https://github.com/lqd
[Types]: https://www.rust-lang.org/governance/teams/compiler#team-types

## Motivation

Our goal is to get the a-mir-fornality project off the ground by offering initial models of the trait solver and borrow checking. 

### The status quo

Most communication and definition of Rust's type/trait system today takes place through informal argument and with reference to compiler internals. a-mir-formality offers a model of Rust at a much higher level, but it remains very incomplete compared to Rust and, thus far, it has been primarily developed by nikomatsakis.

### The next few steps

The goal for a-mir-formality this year is to bootstrap it as a live, maintained project:

* Achieve 2 regular contributors from T-types in addition to nikomatsakis
* Support fuzz testing and/or the ability to test against rustc

### The "shiny future" we are working towards

The eventual goal is for a-mir-formality to serve as the official model of how the Rust type system works.
We have found that having a model enables us to evaluate designs and changes much more quickly than trying to do everything in the real compiler.
We envision a-mir-formality being updated with new features prior to stabilization which will require it to be a living codebase with many contributors.
We also envision it being tested both through fuzzing and by comparing its results to the compiler to detect drift.

## Design axioms

* **Designed for exploration and extension by ordinary Rust developers.** Editing and maintaing formality should not require a PhD. We prefer lightweight formal methods over strong static proof.
* **Focused on the Rust's static checking.** There are many things that a-mir-formality could model. We are focused on those things that we need to evaluate Rust's static checks. This includes the type system and trait system.
* **Clarity over efficiency.** Formality's codebase is only meant to scale up to small programs. Efficiency is distinctly secondary.
* **The compiler approximates a-mir-formality, a-mir-formality approximates the truth.** Rust's type system is Turing Complete and cannot be fully evaluated. We expect the compiler to have safeguards (for example, overflow detection) that may be more conservative than those imposed by a-mir-formality. In other words, formality may accept some programs the compiler cannot evaluate for practical reasons. Similarly, formality will have to make approximations relative to the "platonic ideal" of what Rust's type system would accept.

## Ownership and other resources

**Owner:** nikomatsakis

We will require participation from at least 2 other members of T-types. Current candidates are lcnr + compiler-errors.

## Outputs and milestones

### Outputs

*Final outputs that will be produced*

### Milestones

*Milestones you will reach along the way*

## Frequently asked questions

### What do I do with this space?

*This is a good place to elaborate on your reasoning above -- for example, why did you put the design axioms in the order that you did? It's also a good place to put the answers to any questions that come up during discussion. The expectation is that this FAQ section will grow as the goal is discussed and eventually should contain a complete summary of the points raised along the way.*