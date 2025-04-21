use anchor_lang::prelude::{AccountMeta, Pubkey};

#[derive(Copy, Clone, Debug)]
pub struct BoopFunBuyToken {
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub trading_fees_vault: Pubkey,
    pub bonding_curve_vault: Pubkey,
    pub bonding_curve_sol_vault: Pubkey,
    pub recipient_token_account: Pubkey,
    pub buyer: Pubkey,
    pub recipient: Pubkey,
    pub config: Pubkey,
    pub vault_authority: Pubkey,
    pub wsol: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
}

impl From<BoopFunBuyToken> for Vec<AccountMeta> {
    fn from(accounts: BoopFunBuyToken) -> Self {
        vec![
            AccountMeta::new_readonly(accounts.mint, false),
            AccountMeta::new(accounts.bonding_curve, false),
            AccountMeta::new(accounts.trading_fees_vault, false),
            AccountMeta::new(accounts.bonding_curve_vault, false),
            AccountMeta::new(accounts.bonding_curve_sol_vault, false),
            AccountMeta::new(accounts.recipient_token_account, false),
            AccountMeta::new(accounts.buyer, true),
            AccountMeta::new_readonly(accounts.recipient, false),
            AccountMeta::new_readonly(accounts.config, false),
            AccountMeta::new_readonly(accounts.vault_authority, false),
            AccountMeta::new_readonly(accounts.wsol, false),
            AccountMeta::new_readonly(accounts.system_program, false),
            AccountMeta::new_readonly(accounts.token_program, false),
            AccountMeta::new_readonly(accounts.associated_token_program, false),
        ]
    }
}

#[derive(Copy, Clone, Debug)]
pub struct BoopFunSellToken {
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub trading_fees_vault: Pubkey,
    pub bonding_curve_vault: Pubkey,
    pub bonding_curve_sol_vault: Pubkey,
    pub seller_token_account: Pubkey,
    pub seller: Pubkey,
    pub recipient: Pubkey,
    pub config: Pubkey,
    pub vault_authority: Pubkey,
    pub wsol: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
}

impl From<BoopFunSellToken> for Vec<AccountMeta> {
    fn from(accounts: BoopFunSellToken) -> Self {
        vec![
            AccountMeta::new_readonly(accounts.mint, false),
            AccountMeta::new(accounts.bonding_curve, false),
            AccountMeta::new(accounts.trading_fees_vault, false),
            AccountMeta::new(accounts.bonding_curve_vault, false),
            AccountMeta::new(accounts.bonding_curve_sol_vault, false),
            AccountMeta::new(accounts.seller_token_account, false),
            AccountMeta::new(accounts.seller, true),
            AccountMeta::new_readonly(accounts.recipient, false),
            AccountMeta::new_readonly(accounts.config, false),
            AccountMeta::new_readonly(accounts.vault_authority, false),
            AccountMeta::new_readonly(accounts.wsol, false),
            AccountMeta::new_readonly(accounts.system_program, false),
            AccountMeta::new_readonly(accounts.token_program, false),
            AccountMeta::new_readonly(accounts.associated_token_program, false),
        ]
    }
}