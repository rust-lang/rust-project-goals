# Process-Safe Rustup Toolchain Operations

| Metadata         |                                                                       |
| ---------------- | --------------------------------------------------------------------- |
| Point of contact | @rami3l                                                               |
| Status           | Proposed                                                              |
| What and why     | Allow concurrent rustup instances to safely manage the same toolchain |
| Tracking issue   |                                                                       |
| Zulip channel    | [#t-rustup]                                                           |

[#t-rustup]: https://rust-lang.zulipchat.com/#narrow/channel/490103-t-rustup

## Summary

This project goal aims to make rustup "process safe" in toolchain operations,
i.e. make it safe for multiple rustup instances to operate simultaneously on
the same toolchain, by introducing a new transaction mechanism to
rustup-installed toolchains and their modifying actions.

## Motivation

In rustup's list of open issues, [rust-lang/rustup#988] is a longstanding one.
The problem is very simple: rustup as it currently stands has no defense
mechanisms against concurrent changes to the same toolchain.

It should be emphasized that this is not an arcane edge case, but a quite
common source of frustration for regular users.

Notably, at the moment of writing, rustup's [implicit installation] of the
active toolchain is still enabled by default for proxy binaries like `cargo`.
This means e.g. running `cargo build` twice on the same Rust project directory
with no active toolchain installed beforehand may result in two concurrent
rustup processes trying to install the same toolchain, and this is before
`cargo` is actually invoked. As a result, the user might end up with broken
toolchain installations.

To make things worse, one of those `cargo` invocations is commonly performed by
`rust-analyzer` from the IDE, and the user might not even be aware of the
situation when it happens.

[implicit installation]: https://blog.rust-lang.org/inside-rust/2026/07/03/rustup-update-1.30/#refining-the-implicit-installation-behavior

### The status quo

When installing/updating/uninstalling a toolchain, a "transaction" is being
used. Quoting the doc string of the `Transaction` type:

> A Transaction tracks changes to the file system, allowing them to
> be rolled back in case of an error. Instead of deleting or
> overwriting file, the old copies are moved to a temporary
> folder. If the transaction is rolled back, they will be moved back
> into place. If the transaction is committed, these files are
> automatically cleaned up using the temp system.
>
> All operations that create files will automatically create any
> intermediate directories in the path to the file if they do not
> already exist.
>
> All operations that create files will fail if the destination
> already exists.

It is clear that this approach assumes rustup's full control of the
installation directory, namely `$RUSTUP_HOME` and its subdirectories as well as
the "bindir" (currently `$CARGO_HOME/bin`), but this assumption is flawed in
that it has completely ignored the possible existence of another rustup
process.

When two concurrently-running rustup processes try to install/update/uninstall
the same target toolchain, clashes will inevitably happen and certain failures
will occur. The possible symptoms are quite diverse, but the most typical error
message is along these lines ([rust-lang/rustup#4465]):

> ```console
> [..]
> info: installing component 'cargo'
> info: rolling back changes
> error: failed to install component: 'cargo-x86_64-unknown-linux-gnu', detected conflict: 'bin/cargo'
> ```

... but it can also take a different form such as one of the following:

- `error: "C:\Users\runneradmin\.cargo\bin\rustup.exe" is not a valid subcommand[..]` ([rust-lang/rustup#3709])
- `error: failed to install component: 'rust-src', detected conflict: 'lib/rustlib/src/rust/Cargo.lock'` ([rust-lang/rustup#3716])
- `error: the 'cargo' binary, normally provided by the 'cargo' component, is not applicable to the '1.78.0-x86_64-unknown-linux-gnu' toolchain` ([rust-lang/rust-clippy#12763])

... and this often leaves the user confused, especially because there is no
clear indications of what might have gone wrong.

Given the above background, a new transaction mechanism is needed to ensure
that only one rustup process can modify one nominally-unique toolchain at a
time.

### What we propose to do about it

In early 2026, a [proposal] (Zulip [thread]) was made by @rami3l envisioning
possible new transactional semantics via the use of a toolchain pool. However,
the bandwidth to follow up continously on this proposal was not quite available
at the time, and thus it was put on hold.

The solution proposed in the above document aims to address two types of
problems, namely that of synchronized toolchain modifications and that of
toolchain deduplication. It was later
[realized](https://rust-lang.zulipchat.com/#narrow/channel/490103-t-rustup/topic/Locked.20rustup.3A.20the.202025.20take/near/573621422)
that the latter problem can be resolved based on the former with a dummy
toolchain identification strategy (inspired by [A/B
partitioning](https://source.android.com/docs/core/ota/ab)), and thus the focus
of the first stage of this project will be on the former problem.

Hopefully, once this is done, the dummy toolchain identification strategy
can then be replaced with a proper content-based one, and thus resolving
the latter problem as well.

[proposal]: https://hackmd.io/@-BxJthnhTQWCxHdTwpNSWw/BkAcWKRIZx
[thread]: https://rust-lang.zulipchat.com/#narrow/channel/490103-t-rustup/topic/Locked.20rustup.3A.20the.202025.20take/with/607217932

### Design axioms

The implemented solution should:

- Consider the fact that any rustup operation might be interrupted at any time.
  This means it should be based on the operating system's active management of
  rustup processes (and thus be an integral part of rustup itself), rather than
  on some external "watchdog" process that may complicate the design and
  introduce new failure modes.
- Work on all major host platforms that rustup supports, even if this means
  that a special solution might be required for Windows.
- Prioritize the most common use cases first. For example, most people are
  using a single-user rustup installation on a single machine, so our design can
  compromise on other use cases, e.g. NFS should not be a blocker to landing any
  new transactional semantics despite the fact that NFS is known for not playing
  with FS synchronization very well on bad connections.
- Prioritize correctness over convenience if possible. For example, the
  critical section in our solution might be considerably large to begin with, and
  to be refined over time.

[da]: ../about/design_axioms.md

### Work items over the next year

The plan for the next year would be the following:

- Investigate prior art regarding this topic of interprocess FS
  synchronization, including both locked and lock-free approaches, to see what
  primitives can be used in this problem domain. Some examples of prior art may
  include:
  - Useful [lock-free FS synchronization
    primitives](https://rcrowley.org/2010/01/06/things-unix-can-do-atomically.html)
    & their platform support
  - [`cargo`'s file locking mechanism](https://github.com/rust-lang/cargo/blob/0d67af02c9b0e7b14fc7b24ce54dcec1d5ebead7/src/cargo/util/flock.rs)
  - [`gitoxide`'s file locking mechanism](https://github.com/Byron/gitoxide/tree/caae9260ef3d66998d6826c493631f3d7296c73f/gix-lock)
- Design the appropriate new transactional semantics for rustup.
- Make a PoC and evaluate the feasibility of the design.
- Coming up with a migration plan for the existing toolchain installations to
  the new transactional semantics and leave space for further toolchain
  identification improvements.
- Polishing the code for the said PoC and making it production-ready.
- Shipping the new transaction mechanism after a certain period of testing.

| Task                           | Owner(s) | Notes |
| ------------------------------ | -------- | ----- |
| Initial investigation & design | @rami3l  |       |
| PoC implementation             | @rami3l  |       |
| Feature implementation         | @rami3l  |       |

## Team asks

| Team     | Support level | Notes                                                                                    |
| -------- | ------------- | ---------------------------------------------------------------------------------------- |
| [rustup] | Small         | Members may have comments/thoughts on direction and priorities; Review work for t-rustup |

## Funding

| Purpose     | Cost | Funded | Sponsor(s) |
| ----------- | ---- | ------ | ---------- |
| Contributor | TBD  | No     |            |

## Frequently asked questions

N/A
