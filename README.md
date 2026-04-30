# QuickPay PH
Instant escrow payments for freelancers using Stellar.

## Problem
Freelancers in Manila experience delayed payments and high fees.

## Solution
Escrow-based instant USDC payments using Stellar.

## Timeline
Hackathon MVP: 1–2 days

## Stellar Features
- USDC transfers
- Soroban smart contracts
- Trustlines

## Vision
Faster, cheaper freelance payments in Southeast Asia.

## Prerequisites
- Rust
- Soroban CLI

## Build
soroban contract build

## Test
cargo test

## Deploy
soroban contract deploy

## Example Call
soroban contract invoke --id <ID> --fn create_escrow --arg ...

## Deployed Contract Link 
[1] https://stellar.expert/explorer/testnet/tx/6fc32236341064708836ec6b3cba01e3e621b50531eba5427303c5b9fab0b077
[2] https://lab.stellar.org/r/testnet/contract/CAABSCND2AEPZODEF6JMMT7RWWIUSOWJFLI4LZEXYDDRBWSD2LEK2EZY

## License
MIT