# Richy Token Program — security.txt

[![Security Policy](https://img.shields.io/badge/Security-Policy-red?logo=shield&logoColor=white)](https://github.com/De-ASI-INTERFACE/solana-security-txt-Richy-/blob/master/SECURITY.md)
[![Bounty Program](https://img.shields.io/badge/Bug%20Bounty-Active-brightgreen?logo=hackerone&logoColor=white)](https://github.com/De-ASI-INTERFACE/solana-security-txt-Richy-/security/advisories/new)
[![Audit Status](https://img.shields.io/badge/Audit-Pending-yellow?logo=checkmarx&logoColor=white)](https://github.com/De-ASI-INTERFACE/solana-security-txt-Richy-/blob/master/SECURITY.md)
[![Network](https://img.shields.io/badge/Network-Solana%20Mainnet-9945FF?logo=solana&logoColor=white)](https://solana.com)
[![Organization](https://img.shields.io/badge/Org-De--ASI--INTERFACE-blue?logo=github)](https://github.com/De-ASI-INTERFACE)
[![CI](https://github.com/De-ASI-INTERFACE/solana-security-txt-Richy-/actions/workflows/ci.yml/badge.svg)](https://github.com/De-ASI-INTERFACE/solana-security-txt-Richy-/actions/workflows/ci.yml)
[![security-txt verify](https://github.com/De-ASI-INTERFACE/solana-security-txt-Richy-/actions/workflows/security-txt-verify.yml/badge.svg)](https://github.com/De-ASI-INTERFACE/solana-security-txt-Richy-/actions/workflows/security-txt-verify.yml)
[![](https://img.shields.io/crates/v/solana-security-txt)](https://crates.io/crates/solana-security-txt)
[![](https://docs.rs/solana-security-txt/badge.svg)](https://docs.rs/solana-security-txt/)
[![](https://img.shields.io/crates/v/query-security-txt)](https://crates.io/crates/query-security-txt)

---

**Organization:** De-ASI-INTERFACE  
**Program:** Richy Token Program (SPL Token)  
**Network:** Solana Mainnet-Beta  
**Security Contact:** security@de-asi-interface.io  
**Disclosure:** [GitHub Security Advisories](https://github.com/De-ASI-INTERFACE/solana-security-txt-Richy-/security/advisories/new)

---

This library defines a macro which embeds standardized, machine-parseable security contact information directly into the compiled Solana program binary. It is inspired by <https://securitytxt.org/>.

See the upstream reference implementation: [neodyme-labs/solana-security-txt](https://github.com/neodyme-labs/solana-security-txt)

See an example in the Solana Explorer: <https://explorer.solana.com/address/HPxKXnBN4vJ8RjpdqDCU7gvNQHeeyGnSviYTJ4fBrDt4/security?cluster=devnet>

## Motivation

Users typically interact with a Solana smart contract via the project's web interface, which knows the contract's address. Security researchers often don't.

Having standardized information about your project inside your contract makes it easy for whitehat researchers to reach you if they find any problems.

## Usage

Add the following to the `[dependencies]` section of your Cargo.toml:
```toml
solana-security-txt = "1.1.3"
```

To install the querying tool:
```sh
cargo install query-security-txt
```

To verify a local binary:
```sh
query-security-txt target/sbpf-solana-solana/release/example_contract.so
```

## Security

See [SECURITY.md](../SECURITY.md) for the full disclosure policy, scope, severity tiers, and bounty structure.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE))
* MIT license ([LICENSE-MIT](../LICENSE-MIT))

at your option.
