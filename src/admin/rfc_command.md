# `rfc`, export RFC text

The `cargo rpg rfc` command exports RFC text suitable for inclusion in the rust-lang/rfcs repository. It is intended for use when [preparing the RFC](./prepare_rfc.md) at the start of a goal period.

To use, simply invoke the command with the directory for the goal period, e.g., something like this:

```
> cargo rpg rfc src/2025h1
```

This will read the README.md file and dump a version to stdout that can be copy-and-paste. This version will have URLs adjusted to point at the rust-lang/rust-project-goals repository and other cosmetic changes.