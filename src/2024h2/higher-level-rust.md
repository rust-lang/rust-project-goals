# Towards a higher level Rust

> **Instructions:** Copy this template to a fresh file with a name based on your plan.
> Update the text. Feel free to replace any text with anything, but there are placeholders
> designed to help you get started. Also, while this template has received some iteration,
> it is not sacrosant. Feel free to change the titles of sections or make other changes that you think
> will increase clarity.

| Metadata | |
| --- | --- |
| Owner(s) | [jkelleyrtp][] |
 Teams    | [Lang], [Compiler]            |
| Status | WIP |

[Lang]: https://www.rust-lang.org/governance/teams/lang
[Compiler]: https://www.rust-lang.org/governance/teams/library#team-compiler

## Motivation

Rust is beginning to pick up momentum in a variety of spaces traditionally deemed "higher level." This includes fields like app, game, and web development as well as data science and and scientific computing. Rust's inherent low-level nature lends itself as a solid foundation for these fields in the form of frameworks and libraries.

However, Rust today isn't a great choice for the consumption of these libraries - many projects expose bindings for languages like Python and JavaScript. The motivation of this project goal is to make Rust a better choice for higher level programming subfields by identifying and remedying language papercuts with minimally invasive language changes.

### The status quo

Rust has recently seen tremendous adoption in a number of high-profile projects. These include but are not limited to: Firecracker, Pingora, Zed, Datafusion, Candle, Gecko, Turbopack, React Compiler, Deno, Tauri, InfluxDB, SWC, Ruff, Polars, SurrealDB, NPM and more. These projects tend to power a particular field of development: SWC, React, and Turbopack powering web development, Candle powering AI/ML, Ruff powering Python, InfluxDB and Datafusion powering data science etc.

These projects tend to focus on accelerating development in higher level languages. In theory, Rust itself would be an ideal choice for development in the respective spaces. A Rust webserver can be faster and more reliable than a JavaScript webserver. However, Rust's perceived difficulty and verbosity limit its adoption in these spaces. Various language constructs nudge developers to a particular program structure that might be nonobvious at its outset, resulting in slower development times. Other language constructs influence the final architecture of a Rust program, making it harder to migrate one's mental model as they transition to using Rust. Other Rust language limitations lead to unnecessarily verbose or noisy code. While Rust is not necessarily a bad choice for any of these fields, the analogous frameworks (Axum, Bevy, Dioxus, Polars, etc) are rather nascent and frequently butt up against language limitations.

If we could make Rust a better choice for "higher level" programming - apps, games, UIs, webservers, datascience, high-performance-computing, scripting - then Rust would see much greater adoption outside its current bubble. This would result in more corporate interest, excited contributors, and generally positive growth for the language. With more "higher level" developers using Rust, we might see an uptick in adoption by startups, research-and-development teams, and the physical sciences which could lead to more general innovation.

While new languages focused on high-level applications are gaining traction, the thesis of this project goal is that Rust itself can be tweaked to make it a better choice overall.

Generally we believe this boils down two focuses:

- Make Rust programs faster to write
- Shorten the iteration cycle of a Rust program

### The next few steps

The two key areas we've identified as places to cut down on verbosity and make Rust easier to work are:

- Reducing the frequency of an explicit ".clone()" for cheaply clonable items
- Partial borrows for structs

The key areas we've identifed as avenues to speed up iterative development include:

- Speeding up or caching proc macro expansion
- A per-user cache for compiled artifacts
- A remote cache for compiled artifacts integrated into Cargo itself

Additional - more contentious - "wants" include:

- A less verbose approach for "unwrap" in prototype code
- Named and/or optional/default function arguments
- Succinct usage of a framework's types without top-level "use" imports (enums, structs)

There are other longer term projects that would be interesting to pursue but don't necessarily fit in the 2024 goals:

- Partial compilation of invalid Rust programs that might not pass "cargo check"
- Hotreloading for Rust programs
- A JIT backend for Rust programs
- An incremental linker to speed up test/example/benchmark compilation for workspaces

---

#### Reducing `.clone()` frequency

Across web, game, UI, app, and even systems development, it's common to share sempaphores across scopes. These come in the form of channels, queues, signals, and immutable state typically wrapped in Arc/Rc. In Rust, to use these items across scopes - like `tokio::spawn` or `'static` closures, a programmer must explicitly call `.clone()`. This is frequently accompanied by a rename of an item:

```rust
let state = Arc::new(some_state);

let _state = state.clone();
tokio::spawn(async move { /*code*/ });

let _state = state.clone();
tokio::spawn(async move { /*code*/ });

