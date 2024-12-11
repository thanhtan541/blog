---
layout: post
title:  "Practical patterns in Rust"
date:   2024-10-26
categories: [software, programming, design_pattern]
---

`Validation`
- Problem -> Boolean is cool but not good enough
- Solution -> Use Result enum
- What -> Result<(), CustomError>
- Context -> the validation is critical and mean to break the flow instead of let is through peacefully

`Parse over new(constructor)`
- Problem -> User is very creative and has no boundary
- Solution -> Make sure they are only able to 
- What -> CustomType::parse(raw_string) -> Result<Self, ParseError> {assert()}
- Context -> When interacting with user input or type conversion

`From and TryFrom`
- Problem -> A united approach fo type conversion
- Solution -> Use From or TryFrom trait
- What -> impl From for T {}
- Context -> When internal type need to parse to other type. Useful for complex type that has multiple properties

