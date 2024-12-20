# Report status

Every accepted project goal has an associated tracking issue. These are created [automatically by the project-goals admin tool](../admin/issues.md). Your job as a project goal point of contact is to provide regular status updates in the form of a comment indicating how things are going. These will be collected into regular blog posts on the Rust blog as well as being promoted in other channels.

## Updating the progress bar

When we display the status of goals, we include a progress bar based on your documented plan. We recommend you keep this up to date. You can mix and match any of the following ways to list steps.

### Checkboxes

The first option is to add checkboxes into the top comment on the tracking issue. Simply add boxes like `* [ ]` or `* [x]` for a completed item. The tool will count the number of checkboxes and use that to reflect progress. Your tracking issue will be pre-propulated with checkboxes based on the goal doc, but feel free to edit them.

Best practice is to start with a high level list of tasks:

```
* [ ] Author code
* [ ] Author RFC
* [ ] Accept RFC
* [ ] Test
* [ ] Stabilize
```

each time you provide a status update, check off the items that are done, and add new items with more detailed to-do items that represent your next steps.

### Search queries

For larger project goals, it can be more convenient to track progress via github issues. You can do that by removing all the checkboxes from your issue and instead adding a "Tracked issues" line into the metadata table on your tracking issue. It should look like this:

```
| Metadata      | |
| --------      | --- |
| Point of contact | ... |
| Team(s)       | ... |
| Goal document | ... |
| Tracked issues | [rust-lang/rust label:A-edition-2024 label:C-tracking-issue -label:t-libs](...) |
```

The first 3 lines should already exist. The last line is the one you have to add. The "value" column should have a markdown link, the contents of which begin with a repo name and then search parameters in Github's format. The tool will conduct the search and count the number of open vs closed issues. The `(...)` part of the link should be to github so that users can click to do the search on their own.

You can find an example on the [Rust 2024 Edition tracking issue](https://github.com/rust-lang/rust-project-goals/issues/117).

### Use "See also" to refer to other tracking issues

If you already have a tracking issue elsewhere, just add a "See also" line into your metadata. The value should be a comma-or-space-separated list of URLs or `org/repo#issue` github references:

```
| Metadata      | |
| --------      | --- |
| Point of contact | ... |
| Team(s)       | ... |
| Goal document | ... |
| See also | rust-lang/rust#123 |
```

We will recursively open up the "see also" issue and extract checkboxes (or search queries / see-also tags) from there.

### Binary issues

If we don't find any of the above, we will consider your issue either 0% done if it is not yet closed or 100% done if it is.

## Status update comments

Status updates are posted as comments on the Github tracking issue. You will receive regular pings on Zulip to author status updates periodically. It's a good idea to take the opportunity to update your [progress checkboxes](#checkboxes) as well. 

There is no strict format for these updates but we recommend including the following information:

* What happened since the last update? Were any key decisions made or milestones achieved?
* What is the next step to get done?
* Are you blocked on anyone or anything?
* Is there any opportunity to others to pitch in and help out? 

## Closing the issue

Closing the tracking issue is a signal that you are no longer working on it. This can be because you've achieved your goal or because you have decided to focus on other things. Also, tracking issues will automatically be closed at the end of the project goal period.

When you close an issue, the state of your [checkboxes](#checkboxes) makes a difference. If they are 100% finished, the goal will be listed as completed. If there are unchecked items, the assumption is that the goal is only partly done, and it will be listed as unfinished. So make sure to check the boxes if the goal is done!