---
layout: post
title:  "HTML fun facts"
date:   2024-08-02
categories: [software, development, front_end]
---

`HTML attributes vs DOM properties`
- HTLM attribute is attach to html element and can be parsed to JS Dom. 

`Html stages`
1. Dom Tree
1. Render Object
1. Stacking context: each stackling context has its own render layor
1. Render Layer Tree
1. Graphic layer

`Reflow`
- A pipeline is trigger by js to re-render html
- Each step will utilize CPU or GPU

`Render Layer`
- when elements have css properties: position: relative or absolute

`Graphic Layer`
- when elements have 3D or perspective transform CSS properties
- https://csstriggers.com

`DOM API`
- Class hierarchy: HtmlElement -> Element (has DOM API) -> Node (tree-like) <-| TextNode   
                              HtmlDocument (has DOM API) -> Node

`Observer API`
- There are 3 types:
  1. Intersection: tracking elements overlapping
  1. Mutation: rich text editor, drawing tools
  1. Resize

`Performance Optimization - Web vitals`
- There are 3 measures:
  - Largest Contentful Paint (**LCP**)
    - Target: loading performance
    - Goal: <= 2.5s of page loading
    - Where: 
  - Interaction to Next Paint (**INP**)
    - Target: interactivity
    - Goal: <= 200 miliseconds
    - Where: **reflow**
  - Cumulative Layout Shift (**CLS**)
    - Target: Visual stability
    - Goal: <= 0.1 or less
    - Where: resizing
- Toolchain: Lighthouse

`Performance Optimization - Network Performance`
- Network Protocol:
  - Target: Use latest stable protocol: http2 (http3 is coming)
  - Goal: reduce the cost because http1 has more overhead on tcp connection + http headers
- Javascript Bundle Optimization:
  - Target: js file bundle
  - Goal: deferred load
  - Where: 
    - Optional files could potential be loader afer first renderring
    - ES version also affect bundle size. Newer verison has less weight than older one
    - Reduce filesize via compression and mification
- CSS, Images and Renderring:
  - Target: file format 
  - Goal: reduce file size with same quality via choosing right format and compression
  - Where:
    - Image: webp for image and svg for icon
    - CSS: preloading
    - Font loading:  use `font-display: optional`

`State management`
- use Normal Form technique (NF)
- design table that fits critical and high performance purpose: searching


`Frontend application design workflow`
1. Requirement: breakdown into stories and define non-functional things
1. Wireframe: how user interaction
1. State: use NF to define state db
1. API: to get data from server: short/long polling, server sent event (SSE), websocket

`Short polling, long polling, websocket, sse`

