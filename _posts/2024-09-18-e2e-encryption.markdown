---
layout: post
title:  "End-to-end encrytion"
date:   2024-09-01
categories: [software, development, chat, security]
---

`Problem`
- To prevent data from being read/modified when travelling over an insecure channel.
- E.g: User submit their credential to server over the internet

`Cryptography`
- Data is transformed into a randomize information that is not pragtical to understand
- There are serveral primitives but in messaging application we focus on block cipher
    - A secret key will be used to encrypt and decrypt message
- Then, how to share secret key between two parties?

`Diffie-Hellman Key exchange`
- Goal: Establishing a shared secret key for encrypt/decrypt messages without sending information over publich channel or internet.
- There are components that are exposed in publich area:
    - n: prime number - more than 128bit
    - coprime: a number that share one the only 1 as factor with the number `n`. E.g: 10 and 11 are coprime
    - order: number of coprimes of `n`
    - g: generator - the element in the `order` that whoses power cycle through the whole coprime of `n`. Or called: Primitive Roots mod N, or `Generator of Mod N group`
    - cyclic: group that has at least one generator
- Both parties use these constants to generate their asymmetric keypair (public and private) - Sometime refer Asymmetric key encryption scheme.
- The two public keys are then swap between two parties.
- From local private key and foreign public key, the shared secret key is created - Symmetric key encryption
- Messages are encrypted/decryted using that key

- DLP is the key

`RSA`
- Related terms:
    - Factor: numbers that are being multiplied together equal to original number. E.g: factors of 12 is 1,2,3,4,6,12
    - Prime: number that only has two factors: 1 and itself
    - Semi-prime: number that only has two factors are prime numbers
    - Everytime two primes are being multiplied, the result is semi-prime
    - Modulo: Remainder division, e.g: 13 mod 5 = 3
- Generating keys:
    - select p and q are prime numbers
    - calculate **N** = p * q
    - Calulate Totient **T**, find total coprime numbers of **N** - (|)(n) = (p-1)(q-1) - [link](https://crypto.stackexchange.com/questions/5715/phipq-p-1-q-1)
    - Choose a public key **E** which
        - must be a prime number
        - must be less than Totient
        - must not be a factor of Totient
    - Choose a private key **D** which
        - Product of **D** and **E**, mod T = 1
        - D * E = 1 mod T
- Encryption and Decryption
     - Encryption: Message **M** raise to **E** = Ciphertext **T** MOD **N**
     - Decryption: Ciphertext **C** raise to **D** = Message **M** MOD **N**

`Advance Encryption Standard` - AES
- Symmetric key encryption
- Message are encryted with a block (each byte of key will be a unit of block, e.g: 4x4 = 16 bytes = 128 bit)
- There are multiple modes, each modes will have different steps and make the cipher harder to decrypt
- See more at: TODO
