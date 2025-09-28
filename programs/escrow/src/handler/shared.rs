use anchor_lang::prelude::*;
use anchor_spl::{token_interface::{TokenAccount, Mint, TransferChecked, transfer_checked, TokenInterface}};


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

// pub fn close_token_account(){

// }