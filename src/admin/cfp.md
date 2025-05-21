# Call for proposals

Each goal milestone corresponds to six months, designated in the format YYYYhN, e.g., 2024h2 or 2025h1. To launch a new goal season, you should get started a couple of months before the new season starts:

* For an H1 season, start around mid October of the year before.
* For an H2 season, start around mid April of the year before.

## Using the automated setup command

The easiest way to set up a new Call For Proposals (CFP) period is to use the `cargo rpg cfp` command. This command automates the process of creating the necessary directory structure, copying template files, and updating both the SUMMARY.md and README.md files.

```bash
# Basic usage
cargo rpg cfp 2025h2

# Force overwrite without asking for confirmation
cargo rpg cfp 2025h2 --force

# Dry run - don't make any changes, just show what would be done
cargo rpg cfp 2025h2 --dry-run
```

The command will:
1. Create a new directory for the specified timeframe (e.g., `src/2025h2/`)
2. Copy and process template files from `src/admin/samples/` to the new directory
3. Update the `SUMMARY.md` file to include the new timeframe section
4. Update the main `README.md` with information about the new timeframe

## Manual steps required

After running the `cargo rpg cfp` command, there are still important manual steps that need to be completed:

### 1. Prepare and publish a blog post

You need to prepare a Call For Proposals blog post on the [Inside Rust] blog:
* Use [this sample](./samples/cfp.md) as a starting point
* Copy the sample to the `blog.rust-lang.org` repository as a new post
* Replace placeholders like `YYYYHN` with the actual timeframe (e.g., `2025H2`)
* We use Inside Rust and not the Main blog because the target audience is would-be Rust contributors and maintainers

### 2. Email the mailing list

Send an email to the `all@rust-lang.org` mailing list to announce the Call For Proposals:
* Include a link to the blog post
* Summarize the key dates and process
* Encourage team participation and feedback
* This step is crucial for ensuring all Rust team members are aware of the upcoming goal period

## Manual setup checklist

If you prefer to set up the CFP manually, or need to customize the process beyond what the automated command provides, here's a checklist of steps:

* [ ] Prepare a Call For Proposals blog post on the [Inside Rust] blog based on [this sample](./samples/cfp.md).
    * We use Inside Rust and not the Main blog because the target audience is would-be Rust contributors and maintainers.
* [ ] Update the [main README page](../README.md) to indicate that the next round of goals is begin accepted.
    * [Sample text to include.](./samples/main-readme.md)
* [ ] Create a new directory `src/YYYYhN`, e.g., `src/2025h1`, with the following files. Note that the sample files below include `<!-- XXX -->` directives that are detected by the [mdbook plugin](./mdbook_plugin.md) and replaced with appropriate content automatically.
    * A `src/YYYYhN/README.md` file that contains the draft RFC.
        * [You can start with this sample file.](./samples/rfc.md)
    * A `src/YYYYhN/goals.md` file containing the draft goal listing.
        * [You can start with this sample file.](./samples/goals.md)
    * A `src/YYYYhN/not_accepted.md` file containing the list of goals that were not accepted.
        * [You can start with this sample file.](./samples/not_accepted.md)
* [ ] Modify SUMMARY.md to include your new milestone with some text like what is shown below.

Sample `SUMMARY.md` comments from 2025H1:

```
# ⏳ 2025H1 goal process

- [Overview](./2025h1/README.md)
- [Proposed goals](./2025h1/goals.md)
- [Goals not accepted](./2025h1/not_accepted.md)
```

[Inside Rust]: https://blog.rust-lang.org/inside-rust/

## Receiving PRs

*to be written*
