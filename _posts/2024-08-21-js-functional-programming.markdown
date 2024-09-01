---
layout: post
title:  "Function Programming"
date:   2024-08-21
categories: [software, development, js]
---

`Function purity`-> same input, same output
`Partial application`-> aka curry

**Function definition**
`Parameter vs Argument`
- Parameter is used to describe function signature. E.g function A has two parameter b and c.
- Argument is used to mention the value pass into function call. E.g function A receives argument 1 and 2
`Function shape` -> unary vs binary, and entropy (has more than two parameters)
`High Order Function` -> function receives other function as input and/or return one or more functions

**Point Free** 
- Pass a function and modify it behavior without touch its parameters
- E.g: not(isOdd) === isEven
`Equational Reasoning`
`Referential Transparency`
- meaning with function always return same output for same input.
- Haskell use it to inline or replace function call with it output
`Partial Application and Currying`
- Two method two compose function from general to specilize
- Partial: composit function and it parameters
- Currying: Nested function - mostly used unary function
**Composition**
- Combine multiple function to achieve bigger task. E.g: shippingRate = triplePrice + MinusTwo
`Composition vs Pipiping`
- Composition: from right to left - the right most function executes first
- Piping: from left to right - the left most function executes first
- Example:
``` Javascript
function pipe(...fns) {
    return function piped(v) {
        for (let fn of fns) {
            v = fn(v);
        }
        
        return v;
    }
}
function composed(...fns) {
    return pipe(...fns.reverse());
}
```
`Associativity` - Mathematical concept
- Sometime equal term with composition, e.g : 1 + 2 = 2 + 1 - same result regardless of order of operation
