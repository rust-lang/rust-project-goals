# Commands

The `cargo rpg` command is a CLI for manipulating and checking project goals. This section provides a reference describing (some of) the ability commands. You can also try `cargo rpg --help` to get a summary.

Note that this relies on the [`gh` client](https://github.com/cli/cli), which needs to be installed and configured with a token (for example using `gh auth login`).

## Available Commands

### `cargo rpg cfp`

Sets up a new Call For Proposals (CFP) period. This command automates the process of creating the necessary directory structure, copying template files, and updating both the SUMMARY.md and README.md files.

```bash
# Basic usage
cargo rpg cfp <timeframe>

# Options
cargo rpg cfp <timeframe> --force    # Force overwrite without asking for confirmation
cargo rpg cfp <timeframe> --dry-run  # Don't make any changes, just show what would be done
```

Example:
```bash
cargo rpg cfp 2025h2
```

Note that after running this command, you'll still need to manually:
1. Prepare and publish a blog post on the Inside Rust blog
2. Send an email to the `all@rust-lang.org` mailing list

For more details, see the [Call for proposals](./cfp.md) documentation.
