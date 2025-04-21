use anyhow::Result;
use jupiter_amm_interface::{Amm, AmmContext, KeyedAccount};
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;

pub const BOOP_FUN_PROGRAM: Pubkey = pubkey!("boop8hVGQGqehUK2iVEMEnMrL5RbjywRzHKBmBE7ry4");

pub struct BoopFunAmm {
    key: Pubkey,
}

// TODO: impl Amm for BoopFunAmm
impl BoopFunAmm {
    fn from_keyed_account(keyed_account: &KeyedAccount, amm_context: &AmmContext) -> Result<Self> {
        Ok(BoopFunAmm { key: keyed_account.key })
    }

    fn label(&self) -> String {
        "boop.fun".to_string()
    }

    fn program_id(&self) -> Pubkey {
        BOOP_FUN_PROGRAM
    }

    fn key(&self) -> Pubkey {
        self.key
    }
}