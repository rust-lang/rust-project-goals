# Author updates

## Soliciting updates

Triagebot can ping project-goal owners for updates. To use it, go to Zulip and execute a command like this (you need to use an `@` in front of triagebot).

```
triagebot ping-goals 14 Oct-21
```

The first number (14) is a threshold, it is typically set to the current day of the month (e.g., the above command assumes it is Oct 14). It means "if they have posted a comment in the last 14 days, don't bug them". 

The second string ("Oct-21") is the deadline for updates to be included.

We need to improve this UI.

## Filling out the template

Run the `cargo rpg updates` command to create the blog post template. If running from within vscode, the `--vscode` command will open the result in a fresh tab, which is convenient. Otherwise, use `--output-file $file.md` to create a new file.

The template will be filled in with the list of flagship goals. Each flagship goal will have their [Why this goal?](./merge_rfc.md#author-the-why-this-goal-sections-for-the-flagship-goals) section auto-inserted from the corresponding tracking issue.

The template will also include the detailed list of updates in a `<details>` section as well as any TL;DR comments left by users.

The update template itself is maintained with handlebars, you will find it [here](https://github.com/rust-lang/rust-project-goals/blob/main/templates/updates.hbs).



