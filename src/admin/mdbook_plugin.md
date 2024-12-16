# Mdbook Plugin

The mdbook is controlled by the `mdbook-goals` plugin in this repo.
This plugin makes various edits to the source:

* Linking usernames like <code>&#x40;foo</code> to their github page and replacing them with their display name.
* Linking GH references lke rust-lang/rust#123.
* Collating goals, creating tables, etc.

The plugin can also be used [from the commmand line](./commands.md).

## Expected book structure

The plugin is designed for the book to have a directory per phase of the goal program, e.g., `src/2024h2`, `src/2025h1`, etc.
Within this directory there should be:

* A `README.md` file that will contain the draft slate RFC.
* One file per goal. Each goal file must follow the [TEMPLATE](../TEMPLATE.md) structure and in particular must have
    * a metadata table in its first section
    * a [Summary section](../TEMPLATE.md#summary)
    * a [Ownership and team asks](../TEMPLATE.md#ownership-and-team-asks) containing the subgoal table
* One file per "phase" of the program, e.g., `proposed.md` etc. (These are not mandatory.)

## Plugin replacement text

The plugin will replace the following placeholder texts.
Each placeholder is enclosed within an html comment `<!-- -->`.

### Goal count

The placeholder <code>&lt;-- #GOALS --&gt;</code> will be replaced with the total number of goals under consideration
(this count excludes goals with the status `Not accepted`).

### Goal listing

The placeholder <code>&lt;-- GOALS '$Status' --&gt;</code> will insert a goal table listing goals of the given status `$Status`, e.g., <code>&lt;-- GOALS 'Flagship' --&gt;</code>. You can also list multiple status items, e.g., <code>&lt;-- GOALS 'Accepted,Proposed' --&gt;</code>