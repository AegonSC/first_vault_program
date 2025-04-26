use anchor_lang::prelude::*;
use crate::VaultAccount;
use crate::CustomError;

// Helper Functions outside the program :v

pub fn send_fees<'info> (
    from: &Signer<'info>,
    to: &SystemAccount<'info>,
    system_program: &Program<'info, System>,
    fee: u64,
) -> Result<()> {

    let fee_cpi_account = anchor_lang::system_program::Transfer {
        from: from.to_account_info(),
        to: to.to_account_info(),
    };

    let fee_cpi_ctx = CpiContext::new(system_program.to_account_info(), fee_cpi_account);
    anchor_lang::system_program::transfer(fee_cpi_ctx, fee)?;
    Ok(())
}

pub fn calculate_fee(amount: u64) -> u64 {
    amount / 100 // 1% fee because money can't buy hapinnes but poverty can't buy anything xdxdx
}

// Essential Requires
pub fn assert_owner(vault: &Account<VaultAccount>, initializer: &Signer) -> Result<()> {
    require!(vault.owner == initializer.key(), CustomError::Unauthorized);
    Ok(())
}

pub fn assert_balance(vault: &Account<VaultAccount>, amount: u64) -> Result<()> {
    require!(vault.balance >= amount, CustomError::InsufficientFunds);
    Ok(())
}