---
layout: post
title:  "Runtime Environment"
date:   2024-05-05
categories: [software, development]
---

`What is backpressure?`
- This term is from fluid flow in piping and air vent systems.
- It indicate a problem that the incoming data is too big for the receving end to handle and cause the system down or
unavailable for a period of time.
- In software, this problem likely is to happen with async program, e.g: Nodejs which in desgin to try to consume as much of information and
trigger operation as fast as it can

`How to prevent it?`
- Signal the sending party to slow down
- Introduce a intermidate to store the message until the downstream is ready, e.g: Buffer or Queue
- Batching events, combine multiple piece of data into one
- Pre-aggregate the incomming message, 
- Debounce, only process a event not all. E.g: User input

