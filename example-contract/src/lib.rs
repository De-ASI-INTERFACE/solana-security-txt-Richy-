#![allow(unexpected_cfgs)]
use pinocchio::{
  account_info::AccountInfo,
  entrypoint,
  msg,
  ProgramResult,
  pubkey::Pubkey
};

use solana_security_txt::security_txt;

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    // ── Required fields ───────────────────────────────────────────────────
    name: "Richy Token Program",
    project_url: "https://github.com/De-ASI-INTERFACE",
    contacts: "email:security@de-asi-interface.io,link:https://github.com/De-ASI-INTERFACE/solana-security-txt-Richy-/security/advisories/new,discord:De-ASI-INTERFACE",
    policy: "https://github.com/De-ASI-INTERFACE/solana-security-txt-Richy-/blob/master/SECURITY.md",

    // ── Optional fields ───────────────────────────────────────────────────
    preferred_languages: "en",
    source_code: "https://github.com/De-ASI-INTERFACE/solana-security-txt-Richy-",
    auditors: "Unaudited — internal review only. Third-party audit pending.",
    acknowledgements: "
No external vulnerabilities reported to date.
Responsible disclosures will be acknowledged here.
"
}

entrypoint!(process_instruction);

pub fn process_instruction(
  _program_id: &Pubkey,
  _accounts: &[AccountInfo],
  _instruction_data: &[u8],
) -> ProgramResult {
  msg!("Richy Token Program — De-ASI-INTERFACE");
  Ok(())
}
