# Seamless C support

| Metadata | |
| --- | --- |
| Owner(s) | *Github usernames or other identifying info for goal owners* |
| Teams | *Names of teams being asked to commit to the goal* |
| Status | Not accepted |

## Motivation

Using C from Rust should be as easy as using C from C++: completely seamless,
as though it's just another module of code. You should be able to drop Rust
code into a C project and start compiling and using it in minutes.

### The status quo

Today, people who want to use C and Rust together in a project have to put
substantial work into infrastructure or manual bindings. Whether by creating
build system infrastructure to invoke bindgen/cbindgen (and requiring the
installation of those tools), or manually writing C bindings in Rust, projects
cannot simply drop Rust code into a C program or C code into a Rust program.
This creates a high bar for adopting or experimenting with Rust, and makes it
more difficult to provide Rust bindings for a C library.

By contrast, dropping C++ code into a C project or C code into a C++ project is
trivial. The same compiler understands both C and C++, and allows compiling
both together or separately. The developer does not need to duplicate
declarations for the two languages, and can freely call between functions in
both languages.

C and C++ are still not the same language. They have different idioms and
common types, and a C interface may not be the most ergonomic to use from C++.
Using C++ from C involves treating the C as C++, such that it no longer works
with a C compiler that has no C++ support. But nonetheless, C++ and C integrate
extremely well, and C++ is currently the easiest language to integrate into an
established C project.

This is the level of integration we should aspire to for Rust and C.

### The next six months

To provide seamless integration between Rust and C, we need a single compiler
to understand both Rust and C. Thus, the first step will be to integrate a C
preprocessor and compiler frontend into the Rust compiler. For at least the
initial experimentation, we could integrate components from LLVM, taking
inspiration from `zig cc`. (In the future, we can consider other alternatives,
including a native Rust implementation. We could also consider components from
c2rust or similar.)

We can either generate MIR directly from C (which would be experimental and
incomplete but integrate better with the compiler), or bypass MIR and generate
LLVM bytecode (which would be simpler but less well integrated).

This first step would provide substantial benefits already: a C compiler that's
always available on any system with Rust installed, that generates code for any
supported Rust target, and that always supports cross-language optimization.

We can further improve support for calling C from Rust. We can support
"importing" C header files, to permit using this support to call external
libraries, and to support inline functions.

### The "shiny future" we are working towards

Once C support is integrated, we can generate type information for C functions
as if they were unsafe Rust functions, and then support treating the C code as
a Rust module, adding the ability to import and call C functions from Rust.
This would not necessarily even require header files, making it even simpler to
use C from Rust. The initial support can be incomplete, supporting the subset
of C that has reasonable semantics in Rust.

We will also want to add C features that are missing in Rust, to allow Rust to
call any supported C code.

Once we have a C compiler integrated into Rust, we can incrementally add C
extensions to support using Rust from C. For instance:
- Support importing Rust modules and calling `extern "C"` functions from
  them, without requiring a C header file.
- Support using `::` for scoping names.
- Support simple Rust types (e.g. `Option` and `Result`).
- Support calling Rust methods on objects.
- Allow annotating C functions with Rust-enhanced type signatures, such as
  marking them as safe, using Rust references for pointer parameters, or
  providing simple lifetime information.

We can support mixing Rust and C in a source file, to simplify incremental
porting even further.

To provide simpler integration into C build systems, we can accept a
C-compiler-compatible command line (`CFLAGS`), and apply that to the C code we
process.

We can also provide a CLI entry point that's sufficiently command-line
compatible to allow using it as `CC` in a C project.

## Design axioms

- **C code should feel like just another Rust module.** Integrating C code into
  a Rust project, or Rust code into a C project, should be trivial; it should
  be just as easy as integrating C with C++.

- **This is not primarily about providing *safe* bindings.** This project will
  primarily make it much easier to access C bindings as unsafe interfaces.
  There will still be value in wrapping these unsafe C interfaces with safer
  Rust interfaces.

- **Calling C from Rust should not require writing duplicate information in Rust**
  that's already present in a C header or source file.

- **Integrating C with Rust should not require third-party tools**.

- **Compiling C code should not require substantially changing the information
  normally passed to a C compiler** (e.g. compiler arguments).

## Ownership and other resources

**Owner:** TODO

### Support needed from the project

* Lang team:
  * Design meetings to discuss design changes
  * RFC reviews
* Compiler team:
  * RFC review

## Outputs and milestones

### Outputs

The initial output will be a pair of RFCs: one for an experimental integration of a C compiler into rustc, and the other for minimal language features to take advantage of that.

### Milestones

- Compiler RFC: Integrated C compiler
- Lang RFC: Rust language support for seamless C integration

## Frequently asked questions
