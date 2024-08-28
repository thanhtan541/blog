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

`Behavior Design Patterns`
- problem -> deal with object interaction and responsibility distribution.

`Observer`
- problem -> Allow object to notify other objects
- solution -> Define a object that maintains a list of observers and notify them when state changes
- context -> Pub/sub scenario
- usecases: Event handler, Real-time notification, UI update
- consequences -> Easily to make performance overhead
- example
```javascript
class Subject {
    observers = new Set();

    addObserver(observer) {
        this.observers.add(observer);
    }

    removeObserver(observer) {
        this.observers.delete(observer);
    }

    notifyObserver(message) {
        this.observers.forEach(observer => observer(message);
    }
}
subject.addObserver(message => console.log("Event fired"));
```

`Template`
- problem -> Define a skeleton of an algorithm that will change on different implentation
- solution -> Create a Class with a tempale method to outlines the algo and let sub-class override specific steps.
- context -> When you have common algorithm with varying steps.
- usecases: Data Processing, Form validation
- consequences -> Low readable, cause performance overhead
- example
```javascript
class DataProcessor {
    process() {
        this.loadData();
        this.processData();
        this.saveData();
    }
}
class JsonDataProcessor extends DataProcessor {
    loadData { /* code */}
    processData { /* code */}
    saveData { /* code */}
}
```

`Memento`
- problem -> Capture and externalize an object's internal state
- solution -> Create a Class that stores the states of the original object
- context -> When you want to keep track or manage object state
- usecases: Undo/Redo functinality, Time-tralvel debugging, Saving game or app session
- consequences -> Low readable, cause performance overhead
- example
```javascript
class HistoryManager {
    history = [];
    push(state) {
        this.history.push(createMemento());
    }
    pop() {
        if (this.history.length === 0) return null;
        this.history.pop();
    }
}
class JsonDataProcessor extends DataProcessor {
    loadData { /* code */}
    processData { /* code */}
    saveData { /* code */}
}
```

`Command`
- problem -> Avoiding hard-wiring a request from its invoker
- solution -> Create an object that captured all needed data to perform an action at later time
- context -> When offload the request handler
- usecases: Manage action in your app
- consequences -> Low readable, cause performance overhead
- example
```javascript
class DataProcessor {
    process() {
        this.loadData();
        this.processData();
        this.saveData();
    }
}
class JsonDataProcessor extends DataProcessor {
    loadData { /* code */}
    processData { /* code */}
    saveData { /* code */}
}
```

`Singple Page Application`
- problem -> Avoiding duplicate calls to server and re-rendering
- solution -> Create an interaction site with dynamically fetching data
- context -> When SEO is not critical and user interaction is high
- usecases: Backoffice site
- consequences: Complexity to maintain, low SEO 

`Lazy Load`
- problem -> Loading too many assets at once cause performnce issue
- solution -> use dynamic import to load module when needed
- context -> when init time is too high
- usecases: load web component when needed
- consequences: less readable code

`View Transition`
- problem -> When changing between routes
- solution -> use View Transition API
- context -> handling page changing
- usecases: Animate page change, Morph elements between pages
- consequences: less readable code

`HTML template with interpolation`
- problem -> When using templates for web component, you can't express in the HTML the binding you want
- solution -> Using a trick using ES string template
- context -> Dynamic rendering
- usecases: binding for html
- consequences: less readable code

`Routingn Metadata`
- problem -> Manage shared data between routes
- solution -> Updating the metadata dynamically
- context -> Separate concern
- usecases: Theme-color, change title
- consequences: less readable code

`Show Root` -> Todo

`Multiple Page Application`

`View Transition for MPA`
- problem -> User can see white splash between page load
- solution -> Use the View Transition API
- context -> Improve UX
- usecases: Make MPA feel like SPA, Morph one element from one HTML to another element consequences: 
- consequences: Complex

`Prefetch`
- problem -> Loading performance
- solution -> Using different techniques to prefect the next possible pages like Cache Storage, Worker, or Speculation Rules API
- context -> Improve UX
- usecases: Prefer or re-render next possible pages
- consequences: Complex

`State Management`

`Promisify Data`
- problem -> Likely to change in the future
- solution -> Use promise
- context -> Take advantage of async await
- usecases: Fetching Data
- consequences: More code 
- example: Promise.resolve(data)

`Flux`
- problem -> In large scale project, managing state is too complex and unpredictable
- solution -> Use unidirectional data flow
- context -> centralize state managing
- usecases: CMS, eCommerce
- consequences: More code 


`Lazy Sync`
- problem -> Large dataset to load
- solution -> Make all sync call to server asynchronously and detach from the UI
- context -> Improve UX and performance 
- usecases: Saving data, saving analytics
- consequences: More code 

`Proxy`
- problem -> Control the access to certain object
- solution -> Use Proxy to wrap around the object
- context -> Access management
- usecases: Reactive Programming, Adding Security layer
- consequences: More code 

`Middleware`
- problem -> Handling tasks that affect multiple part of applications like logging, error handling
- solution -> Inject another layer between processes
- context -> Centralize logic
- usecases: API Access, Database access
- consequences: More code 
