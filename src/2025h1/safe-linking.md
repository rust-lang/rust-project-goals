# Research: How to achieve safety when linking separately compiled code

| Metadata           |                                    |
| :--                | :--                                |
| Point of contact   | @m-ou-se                           |
| Status             | Accepted                           |
| Zulip channel      | N/A                                |
| Tracking issue     | [rust-lang/rust-project-goals#267] |

## Summary

Research what "safety" and "unsafety" means when dealing with separately compiled code, like when loading dynamically linked libraries.
Specifically, figure out how it'd be possible to provide any kind of safety when linking external code (such as loading a dynamic library or linking a separately compiled static library).

## Motivation

Rust has a very clear definition of "safe" and "unsafe" and (usually) makes it easy to stay in the "safe" world.
`unsafe` blocks usually only have to encapsulate very small blocks of which one can (and should) prove soundness manually.

When using `#[no_mangle]` and/or `extern { … }` to connect separately compiled code, however, any concept of safety pretty much disappears.

While it might be reasonable to make some assumptions about (standardized) symbols like `strlen`,
the _unsafe assumption_ that a symbol with the same name will refer to something of the expected signature
is _not_ something that one can _prove at compile time_, but is rather an (hopeful, perhaps reasonable) expectation of
the contents of dynamic libraries available at runtime.

The end result is that for use cases like plugins, we have no option than just `unsafe`ly hoping for the best,
accepting that we cannot perfectly guarantee that undefined behavior is impossible linking/loading a library/a plugin/some external code.

### The status quo

Today, combining separately compiled code (from different Rust projects or different languages)
is done through a combination of `extern "…" fn`, `#[repr(…)]`, `#[no_mangle]`, and `extern {…}`.

Specifically:

1. `extern "…" fn` (which has a bit of a confusing name) is used to specify the _calling convention_ or _ABI_ of a function.

   The default one is the `"Rust"` ABI, which (purposely) has no stability guarantees.
   The `"C"` ABI is often used for its stability guarantees, but places restrictions on the possible signatures.

2. `#[repr(…)]` is used to control _memory layout_.

   The default one is the `Rust` layout, which (purposely) has no stability guarantees.
   The `C` layout is often used for its stability guarantees, but places restrictions on the types.

3. `#[no_mangle]` and `extern {…}` are used to control the _symbols_ used for _linking_.

   `#[no_mangle]` is used for _exporting_ an item under a known symbol,
   and `extern { … }` is used for _importing_ an item with a known symbol.

There have often been requests for a "stable Rust abi" which usually refers to a _calling convention_ and _memory layout_ that is
as unrestrictive as `extern "Rust" fn` and `#[repr(Rust)]`, but as stable as `extern "C" fn` and `#[repr(C)]`.

It seems unlikely that `extern "Rust" fn` and `#[repr(Rust)]` would ever come with stablity guarantees, as allowing for changes when stability is not necessary has its benefits.
It seems most likely that a "stable Rust ABI" will arrive in the form of a _new_ ABI,
by adding some kind of `extern "Rust-stable-v1"` (and `repr`) or similar
(such as `extern "crabi" fn` and `#[repr(crabi)]` [proposed here](https://github.com/rust-lang/rust/pull/105586)),
or by slowly extending `extern "C" fn` and `#[repr(C)]` to support more types (like tuples and slices, etc.).

Such developments would lift restrictions on which types one can use in FFI, but just a stable calling convention and memory layout will do almost nothing for safety,
as linking/loading a symbol (possibly at runtime) with a different signature (or ABI) than expected will still immediately lead to undefined behavior.

### Research question and scope

This research project focusses entirely on point 3 above: symbols and linking.

The main research question is:

_**What is necessary for an alternative for `#[no_mangle]` and `extern { … }` to be safe, with a reasonable and usable definition of "safe"?**_

We believe this question can be answered independently of the specifics of a stable calling convention (point 1) and memory layout (point 2).

[RFC3435 "#[export]" for dynamically linked crates](https://github.com/rust-lang/rfcs/pull/3435) proposes one possible way to provide safety in dynamic linking.
The goal of the research is to explore the entire solution space and understand the requirements and limitations that would apply to any possible solution/alternative.

### The next 6 months

- Assemble a small research team (e.g. an MSc student, a professor, and a researcher/mentor).
- Acquire funding.
- Run this as an academic research project.
- Publish intermediate results as a blog post.
- (After ~9 months) Publish a thesis and possibly a paper that answers the research question.

### The "shiny future" we are working towards

The future we're working towards is one where (dynamically) linking separately compiled code (e.g. plugins, libraries, etc.)
will feel like a first class Rust feature that is both safe and ergonomic.

Depending on the outcomes of the research, this can provide input and design requirements for future (stable) ABIs, and potentially pave the way for
safe cross-language linking.

## Design axioms

- Any design is either fully safe, or makes it possible to encapsulate the unsafety in a way that allows one to prove soundness (to reasonable extend).
- Any design allows for combining code compiled with different versions of the Rust compiler.
- Any design is usable for statically linking separately (pre) compiled static libraries, dynamically linking/loading libraries, and dynamically loading plugins.
- Designs require as little assumptions about the calling convention and memory layout.
  Ideally, the only requirement is that they are stable, which means that the design can be used with the existing `extern "C" fn` and `#[repr(C)]`.

## Ownership and team asks

**Owner:** @m-ou-se and/or @Jdonszelmann

| Task                                        | Owner(s) or team(s)                          | Notes                          |
|---------------------------------------------|----------------------------------------------|--------------------------------|
| Discussion and moral support                | ![Team][] [lang]                             |                                |
| Coordination with university                | @Jdonszelmann                                | Delft University of Technology |
| Acquire funding                             | Hexcat (= @m-ou-se + @Jdonszelmann)          |                                |
| Research                                    | Research team (MSc student, professor, etc.) |                                |
| Mentoring and interfacing with Rust project | @m-ou-se, @Jdonszelmann                      |                                |
| Blog post (author, review)                  | MSc student, @Jdonszelmann, @m-ou-se         |                                |
| Experimental implementation                 | Msc student                                  |                                |
| Lang-team experiment                        | ![Team][] [lang]                             | @nikomatsakis                  |
| Standard reviews                            | ![Team][] [compiler]                         |                                |
| Thesis / Paper                              | Research team (MSc student, professor, etc.) |                                |

## Frequently asked questions

### Is there a university and professor interested in this?

Yes! We've discussed this with a professor at the Delft University at Technology, who is excited and already looking for interested students.
