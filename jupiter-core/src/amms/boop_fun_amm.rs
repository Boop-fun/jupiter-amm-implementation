use anyhow::Result;
use jupiter_amm_interface::{AmmContext, KeyedAccount};
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;

pub const BOOP_FUN_PROGRAM: Pubkey = pubkey!("boop8hVGQGqehUK2iVEMEnMrL5RbjywRzHKBmBE7ry4");

pub struct BoopFunAmm {
}

// TODO: impl Amm for BoopFunAmm
impl BoopFunAmm {
    fn from_keyed_account(keyed_account: &KeyedAccount, amm_context: &AmmContext) -> Result<Self> {
        Ok(BoopFunAmm { })
    }
}