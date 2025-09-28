use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken,  token_interface::{TokenAccount,Mint, TokenInterface}};

use crate::Offer;
use crate::error::ErrorCode as EscrowErrorCode;
use crate::handler::shared::transfer_tokens;

#[derive(Accounts)]
#[instruction(id:u64)]
pub struct MakeOffer<'info>{
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(mint::token_program = token_program)]
    pub token_mint_a : InterfaceAccount<'info,Mint>,

    #[account(mint::token_program = token_program)]
    pub token_mint_b : InterfaceAccount<'info,Mint>,

    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_token_account_a : InterfaceAccount<'info,TokenAccount>,

    #[account(
        init,
        payer = maker,
        seeds = [
            b"offer",
            id.to_le_bytes().as_ref(),
        ],
        bump,
        space = Offer::DISCRIMINATOR.len() + Offer::INIT_SPACE
    )]
    pub offer : Account<'info , Offer>,


    #[account(
        init,
        payer = maker,
        associated_token::mint = token_mint_a,
        associated_token::authority = offer,
        associated_token::token_program = token_program
    )]
    pub vault : InterfaceAccount<'info,TokenAccount>,

    pub associated_token_program : Program<'info, AssociatedToken>,
    pub system_program : Program<'info, System>,
    pub token_program : Interface<'info , TokenInterface>,
}

pub fn make_offer(
    context:Context<MakeOffer>,
    id:u64,
    token_a_offered_amount : u64,
    token_b_wanted_amount : u64,
)->Result<()>{

    require!(token_a_offered_amount>0, EscrowErrorCode::InvalidOfferedAmount);

    require!(token_b_wanted_amount>0, EscrowErrorCode::InvalidWantedAmount);
    // We need to transfer the token to the vault 

    let offer_account = &mut context.accounts.offer;
    let maker = &context.accounts.maker;
    let token_mint_a = &context.accounts.token_mint_a;
    let token_mint_b = &context.accounts.token_mint_b;
    let maker_token_a_pda = & context.accounts.maker_token_account_a;
    let vault = & context.accounts.vault;
    let token_program: &Interface<'_, TokenInterface> = &context.accounts.token_program;


    transfer_tokens(maker_token_a_pda, vault, &token_a_offered_amount, token_mint_a, &maker, token_program, None)?;

    offer_account.token_a_offered_amount = token_a_offered_amount;
    offer_account.token_b_wanted_amount = token_b_wanted_amount;
    offer_account.token_mint_a = token_mint_a.key();
    offer_account.token_mint_b = token_mint_b.key();
    offer_account.maker = maker.key();
    offer_account.bump = *&context.bumps.offer;
    offer_account.id = id;

    Ok(())
}