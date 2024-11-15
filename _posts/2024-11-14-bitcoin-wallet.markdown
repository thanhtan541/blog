---
layout: post
title:  "Bitcoin wallet"
date:   2024-11-14
categories: [crypto, blockchain, security]
---

## How to spend your bitcoin
---
There are two information involved.
1. Public information is stored on the blockchain that indicate how much it's worth and so on.
2. Secret information is the secret key of the owner of the bitcoin.

Based on that, `storing bitcoins is all about storing and managing Bitcoin secret keys`.

##Goals for storing secrets##
- Availability: able to spend coin when needed
- Security: nobody has access to your secret key
- Convenience: nobody has access to your secret key

> **_NOTE:_** Simultaneously achiving all three is not easy.

Simple way to store key is to use local file in your personal devices. However, it's not great for availibility and security
Let explore other approach!

### Wallet
A wallet software is usually installed in your device if you store bitcoin locally.
It's useful when you need to manage multiple bitcoin addresses.
Creating keypair is easy

### Encoding keys: base 58 and QR codes 
A way to exchange bitcoin addresses

