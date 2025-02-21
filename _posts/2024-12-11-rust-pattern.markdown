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

`Deactivating Mutability`
- Problem -> There is object that should not be modified even it's in mutable owned.
- Solution -> Use new-type pattern and only implement Deref trait (not DerefMut).
- Usecase -> State object such as Configuration only init once and be shared by multiple threads
```rust
pub struct Immutable<T>(T);

impl<T> Copy for Immutable<T> where T: Copy {}

impl<T> std::ops::Deref for Immutable<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Config {
    pub base_url: String,
}

impl Config {
    pub fn build(self) -> Immutable<Config> {
        Immutable(self)
    }
}

let mut config = Config {
    base_url: "https://example.com".to_string(),
};
config.base_url = "https://example.com".to_string();

let immutable_config = config.build();

println!("immutable_config.base_url: {}", immutable_config.base_url);

let mut mutable_config = immutable_config;
// Cannot assign
mutable_config.base_url = "https://example.com".to_string();
```

`Use HashSet over HashMap`
- Problem -> Group data into a lookupable datastruct HashMap increase the memory allocation for the key
```rust
struct Person {
    name: String,
    age: u8,
}

fn persons_by_name(persons: Vec<Person>) -> HashMap<String, Person> {
    // key.clone() introduce duplicate data between key and value in struct
    persons.into_iter().map(|p| (p.name.clone(), p)).collect()
}
```
- Solution -> Use HashSet with reference to reuse data
```rust
use std::borrow::Borrow;
use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
};

struct PersonWithHash {
    name: String,
    age: u8,
}
impl Borrow<str> for PersonWithHash {
    fn borrow(&self) -> &str {
        &self.name
    }
}

impl PartialEq for PersonWithHash {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for PersonWithHash {}

impl Hash for PersonWithHash {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

fn persons_by_name_with_hashset(persons: Vec<PersonWithHash>) -> HashSet<PersonWithHash> {
    persons.into_iter().collect()
}

fn get_person_by_name<'p>(
    persons: &'p HashSet<PersonWithHash>,
    name: &str,
) -> Option<&'p PersonWithHash> {
    persons.get(name)
}
```
- Usecases: Fast lookup data structure with optimize memory allocation
- Consequences -> More boilerplate code and hard to read
