# Wasm Components

| Metadata         |              |
| :--------------- | ------------ |
| Point of contact | @yoshuawuyts |
| Status           | Proposed     |
| Tracking issue   |              |
| Zulip channel    | N/A          |
| Stabilization    | true         |

## Summary

In 2026 we want to improve the state of Wasm Component support in Rust. This
means adding and stabilizing three new compiler targets, as well as begin
experimentation with Wasm-specific language features.

## Motivation

Rust has been at the forefront of WebAssembly support since WebAssembly was
incepted. The WASI 0.3 specification is expected to be released in early 2026,
adding native support for async operations to Wasm Components. You can think of
this as morally equivalent to support for `async fn` and `async gen fn` at the
ABI level.

Native support for async operations is not only great for networked applications
and plugins, but also specifically for targeting the web. Browser vendors have
expressed interest in natively supporting components, which opens up the
possibility for calling web APIs directly from Rust without having to roundtrip
through JavaScript.

We want to make sure that Rust remains at the forefront of WebAssembly support,
and so during 2026 we want to do the work to make sure that stays the case.

### The status quo

There are a number of things with WebAssembly in Rust that are not ideal:

- We don't have a dedicated web target in Rust, instead the ecosystem primarily
  relies on the `wasm32-unknown-unknown` target which is rather brittle.
- We have a WASI 0.3 target but it is currently tier 3, which means its unavailable via `rustup`
- `std::thread` support is missing in all of the Wasm Component targets, which makes certain apps hard to port.
- Creating bare Wasm Components without WASI support uses the WASI targets, which is confusing.
- Exporting and importing WebAssembly types and functions requires third-party bindgen tooling, introducing friction.
- There is only one possible representation for an "async fn main" in WASI 0.3 and above, yet we require third-party crates to call it.

### The next 12 months

> *Sketch out the specific things you are trying to achieve in this goal period. This should be short and high-level -- we don't want to see the design!*

| Task                                                | Owner(s)      | Notes                                                                                                                                                          |
| --------------------------------------------------- | ------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bring `wasm32-wasip3` to tier 2                     | @alexcrichton | This is the WASI 0.3 target, with support for component model async features. It unlocks async-async component composition, as well as cooperative threading.  |
| Introduce `wasm32-component` target as tier 3       | @alexcrichton | This target is very similar to `wasm32-wasip3`, but does not include the WASI APIs, making it `#[no_std]` + `alloc`.                                           |
| Introduce `wasm32-component-web` target as tier 3   | @yoshuawuyts  | This target is very similar to `wasm32-component`, but with a different `cfg` that enables an ecosystem to be built specifically targeting the web.            |
| Implement `std::thread` for WASIp3                  | @tartanllama  | This is a future extension to the component model, which will be part of a WASI 0.3.x release. Enables Wasm Components' "cooperative multi-threading" feature. |
| Implement `async fn main` for WASIp3                | @yoshuawuyts  |                                                                                                                                                                |
| Experiment with `#[repr(wasm)]` and `extern "wasm"` | tbd           | This item is more aspirational than the others. It would be good to do this, but we may not get to it.                                                         |


### The "shiny future" we are working towards

We would like to bring Rust up to date with async support for Wasm Components.
This means adding a new WASI target (`wasm32-wasip3`) and bringing it to tier 2.
But also adding new targets for bare components with optional async support
(`wasm32-component`) and a variation of that specifically for the web
(`wasm32-component-web`).

WebAssembly's goal is to provide a general-purpose, lightweight, and sandboxed
plugin model. Not just on the web, but also for databases, servers, embedded,
and more. We want Rust to provide a best-in-class experience for WebAssembly,
where targeting Wasm is as easy as targeting any of the native platforms. And in
some cases even exceeding what other platforms are capable of, by leaning into
WebAssembly's distinct advantages.

This project goal brings us closer towards that shiny future, making sure we
stay up to date with the latest WebAssembly developments. While also looking to
push a little beyond that, by making WebAssembly Components easier to target
from Rust.

## Team asks

| Team       | Support level | Notes                                                                                                                  |
| ---------- | ------------- | ---------------------------------------------------------------------------------------------------------------------- |
| [compiler] | small         | New targets will need review and approval                                                                              |
| [lang]     | small         | Experimentation with native Wasm features will need approval. May become "medium" if we are somehow really successful. |
| [libs]     | small         | Threading support will need review                                                                                     |

## Frequently asked questions

### What do I do with this space?

*This is a good place to elaborate on your reasoning above -- for example, why did you put the design axioms in the order that you did? It's also a good place to put the answers to any questions that come up during discussion. The expectation is that this FAQ section will grow as the goal is discussed and eventually should contain a complete summary of the points raised along the way.*
