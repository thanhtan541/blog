---
layout: post
title:  "General knowledge in Rust"
date:   2025-04-09
categories: [software, development, programming]
---

`Opaque Type`
- Definition -> a type is not fully exposed its concrete implementation to the user. It enforces `Information Hiding`, only handler functions are visible.
- Implementation -> `impl Trait`
- Example
```rust
fn returns_something() -> impl std::fmt::Debug {
    vec![1, 2, 3] // The return type is Vec<i32>, but the caller only knows it implements Debug
}
```

`Precise Capturing`
- Definition -> allow you to explicitly control which generic type are captured by `impl Trait`
- Implementation -> use `impl Trait + use<T>`
- Example
```rust
fn foo<T>(_: T) -> impl use<T> Sized { // Captures only T
    ()
}
```
