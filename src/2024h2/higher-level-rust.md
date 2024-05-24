# Towards a higher level Rust

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

These projects tend to focus on accelerating development in higher level languages. In theory, Rust itself would be an ideal choice for development in the respective spaces. A Rust webserver can be faster and more reliable than a JavaScript webserver. However, Rust's perceived difficulty, verbosity, compile times, and iteration velocity limit its adoption in these spaces. Various language constructs nudge developers to a particular program structure that might be non-obvious at its outset, resulting in slower development times. Other language constructs influence the final architecture of a Rust program, making it harder to migrate one's mental model as they transition to using Rust. Other Rust language limitations lead to unnecessarily verbose or noisy code. While Rust is not necessarily a bad choice for any of these fields, the analogous frameworks (Axum, Bevy, Dioxus, Polars, etc) are rather nascent and frequently butt up against language limitations.

If we could make Rust a better choice for "higher level" programming - apps, games, UIs, webservers, datascience, high-performance-computing, scripting - then Rust would see much greater adoption outside its current bubble. This would result in more corporate interest, excited contributors, and generally positive growth for the language. With more "higher level" developers using Rust, we might see an uptick in adoption by startups, research-and-development teams, and the physical sciences which could lead to more general innovation.

While new languages focused on high-level applications are gaining traction, the thesis of this project goal is that Rust itself can be tweaked to make it a better choice overall.

Generally we believe this boils down two focuses:

- Make Rust programs faster to write
- Shorten the iteration cycle of a Rust program

### The next few steps

The two key areas we've identified as places to cut down on verbosity and make Rust easier to work are:

- Reducing the frequency of an explicit ".clone()" for cheaply cloneable items
- Partial borrows for structs

The key areas we've identified as avenues to speed up iterative development include:

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

Across web, game, UI, app, and even systems development, it's common to share semaphores across scopes. These come in the form of channels, queues, signals, and immutable state typically wrapped in Arc/Rc. In Rust, to use these items across scopes - like `tokio::spawn` or `'static` closures, a programmer must explicitly call `.clone()`. This is frequently accompanied by a rename of an item:

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

#### Faster Unwrap Syntax (Contentious)

Another common criticism of Rust in prototype-heavy programming subfields is its pervasive verbosity - especially when performing rather simple or innocuous transformations. Admittedly, even as experienced Rust programmers, we find ourselves bogged down by the noisiness of various language constructs. In our opinion, the single biggest polluter of prototype Rust codebase is the need to call `.unwrap()` everywhere. While yes, many operations can fail and it's a good idea to handle errors, we've generally found that `.unwrap()` drastically hinders development in higher level paradigms.

Whether it be simple operations like getting the last item from a vec:
```rust
let items = vec![1,2,3,4];
let last = items.last().unwrap();
```

Or slightly more involved operations like fetching from a server:
```rust
let res = Client::new()
	.unwrap()
	.get("https://dog.ceo/api/breeds/list/all")
	.header("content/text".parse().unwrap())
	.send()
	.unwrap()
	.await
	.unwrap()
	.json::<DogApi>()
	.await
	.unwrap();
```

It's clear that `.unwrap()` plays a large role in the early steps of every Rust codebase.

A "higher level Rust" would be a Rust that enables programmers to quickly prototype their solution, iterating on architecture and functionality before finally deciding to "productionize" their code. In today's Rust this is equivalent to replacing `.unwrap()` with proper error handling (or `.expect()`), adding documentation, and adding tests.

Programmers generally understand the difference between prototype code and production code - they don't necessarily need to be so strongly reminded that their code is prototype code by forcing a verbose `.unwrap()` at every corner. In many ways, Rust today feels hostile to prototype code. We believe that a "higher level Rust" should be *welcoming* to prototype code. The easier it is for developers to write prototype code, the more code will likely convert to production code. Prototype code by design is the first step to production ready code.

When this topic comes up, folks will invariably bring up `Result` plus `?` as a solution. In practice, we've not found it to be a suitable bandaid. Adopting question mark syntax requires you to change the signatures of your code at every turn. While prototyping you can no longer think in terms of `A -> B` but now you need to think of every `A -> B?` as a potentially fallible operation. The final production ready iteration of your code will likely not be fallible in every method, forcing yet another level of refactoring. Plus, question mark syntax tends to bubble errors *without* line information, generally making it difficult to locate *where* the error is occurring in the first place. And finally, question mark syntax doesn't work on `Option<T>`, meaning `.unwrap()` or pattern matching are the only valid options.

```rust
let items = vec![1,2,3,4];
let last = items.last().unwrap(); // <--- this can't be question-marked!
```

We don't prescribe any particular solution, but ideally Rust would provide a similar shortcut for `.unwrap()` as it does for `return Err(e)`. Other languages tend to use a `!` operator for this case:

```rust
let items = vec![1,2,3,4];
let last = items.last()!;

let res = Client::new()!
	.get("https://dog.ceo/api/breeds/list/all")
	.header("content/text".parse()!)
	.send()!
	.await!
	.json::<DogApi>()
	.await!;
