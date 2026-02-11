# `fcp`, create the FCP merge comment

The `cargo rpg fcp` command creates an FCP comment, since the format for the project goals RFC is distinct from other RFCs. 

To use, simply invoke the command with the directory for the goal period, e.g., something like this:

```
> cargo rpg rfc src/2025h1
```

This will emit a comment to stdout that includes the name of each team which has registered asks along with checkboxes for each individual on that team.

To avoid exceeding GitHub's limit of 50 usernames per message, only leads are cited with `@` usernames.