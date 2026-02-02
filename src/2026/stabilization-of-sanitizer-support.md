# Stabilize MemorySanitizer and ThreadSanitizer Support

| Metadata         |                                    |
|:-----------------|:-----------------------------------|
| Point of contact | @jakos-sec                         |
| Status           | Proposed                           |
| Tracking issue   | [rust-lang/rust-project-goals#403] |
| Zulip channel    | N/A                                |
| Stabilization    | true                               |


## Summary

Stabilize the MemorySanitizer and ThreadSanitizer support. This includes fixing open bugs for the sanitizers to open a path for stabilization and the necessary infrastructure changes to provide precompiled and instrumented standard libraries for the sanitizers.

## Motivation

Sanitizers help with increasing the robustness and security of software and have been increasingly adopted as part of the software development life cycle. Even though the Rust programming language provides memory and thread safety guarantees, use of Unsafe Rust and foreign code in mixed-language binaries do not provide the same memory and thread safety guarantees. Thus, [support for sanitizers must be added to the Rust compiler for secure Rust adoption](https://hackmd.io/@rcvalle/S1Ou9K6H6#Organize-and-stabilize-support-for-sanitizers).

In order for them to be properly usable, the sanitizer support should be stabilized so it is no longer required to use a nightly toolchain and build your own standard libraries.


In the future we want to stabilize all sanitizers (including memtag, cfi, kcfi, safestack, shadow-call-stack) and support them on all targets supported by the Rust compiler.
In the meantime we work towards at least supporting [Tier 1](https://doc.rust-lang.org/nightly/rustc/platform-support.html#tier-1-with-host-tools) targets for all of them.


### The status quo

Currently, there is unstable support for several sanitizers (address, hwaddress, memtag, memory, thread, leak, cfi, kcfi, safestack, shadow-call-stack). The AddressSanitizer and LeakSanitizer (that do not require rebuilding an instrumented standard library) are close to being stabilized (https://github.com/rust-lang/rust/pull/123617). We've just merged a new Tier 2 target (https://github.com/rust-lang/rust/pull/149644) for AddressSanitizer to allow using it with a stable compiler and are planning to repeat same process now for MemorySanitizer and ThreadSanitizer.

### What we propose to do about it

We start with stabilizing the `MemorySanitizer` and `ThreadSanitizer` and pick the rest as they come.

### Work items over the next year

The goal is to stabilize MemorySanitizer and ThreadSanitizer for [Tier 1](https://doc.rust-lang.org/nightly/rustc/platform-support.html#tier-1-with-host-tools) targets and provide a way to use them without rebuilding the standard library (which currently is also behind an unstable flag). This requires a way to ship sanitizer instrumented standard libraries (for Memory- and ThreadSanitizer) through [rustup](https://rustup.rs/).


| Task             | Owner(s)            | Notes                             |
|------------------|---------------------|-----------------------------------|
| Implementation   | @jakos-sec, @1c3t3a | [MCP 951 tracking issue][MCP 951] |
| Reference PR     | @jakos-sec, @1c3t3a |                                   |
| Stabilization PR | @jakos-sec, @1c3t3a |                                   |

[MCP 951]: https://github.com/rust-lang/rust/issues/151253

## Team asks

| Team                          | Support level | Notes                  |
|-------------------------------|---------------|------------------------|
| [infra]                       | Small         |                        |
| [compiler]                    | Medium        | Reviews, stabilization |
| [bootstrap]                   | Medium        | Dedicated reviewer     |
| [project-exploit-mitigations] | Medium        | Dedicated reviewer     |
