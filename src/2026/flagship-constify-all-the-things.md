# Constify all the things

## Summary

Richer const generics and compile-time reflection that works without derives.

## Motivation

### The status quo

Rust's compile-time capabilities are powerful but incomplete. `const fn` enables compile-time evaluation, but users repeatedly hit limitations when trying to use it for real work:

* **Const generics only support primitive types.** You can write `Array<T, 4>` but not `Array<T, Dimensions { width: 4, height: 4 }>`. Using associated constants as const parameters (`Buffer<T, T::SIZE>`) also doesn't work. These limitations force workarounds or abandoning const generics entirely.

* **Reflection requires derives on every type.** Libraries for serialization, logging, or reflection need users to add derives for each type they want to work with. This creates ecosystem friction: crate authors must choose which libraries to support, users must add boilerplate for each library, and new libraries can't reach existing types.

* **Proc macros are the only option for code generation.** When you need compile-time code generation, proc macros work but are difficult to debug, can't access type information, and add compile-time overhead.

Extending const generics and adding compile-time reflection would address all three limitations, letting users write normal Rust code that inspects types and runs at compile time.

### What we are shooting for

**Stable by end of 2026:**

* **Const generics accept structs and enums.** Write `Array<T, Dimensions { width: 4, height: 4 }>` and have it work. Use associated constants like `Buffer<T, T::SIZE>`.

**RFC by end of 2026:**

* **Reflection without derives.** An experimental `comptime` reflection system will be available on nightly and validated against libraries like `bevy_reflect` and `facet`. We aim to have an RFC merged defining the stabilization path.

### Key use cases

* **Zero-boilerplate serialization**: `serialize(&my_struct)` works for any struct without derives. The library inspects field types at compile time.

* **Game engine reflection**: Engines like Bevy can inspect any type's structure directly, making `#[derive(Component)]` optional.

* **Dimension-checked numerics**: Libraries encode physical dimensions in const generic structs, providing compile-time verification of unit calculations.

* **Protocol buffer generation**: Libraries work directly with Rust structs at compile time, without external code generators.

## 2026 goals

(((FLAGSHIP GOALS: Constify all the things)))

## Frequently asked questions

### How do these goals relate to each other?

* **Const generics** extends what values can be used as compile-time parameters—foundational for many advanced patterns.

* **Reflection and comptime** provides the ability to inspect types at compile time, enabling libraries that work without derives.

Together they transform what's possible at compile time: const generics let you parameterize by complex values, reflection lets you adapt to any type's structure.

### How does reflection differ from proc macros?

Proc macros operate on syntax (tokens) before type checking. Reflection operates on types during const evaluation. Reflection can see actual type information (field types, sizes, layouts), is debuggable with standard tools, and doesn't require proc-macro compilation overhead.

### Will reflection make derives obsolete?

Not obsolete, but often optional. Derives generate specialized code and can be more efficient. Reflection offers less boilerplate with potentially some runtime cost. Many libraries will likely offer both modes.
