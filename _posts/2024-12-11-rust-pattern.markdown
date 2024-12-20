---
layout: post
title:  "Design patterns in Rust"
date:   2024-08-16
categories: [software, development, programming]
---

`Downcast`
- Problem -> Trait system (dynamic dispatch) is polymorphism feature equivalent in OOP. However, the in some cases, the concrete type is needed
- Solution -> The process to retrieve conrete type is called *Downcasting**.
- Context -> Suitable for long running application such as web app or multiple threaded application that have pre-configurations
- Usecases: Downcast is suitable if the underlying object is dynamic (not able to detect size at runtime). Beside that, `enum` is good fit
- Consequences -> Since it introcudes runtime's overhead, use it as minimal as possible.
- Example
```rust
fn log(value: &dyn Any) {
    match value.downcast_ref::<String>() {
        Some(text) => println!("Bytes of the string: {:?}", text.as_bytes()),
        None => println!("No string...")
    };
}
```

`Foreign Type Wrapper`
- Problem -> Rust compier is not allowed to applied foreign trait to foreign type
- Solution -> Creating a custom type that wrap the foreign type and impl the needed trait
- Context -> When you want to extend the primitive types with your business logic
- Usecases: Composition in OOP
- Consequences -> There is assumption that inner type is not changed
```rust
struc CustomU32(u32);
impl CustomU32 {
    fn value(7self)-> u32 {
        self.0
    }
} 
```
