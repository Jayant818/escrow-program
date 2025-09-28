use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::{transfer_tokens, Offer, error::ErrorCode as EscrowErrorCode, close_token_account};

#[derive(Accounts)]
pub struct RefundOffer<'info>{
    #[account(mut)]
    pub maker:Signer<'info>,

    pub token_mint_a : InterfaceAccount<'info,Mint>,

    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_ata_a: InterfaceAccount<'info,TokenAccount>,


    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = offer,
        associated_token::token_program = token_program,
    )]
    pub vault : InterfaceAccount<'info , TokenAccount>,

    #[account(
        mut,
        seeds = [
            b"offer",
            offer.id.to_le_bytes().as_ref(),
        ],
        close = maker,
        has_one = maker,
        bump = offer.bump
    )]
    pub offer: Account<'info , Offer>,

    pub token_program : Interface<'info,TokenInterface>,
    pub system_program : Program<'info, System>,
    pub ata_program : Program<'info , AssociatedToken>,
}

pub fn refund_offer(context:Context<RefundOffer>)->Result<()>{

    let offer = &context.accounts.offer;

    let seeds = &[
        b"offer",
        &context.accounts.offer.id.to_le_bytes()[..],
        &[offer.bump],
    ];

    let signer_seeds  = Some(&seeds[..]);

    // Withdraw  the balance from vault to maker ata 
    transfer_tokens(
        &context.accounts.vault, 
        &context.accounts.maker_ata_a, 
        &offer.token_a_offered_amount, 
        &context.accounts.token_mint_a, 
        &offer.to_account_info(), 
        &context.accounts.token_program, 
        signer_seeds
    ).map_err(|_| EscrowErrorCode::FailedVaultWithdrawal)?;

    // Close the account 
    close_token_account(
        &context.accounts.vault, 
        &context.accounts.offer.to_account_info(), 
        &context.accounts.maker, 
        signer_seeds, 
        &context.accounts.token_program
    ).map_err(|_| EscrowErrorCode::FailedVaultClosure)?;

    Ok(())
}

