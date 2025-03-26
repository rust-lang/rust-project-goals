# Summarize updates for the monthly blog post

Usage:

```
> cargo rpg updates --help
```

Run the `cargo rpg updates` command to create the blog post template. If running from within vscode, the `--vscode` command will open the result in a fresh tab, which is convenient. Otherwise, use `--output-file $file.md` to create a new file.

The template will be filled in with the list of flagship goals. Each flagship goal will have their [Why this goal?](./merge_rfc.md#author-the-why-this-goal-sections-for-the-flagship-goals) section auto-inserted from the corresponding tracking issue.

The template will also include the detailed list of updates in a `<details>` section as well as any TL;DR comments left by users.

The update template itself is maintained with handlebars, you will find it [here](https://github.com/rust-lang/rust-project-goals/blob/main/templates/updates.hbs).


This command can also take optional dates to control which comments and updates in the given date range are included in the blog post. This is usually needed to correctly set the starting date right after the previous month's blog post.

For example,

```
> cargo rpg updates 2025h1 2025-03-01
```
