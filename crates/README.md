The support crates for running the rust-project-goals program.

The main crates of interest are:

* `mdbook-goals`, which is an mdbook plugin that processes the project goal markdown files found in `../src` to populate indices and other content dynamically.
* `rust-project-goals-cli`, which contains the main helper tool, invoked with `cargo rpg` (there is an alias defined in the `.cargo` directory).
    * The `rust-project-goals-*` crates are dependencies of `rust-project-goals-cli` for other more specialized tasks.
* The `rust-project-goals` crate is a library used by `mdbook-goals` and by the CLI tool to encapsulate common tasks like scanning for goal documents.
