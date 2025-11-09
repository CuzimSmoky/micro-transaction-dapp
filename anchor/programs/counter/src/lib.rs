#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

declare_id!("FqzkXZdwYjurnUKetJCAvaUw5WAqbwzU6gZEwydeEfqS");

#[program]
pub mod micropayments {
    use super::*;

    pub fn make_offer(
        context: Context<MakeOffer>,
        id: u64,
        token_a_offered_amount: u64,
        token_b_wanted_amount: u64,
    ) -> Result<()> {
        Ok(())
    }

    pub fn take_offer(context: Context<TakeOffer>) -> Result <()> {
        Ok(())
    }

    pub fn refund_offer(context: Context<RefundOffer>) -> Reult <()> {
        Ok(())
    }

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct MakeOffer<'info> {
    // Used to manage associated token accounts
    // ie where a wallet holds a specific type of token
    pub associated_token_program: Program<'info, AssociatedToken>,

    // Work with either the classic token program or
    // the newer token extensions program
    pub token_program: Interface<'info, TokenInterface>,

    // Used to create accounts
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(mint::token_program = token_program)]
    pub token_mint_a: InterfaceAccount<'info, Mint>,

    #[account(mint::token_program = token_program)]
    pub token_mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_token_account_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = maker,
        space = Offer::DISCRIMINATOR.len() + Offer::INIT_SPACE,
        seeds = [b"offer", id.to_le_bytes().as_ref()],
        bump
    )]
    pub offer: Account<'info, Offer>,

    #[account(
        init,
        payer = maker,
        associated_token::mint = token_mint_a,
        associated_token::authority = offer,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
}
}