```


A "higher level Rust" would provide similar affordances to prototype code that it provides to production code. All production code was once prototype code. Today's Rust makes it harder to write prototype code than it does production code. This language-level opinion is seemingly unique to Rust and arguably a major factor in why Rust has seen slower adoption in higher level programming paradigms.


#### Named and Optional Arguments or Partial Defaults (Contentious)

Beyond `.clone()` and `.unwrap()`, the next biggest polluter for "high level" Rust code tends to be the lack of a way to properly supply optional arguments to various operations. This has received lots of discussion already and we don't want to belabor the point anymore than it already has.

The main thing we want to add here is that we believe the builder pattern is *not* a great solution for this problem, especially during prototyping and in paradigms where iteration time is important.

```rust
struct PlotCfg {
   title: Option<String>,
   height: Option<u32>,
   width: Option<u32>,
   dpi: Option<u32>,
   style: Option<Style>
}

impl PlotCfg {
    pub fn title(&mut self, title: Option<u32>) -> &mut self {
        self.title = title;
        self
    }
    pub fn height(&mut self, height: Option<u32>) -> &mut self {
        self.height = height;
        self
    }
    pub fn width(&mut self, width: Option<u32>) -> &mut self {
        self.width = width;
        self
    }
    pub fn dpi(&mut self, dpi: Option<u32>) -> &mut self {
        self.dpi = dpi;
        self
    }
    pub fn style(&mut self, style: Option<u32>) -> &mut self {
        self.style = style;
        self
    }
    pub fn build() -> Plot {
	    todo!()
    }
}
```

A solution to this problem could in any number of forms:
- Partial Defaults to structs
- Named and optional function arguments
- Anonymous structs

We don't want to specify any particular solution:
- Partial defaults simply feel like an extension of the language
- Named function arguments would be a very welcome change for many high-level interfaces
- Anonymous structs would be useful outside of replacing builders

Generally though, we feel like this another core problem that needs to be solved for Rust to see more traction in higher-level programming paradigms.

#### Procedural macro expansion caching or speedup

Today, the Rust compiler does not necessarily cache the tokens from procedural macro expansion. On every `cargo check`, and `cargo build`, Rust will run procedural macros to expand code for the compiler. The vast majority of procedural macros in Rust are idempotent: their output tokens are simply a deterministic function of their input tokens. If we assumed a procedural macro was free of side-effects, then we would only need to re-run procedural macros when the input tokens change. This has been shown in prototypes to drastically improve incremental compile times (30% speedup), especially for codebases that employ lots of derives (Debug, Clone, PartialEq, Hash, serde::Serialize).

A solution here could either be manual or automatic: macro authors could opt-in to caching or the compiler could automatically cache macros it knows are side-effect free.


#### Faster fresh builds

A "higher level Rust" would be a Rust where a programmer would be able to start a new project, add several large dependencies, and get to work quickly without waiting minutes for a fresh compile. A web developer would be able to jump into a Tokio/Axum heavy project, a game developer into a Bevy/WGPU heavy project, or a data scientist into a Polars project and start working without incurring a 2-5 minute penalty. In today's world, an incoming developer interested in using Rust for their next project immediately runs into a compile wall. In reality, Rust's incremental compile times are rather good, but Rust's perception is invariably shaped by the "new to Rust" experience which is almost always a long upfront compile time.

Cargo's current compilation model involves downloading and compiling dependencies on a per-project basis. Workspaces allow you to share a set of dependency compilation artifacts across several projects at once, deduplicating compilation time and reducing disk space usage.

A "higher level Rust" might employ some form of caching - either per-user, per-machine, per-organization, per-library, otherwise - such that fresh builds are just as fast as incremental builds. If the caching was sufficiently capable, it could even cache dependency artifacts at higher optimization levels. This is particularly important for game development, data science, and procedural macros where debug builds of dependencies run *significantly* slower than their release variant. Projects like Bevy and WGPU explicitly guide developers to manually increase the optimization level of dependencies since the default is unusably slow for game and graphics development.

Generally, a "high level Rust" would be fast-to-compile and maximally performant by default. The tweaks here do not require language changes and are generally a question of engineering effort rather than design consensus.


### The "shiny future" we are working towards

A "high level Rust" would be a Rust that has a strong focus on iteration speed. Developers would benefit from Rust's performance, safety, and reliability guarantees without the current status quo of long compile times, verbose code, and program architecture limitations.

A "high level" Rust would:
- Compile quickly, even for fresh builds
- Be terse in the common case
- Produce performant programs even in debug mode
- Provide language shortcuts to get to running code faster

In our "shiny future," an aspiring genomics researcher would:
- be able to quickly jump into a new project
- add powerful dependencies with little compile-time cost
- use various procedural macros with little compile-time cost
- cleanly migrate their existing program architecture to Rust with few lifetime issues
- employ various shortcuts like unwrap to get to running code quicker

----

To imagine a fictional scenario, let's take the case of "Alex."

Alex is a genomics researcher studying ligand receptor interaction to improve drug therapy for cancer. They work with very large datasets and need to design new algorithms to process genomics data and simulate drug interactions. Alex recently heard that Rust has a great genomics library (Genomix) and decides to try out Rust for their next project.

Alex creates a new project and starts adding various libraries. To start, they add Polars and Genomix. They also realize they want to wrap their research code in a web frontend and allow remote data, so they add Tokio, Reqwest, Axum, and Dioxus. They write a simple program that fetches data from their research lab's S3 bucket, downloads it, cleans it, processes, and then displays it.

For the first time, they type `cargo run.` The project builds in 10 seconds and their code starts running. The analysis completes churns for a bit and Alex is greeted with a basic webpage visualizing their results. They start working on the visualization interface, adding interactivity with new callbacks and async routines. Thanks to hotreloading, the webpage updates without fully recompiling and losing program state.

Once satisfied, Alex decides to refactor their messy program into different structs so that they can reuse the different pieces for other projects. They add basic improvements like multithreading and swap out the unwraps for proper error handling.

Alex heard Rust was difficult to learn, but they're generally happy. Their Rust program is certainly faster than their previous Python work. They didn't need to learn JavaScript to wrap it in a web frontend. The `Cargo.toml` is a cool - they can share their work with the research lab without messing with Python installs and dependency management. They heard Rust had long compile times but didn't run into that.

---

Unfortunately, today's Rust provides a different experience.

- Upfront compile times would be in the order of minutes
- Adding new dependencies would also incur a strong compile time cost
- Iterating on the program would take several seconds per build
- Adding a web UI would be arduous with copious calls `.clone()` to shuffle state around
- Lots of explicit unwraps pollute the codebase
- Refactoring to a collection of structs might take much longer than they anticipated


## Design axioms[da]

- Preference for minimally invasive changes that have the greatest potential benefit
- No or less syntax is preferable to more syntax for the same goal
- Prototype code should receive similar affordances as production code
- Attention to the end-to-end experience of a Rust developer
- Willingness to make appropriate tradeoffs in favor of implementation speed and intuitiveness

[da]: ../about/design_axioms.md

## Ownership and other resources

The work here is proposed by Jonathan Kelley on behalf of Dioxus Labs. We have funding for 1-2 engineers depending on the scope of work. Dioxus Labs is willing to take ownership and commit funding to solve these problems.


| Subgoal                             | Owner(s) or team(s)                     | Status            |
| ----------------------------------- | --------------------------------------- | ----------------- |
| `.clone()` problem                  |  [jkelleyrtp] + tbd                     | ![Funded][]       |
| `.unwrap()` problem                 |  [jkelleyrtp] + tbd                     | ![Funded][]       |
| partial borrows                     |  [jkelleyrtp] + tbd                     | ![Funded][]       |
| proc macro expansion caching        |  [jkelleyrtp] + tbd                     | ![Funded][]       |
| global dependency cache             |  [jkelleyrtp] + tbd                     | ![Funded][]       |
| Named/Optional arguments            |  [jkelleyrtp] + tbd                     | ![Funded][]       |

* The ![Funded][] badge indicates that the owner has committed and work will be funded by their employer or other sources.
* The ![Team][] badge indicates a requirement where Team support is needed.

[Funded]: https://img.shields.io/badge/Funded-yellow
[Not funded]: https://img.shields.io/badge/Not%20yet%20funded-red
[Approved]: https://img.shields.io/badge/Approved-green
[Not approved]: https://img.shields.io/badge/Not%20yet%20approved-red
[Complete]: https://img.shields.io/badge/Complete-green
[TBD]: https://img.shields.io/badge/TBD-red
[Team]: https://img.shields.io/badge/Team%20ask-red

### Support needed from the project

- We are happy to author RFCs and/or work with other experienced RFC authors.
- We are happy to host design meetings, facilitate work streams, logistics, and any other administration required to execute. Some subgoals proposed might be contentious or take longer than this goals period, and we're committed to timelines beyond six months.
- We are happy to author code or fund the work for an experienced Rustlang contributor to do the implementation. For the language goals, we expect more design required than actual implementation. For cargo-related goals, we expected more engineering required than design. We are also happy to back any existing efforts as there is ongoing work in cargo itself to add various types of caching.
- We would be excited to write blog posts about this effort. This goals program is a great avenue for us to get more corporate support and see more Rust adoption for higher-level paradigms. Having a blog post talking about this work would be a significant step in changing the perception of Rust for use in high-level codebases.

## Outputs and milestones

### Outputs

*Final outputs that will be produced*

### Milestones

*Milestones you will reach along the way*

## Frequently asked questions

### What do I do with this space?

*This is a good place to elaborate on your reasoning above -- for example, why did you put the design axioms in the order that you did? It's also a good place to put the answers to any questions that come up during discussion. The expectation is that this FAQ section will grow as the goal is discussed and eventually should contain a complete summary of the points raised along the way.*

[jkelleyrtp]: https://github.com/jkelleyrtp
