# Author updates

## Soliciting updates

Triagebot can ping project-goal owners for updates. To use it, go to Zulip and execute a command like this.

```
@triagebot ping-goals 14 Oct-21
```

The first number (14) is a threshold, it is typically set to the current day of the month (e.g., the above command assumes it is Oct 14). It means "if they have posted a comment in the last 14 days, don't bug them". 

The second string ("Oct-21") is the deadline for updates to be included.

We need to improve this UI.

## Drafting the post

The [blog post template][updates template] includes a longer description of the flagship goals as well as a bullet-point or two for each of the other goals -- more than that and the blog post becomes overwhelming, the goal is mostly to let people find the goal they're interested in and click to read the full details. You can fill out the template by hand but you can also use the `cargo rpg updates` ([documented here](./updates.md)) to prepare a rough draft; the tool uses an LLM currently hosted on AWS Bedrock and hence requires setting up an AWS account.

```bash
> cargo rpg updates
```

Once the draft is prepared, create a hackmd in the rust-project-goals hackmd team, post it to `#project-goals` and apply edits (particularly for the flagship goals). Then open a PR against [the blog.rust-lang.org repository](https://github.com/rust-lang/blog.rust-lang.org).

[updates template]: https://github.com/rust-lang/rust-project-goals/blob/main/templates/updates.hbs