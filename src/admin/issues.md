# Creating tracking issues

Usage:

```
> cargo run -- issues
```

The `issues` command is used to create tracking issues at the start of a project goal session. When you first run it, it will simply tell you what actions it plans to take.

To actually commit and create the issues, supply the `--commit` flag:

```
> cargo run -- issues --commit
```

This will also edit the goal documents to include a link to each created tracking issue. You should commit those edits.

You can later re-run the command and it will not repeat actions it has already taken.c