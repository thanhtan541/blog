---
layout: post
title:  "Design patterns"
date:   2024-08-16
categories: [software, development, web_app]
---

`Name`-> Vocabulary: an indicator for a specific pattern instead of nuance explaination
- Problem -> The problem that this pattern strikes for
- Solution -> How to solve it
- Context -> When to use it
- Consequences -> Drawbacks
- Example -> Code implentation

`Singleton`
- Problem -> To avoid duplications of one services in an application
- Solution -> Only allow to init service once during application's lifetime
- Context -> Suitable for long running application such as web app or multiple threaded application that have pre-configurations
- Usecases: Global configurations, Database connection pool, Logging service
- Consequences -> Since it a single point of read, any modification will cause un-expected behavior.
- Example
```javascript
const db_connection = {
    open: async (),
    sendQuery: async (query) => {}
};
init(db_connection); // Global access
```

`Factory`
- Problem -> There are complex operations that is easy to make mistakes
- Solution -> Encapsulate the complex steps into minimal expose API 
- Context -> It works well when there is only small number of dynamic configurations inside the complex operations
- Usecases: FileHandler, HttpConnection 
- Consequences -> There is an additional layer which is high coupling with the product is build on. For example file system in Win or Linux is different
- Example
```javascript
class PDFParse extend Parse {}
class CSVParse extend Parse {}
// Factory class
class Reader {
    getParse(file) {
        // Return suitable parse
    }
}
```

`Decorator`
- Problem -> Avoiding modification of existing code. There is high risk when adding new feature to current structure
- Solution -> Using any technique that wrap an object inside other object
- Context -> Get a good value when dealing with un-controllable code, such as dependencies
- Usecases: Add logging/tracing, caching
- Consequences -> Easy to misuse if ther is lacking of documentation
- Example
```javascript
class Button {
    onClick() {
        //Do something like change button color
    }
}
class SubmitButton extend Button {
    onClick() {
        super.onClick();
        //Submit form data to server
    }
}
```

`Adapter`
- Problem -> Incompatible interfacs when integrating two services
- Solution -> Create a middle layer to glue separate parts
- Context -> Extremely useful when dealing with 3rd party services
- Usecases: MailHanlder, PaymentHandler
- Consequences -> Easy to break when two end making and modification, introduce coupling
- Example
```javascript
class Post {
    save() {
        //Persist into database
    }
    notify() {
        // Invoke mail handler adapter
    }
}
class MailHandlerAdapter {
    config() {}
    send() {}
}
```

`mixin`
- problem -> include shared behaviors among objects
- solution -> using object.assign in javascript
- context -> works like an extension, an object is still working fine without it
- usecases: logging, caching ..
- consequences -> the mixin usually is highly couple with the object it is applied on
- example
```javascript
let sayhimixin = {
    sayhi() {console.log(`logging ${this.name}`} // name is not existed in this scope
}
class user {
    name // property used in mixin
}
object.assign(user.prototype, sayhimixin); //glue code
```

`value object`
- problem -> There is complex data that not realy match with premitive struct
- solution -> Define a data structure with immutability in mind
- context -> Well known in DDD
- usecases: MoneyObject, AddressObject, CoordinateObject
- consequences -> Easily to overuse
- example
```javascript
class Money {
    constructor(amount, currency) {
        this.amount = amount;
        this.currency = currency;
    }
    equal(other) {
        this.amount = other.amount &&
        this.currency = other.currency;
    }
}
```

