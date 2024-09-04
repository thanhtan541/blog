---
layout: post
title:  "Design patterns"
date:   2024-09-01
categories: [software, development, web_app, security]
---

`Tabnabiing or Browser Hijack`
- Problem -> The adverasy use js script to modify the window.opener.location.herf
- Solution -> use a's attribute **rel** with **noopener" and "noreferrer"
- Usecase:
    - Noopener: no opener object 
    - Noreferrer: no referrer link
- Consequences:
    - No good in MPA

`bfcache` - Back/forward cache
- Problem: rerender the whole page when go back is costly
- Solution: using bfcache to take a snapshot before doing the forward
- Usecase:
    - Mobile app: reduce time when travel multiple app

`Front-end Security`
- XSS
- CSRF
- UI Redressing
- MITN
