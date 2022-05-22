# About

This rust workspace attempts to distill a confusion I have around `SourceFile` usage when attempting to generate test code from a procedural macro.

# Instructions

See `./run-ticket.sh` for generated output which will be filed in a ticket.

# Description

The `./mymacro` crate provides a procedural macro which requires the annotated item and attribute to have an associated filesystem path (ie `SourceFile::is_real` is `true`).

It then wraps the `item` by textually prefixing:

```
#[cfg(test)]
#[test]
```

The reason for `#[cfg(test)]` is to emulate my development usage where I attempt to general a `#[cfg(test)] mod …`. Note that if you remove `#[cfg(test)]` prefix, the build succeeds!

With that prefix, build fails because there are non-real `SourceFile`'s from generated strings.

**However,** `cargo expand --tests-lib` appears to correctly work and produce the "correct" expansion!

So why does `cargo expand` work where `cargo build` fails? Shouldn't `expand` produce an identical intermediate representation as the actual compiler generates?
