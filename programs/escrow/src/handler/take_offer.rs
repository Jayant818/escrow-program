use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::Offer;

use crate::error::ErrorCode as EscrowErrorCode;

use crate::handler::shared::{transfer_tokens,close_token_account};


#[derive(Accounts)]
pub struct TakeOffer<'info>{

    #[account(mut)]
    pub taker : Signer<'info>,

    pub maker: SystemAccount<'info>,

    pub token_mint_a : InterfaceAccount<'info,Mint>,

    pub token_mint_b : InterfaceAccount<'info,Mint>,

    // We will get Tokens in this
    #[account(
        init_if_needed,
        payer= taker,
        associated_token::mint = token_mint_a,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    pub taker_token_account_a : InterfaceAccount<'info,TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token_mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    pub taker_token_account_b : InterfaceAccount<'info,TokenAccount>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = token_mint_b,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_token_account_b : InterfaceAccount<'info,TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = offer,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info,TokenAccount>,

    #[account(
        mut,
        close = maker,
        has_one = maker,
        has_one = token_mint_b,
        seeds = [b"offer", offer.id.to_le_bytes().as_ref()],
        bump = offer.bump,

    )]
    pub offer : Account<'info,Offer>,

    pub system_program : Program<'info,System>,

    pub token_program : Interface<'info, TokenInterface>,

    pub associated_token_program : Program<'info, AssociatedToken>,
}

pub fn take_offer(
    context:Context<TakeOffer>,
)->Result<()>{
    //  we need to somehow check that taker ata of mint_b has enough tokens 
    let offer = &context.accounts.offer;

    // 1) Transfer from vault to taker ata of mint_a, vault is owned by offer so we need offer seeds 
    let offer_account_seeds  = &[
        b"offer",
        &context.accounts.offer.id.to_le_bytes()[..],
        &[offer.bump]
    ];

    let signer_seeds = Some(&offer_account_seeds[..]);

    transfer_tokens(
        &context.accounts.vault,
        &context.accounts.taker_token_account_a,
        &offer.token_a_offered_amount, 
        &context.accounts.token_mint_a, 
        &offer.to_account_info(), 
        &context.accounts.token_program, 
        signer_seeds
    ).map_err(|_| EscrowErrorCode::FailedVaultWithdrawal)?;

    // close the vault and return the rent to the maker
    close_token_account(
        &context.accounts.vault, 
        &offer.to_account_info(), 
        &context.accounts.maker, 
        signer_seeds, 
        &context.accounts.token_program
    ).map_err(|_| EscrowErrorCode::FailedVaultClosure)?;

    // Now transfer from taker_ata_b to maker_ata_b

    transfer_tokens(
        &context.accounts.taker_token_account_b, 
        &context.accounts.maker_token_account_b, 
        &context.accounts.offer.token_b_wanted_amount, 
        &context.accounts.token_mint_b, 
        &context.accounts.taker, 
        &context.accounts.token_program, 
        None
    ).map_err(|_| EscrowErrorCode::InsufficientTakerBalance)?;


    Ok(())
}

