# ethos-rs

This is a Rust rewrite of ethos, a backend for the [Festa do Taipe](https://www.festadotaipe.xyz/) project.
Initially written in Typescript with the NestJS framework, it uses GraphQL as communication protocol and Prisma as ORM.

The goal is to recreate every feature in Rust and, in the end, compare both of the results with benchmarks.
It'll be tested for serverless environment and coldstarts.

## Features

- [ ] Project tenants
- [ ] Wallets, Profiles and login with Signatures via Wallet
- [ ] Collections, Nfts, Attributes and REST endpoints, etc
- [ ] Notifications via email using Queues
- [ ] Jobs fetching tickets and sending them email
- [ ] Minting NFTs with Blockchain transactions
