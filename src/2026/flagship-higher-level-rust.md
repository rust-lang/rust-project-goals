# Higher-level Rust

## Summary

Making Rust more approachable for quick tasks, prototyping, and application development without sacrificing its core values.

## Motivation

### The status quo

Rust is capable of high-level application development, but current tooling creates friction for developers who want to use Rust for quick tasks or prototyping:

* **Starting a program requires project setup.** Even a simple utility needs a project directory, `Cargo.toml`, and `main.rs`. Compare this to Python or JavaScript, where you write one file and run it. Many developers who would benefit from Rust's type system instead reach for scripting languages because the setup cost is too high.

* **Ref-counted types require verbose cloning.** `Rc` and `Arc` require explicit `.clone()` calls and awkward temporary variables when sharing data across closures or async blocks. This ceremony slows iteration and clutters code. Frameworks like Dioxus and tokio-based services rely heavily on shared ownership, and the ergonomic cost has led projects to create custom preprocessors or arena-based designs as workarounds.

The result is a perception gap: Rust seems like "the language for when performance matters" rather than "a language for getting things done."

### What we are shooting for

By the end of 2026:

* **Single-file programs run directly.** Write a Rust file with embedded dependencies, make it executable, and run it. No project setup, no separate manifest. Useful for bug reports, teaching, utilities, and prototyping.

* **Ref-counting ergonomics are prototyping on nightly.** A `Share` trait and move expressions will be available on nightly, providing better syntax for cloning into closures. The code remains explicit about where sharing happens, but the boilerplate disappears. Stabilization may follow pending evaluation.

### Key use cases

* **Bug reports**: Share a single runnable file that anyone can execute. No repository setup required.

* **Teaching**: Provide complete examples in a single file. Students experiment immediately without learning project structure first.

* **Personal utilities**: Write small tools in Rust instead of shell scripts. A single-file script with type checking and access to crates.io.

* **Application prototyping**: Focus on the interesting parts when exploring GUI or async designs, rather than fighting clone ergonomics.

* **Production async code**: The "Cloudflare pattern" of cloning multiple values before spawning becomes a single readable expression.

### Design axioms

* **Lower the floor without lowering the ceiling.** Make simple things simple without compromising Rust's power for complex cases.

* **Explicit is ergonomic.** Visibility into what code does remains important. Remove unnecessary ceremony, not behavior. When you share an `Arc`, the code should say so—just without the boilerplate.

* **Meet developers where they are.** Not every task requires a project structure. Rust should adapt to what the developer is building.

## 2026 goals

(((FLAGSHIP GOALS: Higher-level Rust)))

## Frequently asked questions

### How do these goals relate to each other?

* **Cargo script** addresses the friction of *starting* a Rust program—removing project setup ceremony.

* **Ergonomic ref-counting** addresses friction in *writing* certain programs—particularly async code and GUI applications.

Together they make Rust more approachable: easier to start, more pleasant to write.

### Does this mean Rust is becoming a scripting language?

No. Rust remains compiled with static types and ownership semantics. Cargo scripting adds convenience for single-file programs while keeping everything that makes Rust valuable. The difference is purely in developer experience.

### Will these changes affect existing code?

No. Cargo scripting is an addition to how Cargo can be invoked. Ergonomic ref-counting features are opt-in.
