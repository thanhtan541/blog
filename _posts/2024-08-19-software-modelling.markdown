---
layout: post
title:  "Software Modeling"
date:   2024-08-16
categories: [software, development, architecture]
---

`What is Software modeling?`
- A process to layout what the software look like and its behaviors
- Event-driven architucture is a framework to build software based on event interaction
- Specifying behaviors (given-when-then, etc). How component reacts to certain events during certain state
    -E.g : given song (compoent) is playing (state), when click pause button is pressed(event), Then the song should be paused (another state)
- Event-first vs state-first two approach to design behavior
    - Event-first: Design a set of events that will be used in a application, ButtonCliked, FormSubmitted
    - State-first: Design a set of stated that service may be changed overtime, Polling, Ready, In-Progress
    - It depends on the ratio between state and event. If the components has alot of states, it is good indicator to start with state. E.g: func topup(amount) { if state == active -> increase}