let _state = state.clone();
let callback = Callback::new(move |_| { /*code*/ });
```

This can become both noisy - `clone` pollutes a codebase - and confusing - what does `.clone()` imply on this item? Calling `.clone()` could imply an allocation or simply a RefCount increment. In many cases it's not possible to understand the behavior without viewing the `clone` implementation directly.

A higher level Rust would provide a mechanism to cut down on these clones entirely, making code terser and potentially clearer about intent:

```rust
let state = Arc::new(some_state);

tokio::spawn(async move { /*code*/ });

tokio::spawn(async move { /*code*/ });

let callback = Callback::new(move |_| { /*code*/ });
```

While we don't necessarily propose any one solution to this problem, we believe Rust can be tweaked in way that makes these explicit calls to `.clone()` disappear without significant changes to the language.

#### Partial borrows for structs

Another complication programmers run into is when designing the architecture of their Rust programs with structs. A programmer might start with code that looks like this:

```rust
let name = "Foo ";
let mut children = vec!["Bar".to_string()];

children.push(name.to_string())
```

And then decide to abstract it into a more traditional struct-like approach:

```rust
struct Baz {
    name: String,
    children: Vec<String>
}

impl Baz {
    pub fn push_name(&mut self, new: String) {
        let name = self.name()
        self.push(new);
        println!("{name} pushed item {new}");
    }

    fn push(&mut self, item: &str) {
        self.children.push(item)
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}
```

While this code is similar to the original snippet, it no longer compiles. Because `self.name` borrows `Self`, we can't call `self.push` without running into lifetime conflicts. However, semantically, we haven't violated the borrow checker - both `push` and `name` read and write different fields of the struct.


Interestingly, Rust's disjoint capture mechanism for closures, *can* perform the same operation *and* compile.

```rust
let mut modify_something =  || s.name = "modified".to_string();
let read_something =  || &s.children.last().unwrap().name;

let o2 = read_something();
let o1 = modify_something();
println!("o: {:?}", o2);
```

This is a very frequent papercut for both beginner and experience Rust programmers. A developer might design a valid abstraction for a particular problem, but the Rust compiler rejects it even though said design does obey the core axioms of the borrow checker.

As part of the "higher level Rust" effort, we want to reduce the frequency of this papercut, making it easier for developers to model and iterate on their program architecture.

For example, a syntax-less approach to solving this problem might be simply turning on disjoint capture for *private methods only*. Alternatively, we could implement a syntax or attribute that allows developers to explicitly opt in to the partial borrow system. Again, we don't want to necessarily prescribe a solution here, but the best outcome would be a solution that reduces mental overhead with as little new syntax as possible.


#### Procedural macro expansion caching or speedup

Today, the Rust compiler does not necesarily cache the tokens from procedural macro expansion. On every `cargo check`, and `cargo build`, Rust will run procedural macros to expand code for the compiler. The vast majority of procedural macros in Rust are idempotent: their output tokens are simply a deterministic function of their input tokens. If we assumed a procedural macro was free of side-effects, then we would only need to re-run procedural macros when the input tokens change. This has been shown in prototypes to drastically improve incremental compile times (30% speedup), especially for codebases that employ lots of derives (Debug, Clone, PartialEq, Hash, serde::Serialize).

A solution here could either be manual or automatic: macro authors could opt-in to caching or the compiler could automatically cache macros it knows are side-effect free.


#### Faster fresh builds

A "higher level Rust" would be a Rust where a programmer would be able to start a new project, add several large dependencies, and get to work quickly without waiting minutes for a fresh compile. A web developer would be able to jump into a Tokio/Axum heavy project, a game developer into a Bevy/WGPU heavy project, or a data scientist into a Polars project and start working without incurring a 2-5 minute penalty. In today's world, an incoming developer interested in using Rust for their next project immediately runs into a compile wall. In reality, Rust's incremental compile times are rather good, but Rust's perception is invariably shaped by the "new to Rust" experience which is almost always a long upfront compile time.

Cargo's current compilation model involves downloading and compiling dependencies on a per-project basis. Workspaces allow you to share a set of dependency compilation artifacts across several projects at once, deduplicating compilation time and reducing disk space usage.

A "higher level Rust" might employ some form of caching - either per-user, per-machine, per-organization, per-library, otherwise - such that fresh builds are just as fast as incremental builds. If the caching was sufficiently capable, it could even cache dependency artifacts at higher optimization levels. This is particularly important for game development, data science, and procedural macros where debug builds of dependencies run *significantly* slower than their release variant. Projects like Bevy and WGPU explicitly guide developers to manually increase the optimization level of dependencies since the default is unusably slow for game and graphics development.

Generally, a "high level Rust" would be fast-to-compile and maximally performant by default. The tweaks here do not require language changes and are generally a question of engineering effort rather than design consensus.


### The "shiny future" we are working towards

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

[jkelleyrtp]: https://github.com/jkelleyrtp
