# Const Generics

| Metadata         |             |
|:-----------------|-------------|
| Point of contact | @BoxyUwU    |
| Teams            | lang        |
| Task owners      | @BoxyUwU    |
| Status           | Proposed    |
| Tracking issue   |             |
| Zulip channel    | N/A         |

## Summary

Work towards stabilizing the remaining const generics functionality that was left out of the original `min_const_generics` feature.

## Motivation & Status Quo

The `min_const_generics` feature was stabilized with a number of limitations, while some have since been lifted there are still some things we do not support:
- Inferred arguments to const parameters (`feature(generic_arg_infer)`)
- Const parameters with arbitrary user-defined types (`feature(adt_const_params)`)
- Non-concrete uses of associated constants in the type system (`feature(min_generic_const_args)`/`feature(associated_const_equality)`)

When using const generics it is common to run into these limitations and be unable to move forwards, having to rewrite your code to use workarounds or not use const generics at all. This is a poor user experience and makes the language feel incomplete.

### The next 6 months

- Get `generic_arg_infer`'s reference PR pushed over the finish line
- Write a document outlining the main design decisions of `adt_const_params` and discuss it with the lang team, follow up with an RFC
- Finish implementing the `min_generic_const_args` (mgca) feature prototype 

### The "shiny future" we are working towards

All parts of the const generics feature that were left out of the minimum stabilization have now been stabilized. Users do not encounter "functionality cliffs" where const generics
suddenly stops working as well as type generics, forcing code to be rewritten to work around language limitations.

## Ownership and team asks

| Task                              | Owner(s) or team(s) | Notes |
|-----------------------------------|---------------------|-------|
| `adt_const_params` design meeting | [lang]              |       |
| `adt_const_params` RFC draft      | @BoxyUwU            |       |
| Finish `mgca` prototype           | @BoxyUwU @camelid   |       |
