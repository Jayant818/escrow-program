use anchor_lang::prelude::*;
use anchor_spl::{ token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,CloseAccount, close_account}};


pub fn transfer_tokens<'info>(
    from: &InterfaceAccount<'info,TokenAccount>,
    to:&InterfaceAccount<'info,TokenAccount>,
    amount : &u64,
    mint : &InterfaceAccount<'info,Mint>,
    authority : &AccountInfo<'info>,
    token_program : &Interface<'info , TokenInterface>,
    pda_seeds : Option<&[&[u8]]>
)->Result<()>{

    let transfer_accounts = TransferChecked{
        from: from.to_account_info(),
        authority: authority.to_account_info(),
        mint:mint.to_account_info(),
        to:to.to_account_info()
    };

    let signer_seed = pda_seeds.map(|seed| [seed]);

    let cpi_ctx = if let Some(ref seed) = signer_seed{
        CpiContext::new_with_signer(token_program.to_account_info(), transfer_accounts, seed)
    }else{
        CpiContext::new(token_program.to_account_info(),transfer_accounts )
    };
  

    transfer_checked(cpi_ctx, *amount, mint.decimals)?;

    Ok(())
}

// Token_account 
// authority 
// signer_seed
// token_program
pub fn close_token_account<'info>(
    token_account : &InterfaceAccount<'info,TokenAccount>,
    authority: &AccountInfo<'info>,
    destination : &AccountInfo<'info>,
    seeds :Option<&[&[u8]]>,
    token_program: &Interface<'info,TokenInterface>,
)->Result<()>{

    let close_accounts = CloseAccount{
        destination:destination.to_account_info(),
        authority:authority.to_account_info(),
        account:token_account.to_account_info()
    };

    let signer_seeds = seeds.map(|seed| [seed]);

    // CPI TO Close the account and return the rent to dest.

    close_account(
        if let Some(seed_arr) = signer_seeds.as_ref(){
            CpiContext::new_with_signer(token_program.to_account_info(), close_accounts, seed_arr)
        }else{
            CpiContext::new(token_program.to_account_info(), close_accounts)
        }
    )?;

    Ok(())
}