# Async Future Memory Optimisation

> **Instructions:** Copy this template to a fresh file with a name based on your plan.
> Give it a title that describes what you plan to get done in the next 6 months
> (e.g., "stabilize X" or "nightly support for X" or "gather data about X").
> Feel free to replace any text with anything, but there are placeholders
> designed to help you get started.
>
> The **point of contact** is the person responsible for providing updates.
>
> The **status** should be either **Proposed** (if you have owners)
> or **Proposed for mentorship** (if you do not yet).

| Metadata         |                                                                                  |
| :--------------- | -------------------------------------------------------------------------------- |
| Point of contact | @dingxiangfei2009                                                                |
| Status           | Proposed                                                                         |
| Tracking issue   |                                                                                  |
| Zulip channel    | N/A                                                                              |

## Summary

We want to solve `async`-future memory bloat problem.

[_Playground_](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=68493d5518e0cfec694750e62157a347)

```rust
async fn combining_future(compute: impl Future<Output = ()>) {
    async {}.await;
    compute.await;
}

fn main() {
    let blob = [0u8; 65536];
    let monster = combining_future(combining_future(combining_future(combining_future(async move {
        async {}.await;
        println!("{:?}", blob)
    }))));
    println!("{}", std::mem::size_of_val(&monster));
    //~^ prints at least 1000000 bytes
}
```

We would like to deeliver the first `async`-future memory packing scheme `-Zpack-coroutine-layout` as a nightly-only compiler flag, followed by a more aggressive memory packing scheme built on top of the foundation of the first `pack-coroutine-layout` support. Meanwhile, we would like to conduct two experiments in two possible approaches that could further improve the memory economy of `async`-futures.

- Explore the memory layout optimisation by enabling coroutine state inlining.
- Explore direct lowering of Rust `async`-futures, and coroutines in particular, into native [LLVM coroutine intrinsics](https://llvm.org/docs/Coroutines.html#coroutine-structure-intrinsics).

## Motivation

### The status quo

Exponential growth of `async`-future types has been a long-standing issue, tracked by [#62958](https://github.com/rust-lang/rust/issues/62958). In addition, any data that is alived and used in between more than two `await` points will incur further penalty, because their memory slots are indiscriminantly reserved throughout the futures' life span.

This issue has manifested in two ways that hinders the adoption of general `async` Rust or mandates unergonomic mitigation of this issue.

- On cloud platforms and in general network computing applications, it has led to unexpected or even unexplainable stack-overflow that is hard to debug. Application developers have to resort to unnecessary heap allocation and indirection in order to restore reliability.
- On embedded platforms and specialised environment, such as Linux Kernel, this issue has been quoted as the greatest concern to respective developers. Rust `async` has been a very attractive abstraction, but the memory footprint issue has deterred adoption in such fields because the memory requirement is more stringent.

### What we propose to do about it

A proposed fix has been proposed in [#135527](https://github.com/rust-lang/rust/pull/135527) which strives for perfect preservation of coroutine semantics while reducing the memory bloat contributed by coroutine captures, which has been the most common case for large future sizes. This experimental implementation work has shown success in improving future sizes. It also retains the default memory layout scheme in order to allow experimentation without affecting existing stable Rust users. Based on this foundation, the proposed work involves a second packing scheme to relax the overall layout computation, so that memory allocation can still be released even values are live across more than one `await` suspension points, enabling even further memory compaction. These changes will continue to abide by the design axiom, under which the surface Rust language remains unchanged and improvement work should be contained only in the internal Application Binary Interface (ABI) or code generation.

The proposed work also includes two further experiments as stretch goals.

- Enabling inlining of coroutine states. Many cases of bloat in `async`-future arise from combining futures in a nested fashion, where one future needs to drive other futures towards completion. The state and data of the nested futures are currently opaque to the driver future and the layout calculation must consider the memory layouts of the futures in isolation. This unfortunately leads to sub-optimal memory allotment leaving padding gaps that could have been used. Inlining these futures under reasonable condition, on the other hand, allows the layout computation to pool the liveness information of futures' internal data as a whole and enable the opportunity to fill the padding gaps.
- Experimenting with unstable coroutine intrinsics of LLVM. LLVM has since made coroutine intrinsics availabe, albeit without stability guarantee across releases at the moment. Nevertheless, this enables a code generation strategy for Rust `async` futures, which is to directly lower coroutine MIRs into LLVM IR coroutines, through one or more lowering strategies from LLVM. This allows LLVM to independently compute coroutine liveness information and eventually layout information directly, like how regular Rust synchronous `fn` is compiled, by exploiting the target information known to LLVM. This will potentially also allow LLVM to further apply optimisation to the final coroutine code, given more accurate information made available by the Rust code generator.

### Work items over the next year

| Task        | Owner(s) | Notes |
| ----------- | -------- | ----- |
| Polish the initial PR and merge | @dingxiangfei2009  | This is a work item that is most feasible, with work ready for upstreaming |
| Implement the second packing scheme, removing the liveness restriction | @dingxiangfei2009 | This is a work item that can be experimental, with existing work ready for reviews |
| Perform industry-wide survey to measure improvements | @dingxiangfei2009 et al. | This is a work item that requires collaboration with partners |
| Implement coroutine inlining under reasonable condition | @dingxiangfei2009 | This is a stretch goal that requires design meetings |
| Explore lowering to native LLVM coroutine, draft design documentation, liaison with LLVM on potential upstream work | Tentatively @dingxiangfei2009 | This is a work item that requires collaboration with compiler team and partners at LLVM |

## Team asks

| Team       | Support level | Notes                                   |
| ---------- | ------------- | --------------------------------------- |
| [compiler] | Medium   | Is this Small or Medium? Does it need a champion? |

## Frequently asked questions
