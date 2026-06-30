# Allow turbofishing late bound vars

| Metadata         |                                                                                  |
| :--------------- | -------------------------------------------------------------------------------- |
| Point of contact | @oli-obk                                   |
| Status           | Proposed                                                                         |
| Funding contact     | [RustNL](https://rustnl.org)                          |
| Tracking issue   |      |
| Zulip channel    | https://rust-lang.zulipchat.com/#narrow/channel/600108-t-types.2Fearly-late-cleanup |

## Summary

Opaque types and function item types are handling late bound lifetimes very hackily.

We think something along the lines of the ideas below can work, but mainly we want to try out lots of things here and land whatever sticks to the wall ("sticky" meaning "improves things")

* make FnDef's args higher ranked (i.e. put them behind a binder)
* update all the HIR ty lowering logic for args to functions to properly map stuff to late bound parameters and check them
* track late bound function parameters in generics_of
* try to make opaque types nicer by stopping to duplicae args/params all over the place

Sparse on purpose, we're figuring things out as we go

## Motivation

### The status quo

The following snippet fails to compile

```rust
// 'a is late bound
fn foo<'a>(b: &'a u32) -> &'a u32 { b }

fn main() {
// error
let f /* : FooFnItem<????> */ = foo::<'static>;
}
```

while

```rust
// 'a is late bound
fn foo<'b: 'b, 'a>(b: &'a u32) -> &'a u32 { b }

fn main() {
// FCW
let f /* : FooFnItem<????> */ = foo::<'static>;
}
```

compiles with a FCW warning.

We want both to become expressible, and irrespective of syntax, we need to support it first in the compiler.

### What we propose to do about it

1. change datastructures to support this
2. eagerly generate bound vars for all FnDefs instead of lazily only doing it when getting the FnSig for them
3. change borrowck to run first with the new system, and if that errors, run with the old system. Then turn all errors that only happen in the new system into a FCW and feature gate all errors that only happened in the old system.

### Work items over the next year

| Task        | Owner(s) | Notes |
| ----------- | -------- | ----- |
| add binders to FnDef | @addiesh  |       |
| fill binders on FnDef with vars | @addiesh |       |
| change borrowck | @oli-obk |      |


## Team asks

| Team       | Support level | Notes                                   |
| ---------- | ------------- | --------------------------------------- |
| [compiler] | small         |                                         |
| [lang]     | small         | Not relevant yet, but in the future     |
| [types]    | large         |                                       |

## Funding

| Purpose | Cost | Funded | Sponsor(s) |
|---------|------|--------|------------|
| Reviews and Impl | Ask | Partial |  |

## Frequently asked questions

### Can we partially turbofish a function with multiple late bound vars?

e.g. does `some_fn::<'a, '_, 'static>` leave the middle as a late bound var?
Or do we need new syntax?