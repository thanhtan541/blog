---
layout: post
title:  "Hybrid Public Key Encryption"
date:   2024-11-14
categories: [crypto, security]
---

## HPKE: Standardizing public-key encryption
---

### Purpose
A way for two parties can safely exchange information over public channel, such as the Internet

### Before HPKE
Diffe-Hellmain key exchange method was a breakthough. It is a serie of mathematical techniques that produce a `shared secret key` that both parties can use to encryt exchange data.
The wellknown application of this concept is the **Transport Layer Security (TLS)**.

> **_NOTE:_** Details at [Link]()

The PKE (wihtout H) is a protocol that each party share its Public Key to other side for encrytion and signing and use Private Key to verify message authencity and decrytion.

### HPKE
The H (Hybrid) mean that there is an extra step is added to PKE protocal.
Usually, it's a non interaction step that aim to produce shared secret between two party.
Thus, there are two phases.
- Unantheticated: Exhange Public Key over public channel
- Authenticated: Use Private Key and other end public key to produce Shared Secret. Check: RSA and Eliptic Curse

### Goals
Simple, Reuasble, Future-Proof.

### Flows
There are mutliple stages:
1. Key Encapsulation Mechanism (KEM): can use DH protocol for contrustion.
  - KEM has two algorithms: Encapsulation (Encap) and Decapsulation (Decap)
    - Encap creates a symmetric secret and wrap it for a public key that corresponding private key can unwrap it. 
    - Decap takes the encapsulated key and private key to compute the original shared secret.
1. From shared secret: a serie of derived keys are used for encryption and decryption.

### How it fullil its promises
- Algorithm agility: easy to swap implementation based on application context
  Three algorithms are used
    - KEM: securely creates `shared secret` 
    - Key Derived Function (KDF): use `shared secret` to create `derived keys` for encypt/decrypt, gauranty `post-secrecy`.
    - Authentication Encryption Algorithm (AEAD): 
- Authentication: PKE may be the go-to approach to authenticate message. However, there are other method worth to be considered.

