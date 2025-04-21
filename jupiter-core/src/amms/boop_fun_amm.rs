use anyhow::Result;
use bincode::deserialize;
use serde::{Serialize, Deserialize};
use jupiter_amm_interface::{AccountMap, Amm, AmmContext, KeyedAccount};
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;

pub const BOOP_FUN_PROGRAM: Pubkey = pubkey!("boop8hVGQGqehUK2iVEMEnMrL5RbjywRzHKBmBE7ry4");
pub const SOL_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");

pub struct BoopFunAmm {
    key: Pubkey,
    bonding_curve: BondingCurve,
}

#[derive(Serialize, Deserialize)]
pub struct BondingCurve {
    pub creator: Pubkey,
    pub mint: Pubkey,
    pub virtual_sol_reserves: u64,
    pub virtual_token_reserves: u64,
    pub graduation_target: u64,
    pub graduation_fee: u64,
    pub sol_reserves: u64,
    pub token_reserves: u64,
    pub damping_term: u8,
    pub swap_fee_basis_points: u8,
    pub token_for_stakers_basis_points: u16,
    pub status: u8,
}

// TODO: impl Amm for BoopFunAmm
impl BoopFunAmm {
    fn from_keyed_account(keyed_account: &KeyedAccount, amm_context: &AmmContext) -> Result<Self> {
        // First 8 bytes are the discriminator
        let byte_array = &keyed_account.account.data[8..];
        let bonding_curve: BondingCurve = deserialize(byte_array).unwrap();
        Ok(BoopFunAmm { key: keyed_account.key, bonding_curve })
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

    fn get_reserve_mints(&self) -> Vec<Pubkey> {
        vec![self.bonding_curve.mint, SOL_MINT]
    }

    // The bonding curve keeps an internal state of the reserves without
    // using the token accounts' balances.
    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        vec![]
    }

    fn update(&mut self, account_map: &AccountMap) -> Result<()> {
        Ok(())
    }
}