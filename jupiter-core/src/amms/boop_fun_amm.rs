use anyhow::Result;
use bincode::deserialize;
use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};
use jupiter_amm_interface::{AccountMap, Amm, AmmContext, KeyedAccount, Quote, QuoteParams, SwapMode};
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;

pub const BOOP_FUN_PROGRAM: Pubkey = pubkey!("boop8hVGQGqehUK2iVEMEnMrL5RbjywRzHKBmBE7ry4");
pub const SOL_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
// 1 billion tokens
pub const TOKEN_TOTAL_SUPPLY: u64 = 1_000_000_000_000_000_000;

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

impl BondingCurve {
  pub fn calculate_token_amount_out(&self, sol_amount: u64) -> u64 {
      let scaling_factor = (self.damping_term as u128)
          .checked_mul(self.virtual_token_reserves as u128)
          .unwrap()
          .checked_mul(LAMPORTS_PER_SOL as u128)
          .unwrap();

      let initial_sol = (self.virtual_sol_reserves as u128)
          .checked_add(self.sol_reserves as u128)
          .unwrap();

      let amount_out = scaling_factor
          .checked_div(initial_sol)
          .unwrap()
          .checked_sub(
              scaling_factor
                  .checked_div(initial_sol.checked_add(sol_amount as u128).unwrap())
                  .unwrap(),
          )
          .unwrap();

      amount_out as u64
  }

  pub fn calculate_sol_amount_out(&self, token_amount: u64) -> u64 {
      let scaling_factor = (self.damping_term as u128)
          .checked_mul(self.virtual_token_reserves as u128)
          .unwrap()
          .checked_mul(LAMPORTS_PER_SOL as u128)
          .unwrap();

      let tokens_issued = TOKEN_TOTAL_SUPPLY.checked_sub(self.token_reserves).unwrap() as u128;

      let base_denominator = (self.virtual_token_reserves as u128)
          .checked_sub(tokens_issued)
          .unwrap();

      let amount_out = scaling_factor
          .checked_div(base_denominator)
          .unwrap()
          .checked_sub(
              scaling_factor
                  .checked_div(base_denominator.checked_add(token_amount as u128).unwrap())
                  .unwrap(),
          )
          .unwrap();

      amount_out as u64
  }

  pub fn calculate_swap_fee(&self, amount: u64) -> u64 {
    amount
        .checked_mul(self.swap_fee_basis_points as u64)
        .unwrap()
        .checked_div(10_000)
        .unwrap()
  }
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

    fn quote(&self, quote_params: &QuoteParams) -> Result<Quote> {
      let in_amount = quote_params.amount;
      let bonding_curve_mint = self.bonding_curve.mint;
      let input_mint = quote_params.input_mint;
      let output_mint = quote_params.output_mint;
      let swap_mode = quote_params.swap_mode;

      match swap_mode {
        SwapMode::ExactIn => {
          let is_buy = input_mint == SOL_MINT && output_mint == bonding_curve_mint;

          if is_buy {
            let fee_amount = self.bonding_curve.calculate_swap_fee(in_amount);
            // TODO: Actual buy amount can be smaller
            Ok(Quote {
              in_amount,
              out_amount: self.bonding_curve.calculate_token_amount_out(in_amount.checked_sub(fee_amount).unwrap()),
              fee_amount,
              fee_mint: SOL_MINT,
              fee_pct: Decimal::from(self.bonding_curve.swap_fee_basis_points),
              ..Quote::default()
            })
          } else if input_mint == bonding_curve_mint && output_mint == SOL_MINT {
            let out_amount = self.bonding_curve.calculate_sol_amount_out(in_amount);
            let fee_amount = self.bonding_curve.calculate_swap_fee(out_amount);

            Ok(Quote {
              in_amount,
              out_amount,
              fee_amount,
              fee_mint: SOL_MINT,
              fee_pct: Decimal::from(self.bonding_curve.swap_fee_basis_points),
              ..Quote::default()
            })
          } else {
            return Err(anyhow::anyhow!("Invalid quote params"));
          }
        },
        SwapMode::ExactOut => {
          return Err(anyhow::anyhow!("ExactOut is not supported"));
        }
      }
    }
}