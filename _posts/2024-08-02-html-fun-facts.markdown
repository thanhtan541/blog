---
layout: post
title:  "HTML fun facts"
date:   2024-08-02
categories: [software, development, front_end]
---

`HTML attributes vs DOM properties`
- HTLM attribute is attach to html element and can be parsed to JS Dom. 
- Html stages
1. Dom Tree
1. Render Object
1. Stacking context: each stackling context has its own render layor
1. Render Layer Tree
1. Graphic layer

`Reflow`
- A pipeline is trigger by js to re-render html
- Each step will utilize CPU or GPU

`Render Layer`
- when elements have css properties: position: relative | absolute

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
