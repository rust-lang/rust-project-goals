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

The goal for a-mir-formality this year is to get in place to perform minimal modeling:

* ability to run simple `.rs` files as tests (to be contributed by nikomatsakis)
* ability to run 

### The "shiny future" we are working towards

The eventual goal is for a-mir-formality to serve as the official model of how the Rust type system works.

## Design axioms

**

## Ownership and other resources

**Owner:** nikomatsakis



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