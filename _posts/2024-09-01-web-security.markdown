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

`Cookie policy header`
- Control cookie to be shared and used.
- Best practices are limit client-side and cross-side navigation to access the cookie
- Types of web cookies:
    - First-party cookies: This cookie usually used to remember non-sensitive information like language's setting. Normally all sites that you visitted have this kind of cookie
    - Third-party cookies: Cookies from sites that you are not visitted. For example, ads tracking, google's analytics
    - Session cookies: Temporary cookies that be removed when your close the browsers or your session is ended
    - Persitent cookies: Last longer than session cookies. Login's information is one the usecase. Normally there is a hard limit for cookie's lifetime required by laws, e.g: 6 months ...

`HSTS` - HTTP Stric Transport Security
- Set of rules that indicates transport security polcies

`CORS` - Cross-origin resource sharing

`QUIC` - http transport protocol

`Same Origin Policy`
- There is a tuple: (protocol + domain + port) that indicate origin
- Ways around it:
    - CORS
    - JSON with Padding (JSONP)
    - Proxies
    - PostMessage API
    - Websockets
    - document.domain

