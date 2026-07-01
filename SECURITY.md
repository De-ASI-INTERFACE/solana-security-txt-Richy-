# Security Policy — Richy Token Program

**Organization:** De-ASI-INTERFACE  
**Program:** Richy Token Program (SPL Token)  
**Network:** Solana Mainnet-Beta  
**Last Updated:** 2026-07-01  

---

## Supported Versions

Only the most recently deployed and verified on-chain version of the Richy Token Program is in scope.

| Version | Status     | In Scope |
|---------|------------|----------|
| Latest  | Active     | ✅ Yes   |
| Prior   | Deprecated | ❌ No    |

---

## Reporting a Vulnerability

Please **do not disclose publicly** until we have had a reasonable opportunity to investigate and remediate.

### Preferred Disclosure Channels (in order of preference)

1. **GitHub Private Security Advisory** *(preferred)*  
   https://github.com/De-ASI-INTERFACE/solana-security-txt-Richy-/security/advisories/new

2. **Email**  
   security@de-asi-interface.io  
   Please encrypt sensitive reports using our PGP key (available upon request).

3. **Discord**  
   De-ASI-INTERFACE  
   Use only for initial contact — do not share exploit details over Discord.

---

## Response SLA

| Milestone                        | Target Timeframe                                                         |
|----------------------------------|--------------------------------------------------------------------------|
| Acknowledgement of report        | 48 hours                                                                 |
| Initial triage and severity call | 5 business days                                                          |
| Patch development complete       | 14 business days (Critical), 30 days (High/Medium)                      |
| Public disclosure (coordinated)  | 90 days post-report or after patch deployment, whichever comes first     |

We follow a **coordinated vulnerability disclosure (CVD)** model and will credit you appropriately.

---

## Scope

### In Scope

- Richy Token Program on-chain logic (mint, transfer, burn, freeze authority)
- SPL Token account ownership or authority escalation
- Unauthorized minting or token supply manipulation
- Reentrancy or cross-program invocation (CPI) exploits
- Account confusion or missing signer checks
- Arithmetic overflow/underflow vulnerabilities
- Denial-of-service vectors against program execution

### Out of Scope

- Front-end or off-chain UI issues not directly tied to on-chain fund loss
- Social engineering or phishing attacks
- Third-party Solana RPC infrastructure issues
- Known Solana runtime or BPF VM bugs already reported to Solana Labs
- Issues in undeployed or devnet program IDs
- Spam or low-signal reports with no demonstrated impact

---

## Severity Tiers & Bounty Structure

Bounties are awarded at the sole discretion of De-ASI-INTERFACE based on impact, report quality, and novelty.

| Severity    | Description                                                                 | Bounty Range (USD equiv. in SOL) |
|-------------|-----------------------------------------------------------------------------|----------------------------------|
| 🔴 Critical | Direct loss of user funds, unauthorized mint, full authority escalation     | $5,000 – $25,000                 |
| 🟠 High     | Partial fund loss, freeze authority bypass, significant DoS                 | $1,000 – $5,000                  |
| 🟡 Medium   | Logic errors with limited financial impact, privilege escalation edge cases | $250 – $1,000                    |
| 🟢 Low      | Informational issues, minor deviations from spec, low-impact bugs           | $50 – $250                       |

> Bounties are paid in SOL at the 7-day VWAP at time of payout. We reserve the right to adjust amounts based on report quality, proof-of-concept completeness, and whether the vulnerability was previously known.

---

## Safe Harbor

De-ASI-INTERFACE will not pursue legal action against researchers who:

- Report vulnerabilities through the channels defined above
- Do not exploit findings beyond what is necessary to demonstrate the vulnerability
- Do not access, modify, or exfiltrate user data or funds
- Act in good faith and give us reasonable time to respond before public disclosure

---

## Acknowledgements

The following researchers have contributed to the security of this program through responsible disclosure:

*No disclosures on record yet. Be the first.*

---

## Legal

This policy does not create a contract or any legal obligation on the part of De-ASI-INTERFACE beyond what is stated herein. We reserve the right to modify this policy at any time.
