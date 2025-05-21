This repository runs the Rust open source "project goals" program.

It is structured as an mdbook combined with custom rust code to manage the contents of that mdbook and perform other chores.

The mdbook sources can be found in `src` and in `book.toml`.

The `src/admin` directory in particular contains instructions targeting the people who maintain and run the project goal program. It describes the processes for tasks like beginning a new goals program, authoring goal updates, and so forth.

Many of the tasks in `src/admin` are automated via a utility called `cargo rpg`.

The sources for `cargo rpg` as well as the plugin for the mdbook processor are found in crates located in the `crates` directory. The `crates/README.md` summarizes the role of each crate.