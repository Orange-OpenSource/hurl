It is often good practice, especially when venturing on large API refactors, to double-check for any newly created memory leaks.

Some leaks can only be spotted in external projects that show advance use cases of `rust-libxml`, for example allocating a `Node` in a default trait of a struct with a `Node` field. For now the only safe approach to that pattern is using the `Node::null()` placeholder, but the Rust idiomatic approach is to instead refactor to an `Option<Node>` field.

Some, more direct, leak scenarios can already be spotted from the libxml test suite, and one can use valgrind to obtain a report via a call of the form:

```
  valgrind --leak-check=full target/debug/base_tests-3d29e5da1f969267
```

Additionally, as Rust nightlies keep evolving, a specific allocation system may be necessary to properly run valgrind. At the time of writing, `rust-libxml` tests need no such changes, but some external projects do. For convenience, here is a known working preamble, which can be added to the preambles of executable files, including example and test files.

```rust
#![feature(alloc_system, allocator_api)]
extern crate alloc_system;
use alloc_system::System;

#[global_allocator]
static A: System = System;
```

For more discussion motivating this explanation, see the respective [GitHub pull request](https://github.com/KWARC/rust-libxml/pull/43).