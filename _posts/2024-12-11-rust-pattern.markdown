---
layout: post
title:  "Design patterns in Rust"
date:   2024-08-16
categories: [software, development, programming]
---

`Downcast`
- Problem -> Trait system (dynamic dispatch or type-erasure) is polymorphism feature equivalent in OOP. However, the in some cases, the concrete type is needed
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

`Clone on write a.k.a Copy on write in other programming language`
- Problem -> Allocation is costly. There are subset of operation only need to read the data.
- Solution -> Clone (or Copy) on Write - `COW` is a technique to delay copying data until it's needed.
- Usecase -> Working on `&str` and `String` is common operation.
- Consequences -> Add a slight overhead to check wether data is being cloned (or copied)
```rust
// Cow from std::borrow
pub enum Cow<'a, B>
where
    B: 'a + ToOwned + ?Sized,
{
    Borrowed(&'a B),
    Owned(<B as ToOwned>::Owned),
}

fn to_upper_case(input: &mut Cow<[char]>) {
    for i in 0..input.len() {
        if input[i].is_ascii_lowercase() {
            input.to_mut()[i] = input[i].to_ascii_uppercase();
        }
    }
}

let input = ['A', 'B', 'C']; // All elements are uppercase.
let mut cow_input = Cow::from(&input);
to_upper_case(&mut cow_input);

match cow_input {
    Cow::Borrowed(_) => println!("Only borrow"), // Only borrow
    Cow::Owned(_) => println!("Move happened"),
}
```
