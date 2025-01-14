# Call for proposals

Each goal milestone corresponds to six months, designated in the format YYYYHN, e.g., 2024H2 or 2025H1. To launch a new goal season, you should get started a month or two before the new season starts:

* For an H1 season, start around mid October of the year before.
* For an H2 season, start around mid April of the year before.

This is the checklist of steps to starting accepting goal proposals:

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
