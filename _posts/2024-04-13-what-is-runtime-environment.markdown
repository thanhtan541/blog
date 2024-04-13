---
layout: post
title:  "Runtime Environment"
date:   2024-04-13
categories: [software, development]
---

`What kind of problem does it address?`
- There are underlying components that an application need to interact with in order to function properly inside a machine. Such as:
  - Operating System (OS)
  - Hardware
  - Other programs: tcp, socket, i/o, dns ...
- As developers, we focus more on business problem rather than these low-level concerns, that's why we choose high-level programming languagues (Rust, ...) to work on. It saves enormous time on development.
- Thus, `hiding away the underlying complexity and provide a friendly platform` that our code can run smoothly is why Runtime Enviroment existed.

`What is it?`

- Runtime refers to the environment in which a program or application is executed or run. It provides the necessary support for the program to function and interact with the underlying operating system, hardware, and other programs.
- The runtime environment acts as a bridge between the application and the underlying system, ensuring that the application can run smoothly and efficiently.
- Let's consider a pattern that we used daily to capture the external service and expose a set of intuitive apis, called Adapter. Runtime is a adapter.
- Thus, same application code can run on different hardware, os, or in techinical term, cross-platform. Each platform has a dedicated runtime, e.g: Nodejs support Windows, Linux, Macos, AIX (IBM)

`How does it work?`
 Runtime contains a set of components
- A garbage collector 
- A compiler
- A set of libraries for core functionalities
- An execution engine that manages program execution, hamdling tasks such as loading code, memory allocation and threading.
- Examples of runtime environments include the Java Virtual Machine (JVM) for Java applications, the .NET runtime for .NET applications, and the Python interpreter for Python applications.

