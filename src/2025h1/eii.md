# Externally Implementable Items

| Metadata |                                                                |
| -------- | -------------------------------------------------------------- |
| Owner(s) | [Jonathan Dönszelmann](@jdonszelmann) and [Mara Bos](@m-ou-se) |
| Teams    | [lang], [compiler], [libs]                                     |
| Status   | Proposed                                                       |

## Summary

We intend to implement [Externally Implementable Items](https://github.com/rust-lang/rust/issues/125418) in the compiler.
The plan is to do so in a way that allows us to change the way `#[panic_handler]` and similar attributes are handled,
making these library features instead of compiler built-ins.
We intend to eventually support both statics and functions,
but the priority is at functions right now.

## Motivation

(as per the rfcs[^1][^2][^3] on this):

We have several items in the standard library that are overridable/definable by the user crate.
For example, the (no_std) `panic_handler`, the global allocator for `alloc`, and so on.

Each of those is a special lang item with its own special handling.
Having a general mechanism simplifies the language and makes this functionality available for other crates,
and potentially for more use cases in `core`/`alloc`/`std`.

In general, having externally implementable items be feature of the language instead of magic lang items and linker hacks gives more flexibility.
It creates a standard interface to expose points where libraries can be customized.

Additionally, making externally implementable items a language feature makes it easier to document these points of customization.
They can become part of the public api of a crate.

[^1]: https://github.com/rust-lang/rfcs/pull/3632
[^2]: https://github.com/rust-lang/rfcs/pull/3635
[^3]: https://github.com/rust-lang/rfcs/pull/3645

### The status quo

Today, "externally implementable items" exist in various forms that each have their own implementation.
Examples are the `#[panic_handler]`, the global allocator, the global logger of the `log` crate, and so on.
Some of these are magical lang items, whereas others need to be set at runtime or are done (unsafely) through a global (`#[no_mangle]`) linker symbol.

After [RFC 3632], which proposes a new syntax for externally implementable _functions_,
several alternative ideas were proposed in rapid succession
that focussing on [statics](https://github.com/rust-lang/rfcs/pull/3635), [traits](https://github.com/rust-lang/rfcs/pull/3645), and [impl blocks](https://github.com/rust-lang/rfcs/pull/3632) rather than function definitions.
Each of these having rougly equivalent power, but using a different part of Rust to achieve it.

The lang team agreed that this is a problem worth solving, and accepted it as a _lang experiment_.[^4]

While working on implementing possible solutions,
we concluded that it'd be better to make use of attributes rather than new syntax, at least for now.[^5]

Because this requires support for name resolution in attributes, this led to a big detour:
refactoring how attributes are implemented and handled in rustc.[^6]
The main part of that is now merged[^7], allowing us to finally continue on implementing the externally implementable items experiment itself.

[^4]: https://github.com/rust-lang/rfcs/pull/3632#issuecomment-2125488373
[^5]: https://github.com/rust-lang/rust/issues/125418#issuecomment-2360542039
[^6]: https://github.com/rust-lang/rust/issues/131229
[^7]: https://github.com/rust-lang/rust/pull/131808

### The next 6 months

The goal for the next six months is to finish the implementation of externally implementable items (as an experimental feature).

It is not unthinkable that we run into more obstacles that requires some changes in the compiler,
but we estimate that six months is enough to make the feature available for experimentation.

### The "shiny future" we are working towards

In the longer term, this feature should be able to replace the magic behind the panic handler, global allocator, oom handler, and so on.
At that point, an attribute like `#[panic_handler]` would simply be a regular (externally implementable) item exported by `core`, for example.

After stabilization, other crates in the ecosystem, such as the `log` crate, should be able to make use of this as well.
E.g., they could have a `#[log::global_logger]` item that can be used to provide the global logger.

In the longer term, this could enable more fine grained customization of parts of `core`, `alloc` and `std`, such as panic handling.
For example, right now, all kind of panics, including overflows and out-of-bounds panics, can all only be handled
through the `#[panic_handler]`, which will only get a panic message containing an (english) description of the problem.
Instead, one could imagine having a `#[panic_oob_handler]` that gets the index and size as arguments,
allowing one to customize the default behavior.

## Design axioms

The experimental feature we implement should:

- be able to replace how `#[panic_handler]` and global allocator features are implemented.
  - This means the feature should not have a higher (memory, performance, etc.) cost than how those features are currently implemented.
  - This also puts some requirements on the supported functionality, to support everything that those features currently support.
    (E.g., being able to provide a default implementation that can be overridden later.)
- be ergonomic.
  - This means that mistakes should not result in confusing linker errors, but in reasonable diagnostics.
- allow for semver-compatible upgrade paths.
  - E.g. if a crate wants to change the signature or kind of an externally implementable item,
    it should be possible to have some backwards-compatible path forward.
- be as close to zero-cost as possible.
  - E.g. adding the option for more fine grained panic handlers should not result in a loss of performance.

## Ownership and team asks

**Owner:** [Jonathan Dönszelmann](@jdonszelmann) and [Mara Bos](@m-ou-se)

TODO

## Frequently asked questions

- None yet.
