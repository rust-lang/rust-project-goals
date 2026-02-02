# Interactive cargo-tree: TUI for Cargo's dependency graph visualization

| Metadata             |                    |
| :------------------- | ------------------ |
| Point of contact     | @orhun             |
| Status               | Proposed           |
| Flagship             | Building blocks    |
| Tracking issue       |                    |
| Other tacking issues | [#11213], [#15473] |
| Zulip channel        | [TUI for Cargo]    |

## Summary

The goal is to build an interactive terminal UI for Cargo's dependency graph visualization based on the [`cargo-tree`] command.

## Motivation

Cargo's tree command is one of the crucial tools for Rust developers to understand their project's dependency graph, debugging version conflicts and analyzing the features used in dependencies. While the current command-line interface exposes this information accurately, it is still difficult to explore large or complex dependency graphs due to the lack of interactivity. For example, users pipe the output of [`cargo-tree`] to other tools like `less` or `grep` to filter or search for specific dependencies, which can be cumbersome and time-consuming. Also, as the Rust ecosystem continues to grow, the complexity of dependency graphs is increasing, making this workflow even more challenging.

This project goal aims to improve the experience of exploring dependency graphs by introducing an interactive mode for the [`cargo-tree`] command. This interactive terminal UI will allow users to perform dependency graph operations using key bindings. These operations will be aligned with the existing command-line interface of [`cargo-tree`], such as focusing on a specific dependency, but will not be limited to them. With the additional capabilities and visual improvements provided by this TUI, users will be able to navigate and analyze their dependency graphs more effectively.

### The status quo

The primary audience for this work is Rust developers working on medium to large projects, especially those dealing with:

- deep dependency trees with many transitive dependencies,
- version conflicts and duplicate dependencies,
- complex feature sets across multiple dependencies,
- and different dependency types in a workspace context.

Today, these developers struggle to efficiently use the existing [`cargo-tree`] CLI due to:

1. The static output: once it's printed, it cannot be interactively explored.
2. Linear navigation: large trees require scrolling or piping to a pager.
3. Iterative workflow: users frequently re-run the command with different flags (`--invert`, `--edges`, `--target`, etc.) to get answers.

These struggles were reflected in multiple issues ([#11213], [#15473]) and the discussions suggested that an interactive terminal UI could address these pain points.

### The next 6 months

The goal for the next 6 months is to design and implement an interactive terminal UI for dependency graph visualization as an alternative to the existing [`cargo-tree`] command. In the long term, this could be shipped as a part of Cargo and would be invoked with a command-line flag, but the initial prototyping and validation will be done externally in the [cargo-tree-tui] repository.

| Task                                   | Owner(s) | Notes                                                |
| -------------------------------------- | -------- | ---------------------------------------------------- |
| Implement basic interactive tree       | @orhun   | already done in [cargo-tree-tui]                     |
| Match [`cargo-tree`] semantics         | @orhun   | [cargo-tree-tui#4]                                   |
| Improve the UX & styling               | @orhun   | [cargo-tree-tui#8]                                   |
| Directly depend on cargo as dependency | @orhun   | There might be some upstream APIs need to be changed |
| Optimize for performance               | @orhun   | [cargo-tree-tui#37]                                  |
| Prepare for inclusion in Cargo         | @orhun   | Integration and documentation work                   |

### The "shiny future" we are working towards

The long-term goal is to make it easier to explore dependency graphs and letting Rust developers gain insights about their dependencies more effectively.

In this future, it will be easier and quicker to get answers to questions like:

- Why is this dependency present? What is bringing it in? / What is it bringing in?
- Where is the version conflict happening? / How can I resolve it?
- Why is my dependency graph so large? / How can I reduce it?

without repeatedly re-running commands or restructuring their queries.

In the distant future, implementing this would open up possibilities for adding an interactive mode for other commands in Cargo as well.

## Team asks

| Team    | Support level | Notes                                                         |
| ------- | ------------- | ------------------------------------------------------------- |
| [cargo] | small         | Alignment on direction, possible integration help and review. |

## Frequently asked questions

n/A

[#11213]: https://github.com/rust-lang/cargo/issues/11213
[#15473]: https://github.com/rust-lang/cargo/issues/15473
[TUI for Cargo]: https://rust-lang.zulipchat.com/#narrow/channel/246057-t-cargo/topic/TUI.20for.20cargo
[`cargo-tree`]: https://doc.rust-lang.org/cargo/commands/cargo-tree.html
[cargo-tree-tui]: https://github.com/orhun/cargo-tree-tui
[cargo-tree-tui#4]: https://github.com/orhun/cargo-tree-tui/issues/4
[cargo-tree-tui#8]: https://github.com/orhun/cargo-tree-tui/issues/8
[cargo-tree-tui#37]: https://github.com/orhun/cargo-tree-tui/issues/37
