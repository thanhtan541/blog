---
layout: post
title:  "Garbage Collector"
date:   2024-04-14
categories: [software, development]
---

`What kind of problem does it address?`
- Back in the day, C was the high-level language, developers need to manage memory from code perpective.
- There are few obvious drawbacks with this approach
  - Code is mixed with business logic and low level task
  - Easy to over allocate memory or deallocate two early
  - Error-pornce
- Thus, `we need a solution to handle memory allocation and deallocation behind the scene`

`What is it?`
- Garbage collector (GC) is a component of runtime enviroment. 
- Many modern programming language implements GC to free developer from managing memory responsibility.
- It is a deamon process.

`How does it work?`
- How does it know a object is no longer in use and free it?
  - Based on the data in object header: mark word and klass word
  - There is no reference point to that space.
- JVM will put those objects in `marked` state.
- Then JVM will perform GC and memory reclamation.
- Usually, JVM split memory into different spaces to increase the performance of GC:
  - Short-live: Objects are newly created.
  - Survior: Objects that survice after GC run throught short-live space.
  - Permanent: Objects that survice after GC run throught survior space.

  `When do objects consider to be sweep by GC`
- Stack frame is poped out of stack
- Nullify
- Referencing

`Are there any trade-off?`
- Performance is decrease because of extra overhead.
- There is still change of memory leak such ad cyclic referencing.

`What is next?`
- Discussing about each components of a Runtime Environment
