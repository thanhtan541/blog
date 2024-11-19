---
layout: post
title:  "Cryptography"
date:   2024-08-16
categories: [security]
---

`Why?`
- **WWW** creates a massive connection around the world which also increase the surface of attack.
- We need to robust security protocol to make sure the connection between point A and B are reliable.

`What?`
- Crytography is where security engineering meets mathematics.
- It provides a mechanism to send messages securely by transform the ogirin text to hard for decoding version

`How?`
- There are 5 cryto primitives
    1. Random function: take any input length and output a random string of fixed length
        - Properties:
            1. One-wayness
                - Problems: 
                - Solution: Hash function: h(x) -> y.
                - Usecase: Password Hash, signature, checksum
            1. Randonness
            1. Collison-free: Hard to find two inputs has same hash
    1. Random generator - stream cipher: similar to Random function, except it has a Secret key which size equal to Input Data
        - Properties:
            1. Encrypt/Decrypt: using secret key
            1. Key and plaintext has same size
            1. Time to guess plain text equals to guess key, it means each cipher has its own key
        - Usecase: protect stearm data such as message and voice ..
        - Cipher: the output of function that use Plaintext and Key:
        - Formula: C = P XOR K
    1. Random Permutations — Block Ciphers
    1. Public Key Encryption and Trapdoor One-Way Permutations
    1. Digital Signatures

`Scheme vs Mechanism`
- Scheme is prefered to a set of goals and objectives, e.g: how to archive authencity, security based on environment. Based on these, we can choose specific mechanisms.
- Mechanism focus on how, which means conrete implementations. E.g: AES or RSA ...
`Key encapsulation vs wrapping`
- Key encapsulation prefer to process involved keypair to encap and decap a shared secret key. Example: KEM
- Key wrapping is used to describe a key to encryption another key. It usually involve **symmetric encryption**. Example: Key Management Service

`Common encrytion scheme/mechanism/system`
- Key Encryption Mechanism - **KEM**  
    - Goal: to protect and adversaries to read or modify the encapsulation secret key (sk). The **sk** is usally known as `shared secret key` between two party to encrypt and decrypt exchanged messages.
    - How: Use an asymmetric ecnryption system to exchange public key. `Note`: the secret key is usually smaller then other mechanism.
