# Author updates

## Soliciting updates

Triagebot can ping project-goal owners for updates. To use it, go to Zulip and execute a command like this (you need to use an `@` in front of triagebot).

```
@triagebot ping-goals 14 Oct-21
```

The first number (14) is a threshold, it is typically set to the current day of the month (e.g., the above command assumes it is Oct 14). It means "if they have posted a comment in the last 14 days, don't bug them".

The second string ("Oct-21") is the deadline for updates to be included.

We need to improve this UI.

## Filling out the template

After the updates have been published, they can be summarized in a monthly blog post, as described in [this dedicated chapter](./updates.md).
