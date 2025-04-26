#![allow(unused)]

use anchor_lang::prelude::*;

declare_id!("DsAYjtNg2xrV1xVcXNdXGB49mCJAfUnubq1jsZMNuawn");

pub mod events;
use crate::events::{DepositEvent, WithdrawEvent, CloseVaultEvent};

pub mod helpers;
use crate::helpers::{send_fees, calculate_fee, assert_owner, assert_balance};



#[program]
pub mod my_vault_program {
    use super::*;

    pub fn init_vault(ctx: Context<InitVault>) -> Result<()> {
        ctx.accounts.vault.balance = 0;
        ctx.accounts.vault.owner = ctx.accounts.initializer.key();
        Ok(())
    }


    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        assert_owner(&ctx.accounts.vault, &ctx.accounts.initializer)?;

        let fee = calculate_fee(amount);
        let net_amount = amount - fee;
        

        send_fees(
            &ctx.accounts.initializer, // from
            &ctx.accounts.fee_collector, // to
            &ctx.accounts.system_program, // system_program
            fee, // fee calculated above
        );

        // Transfer the remaining amount to the vault
        let cpi_accounts = anchor_lang::system_program::Transfer {
            from: ctx.accounts.initializer.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(ctx.accounts.system_program.to_account_info(), cpi_accounts);
        anchor_lang::system_program::transfer(cpi_ctx, amount)?;


        // Update the vault's balance
        ctx.accounts.vault.balance += amount;

        emit!(DepositEvent {
            initializer: ctx.accounts.initializer.key(),
            amount,
        });

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        assert_owner(&ctx.accounts.vault, &ctx.accounts.initializer)?;

        // Calculating the fee and total amount
        let fee = calculate_fee(amount);
        let total_amount = amount + fee;

        // Ensure vault has enough balance for de withdrawal
        assert_balance(&ctx.accounts.vault, total_amount)?;


        // Transfer the fee to the fee collector
        send_fees(
            &ctx.accounts.initializer, // from
            &ctx.accounts.fee_collector, // to
            &ctx.accounts.system_program, // system_program
            fee, // fee calculated above
        );

        // Transfer the remaining amount to the initializer
        let cpi_accounts = anchor_lang::system_program::Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.initializer.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(ctx.accounts.system_program.to_account_info(), cpi_accounts);
        
        anchor_lang::system_program::transfer(cpi_ctx, amount)?;


        // Updating the vault's balance
        ctx.accounts.vault.balance -= amount;

        emit!(WithdrawEvent {
            initializer: ctx.accounts.initializer.key(),
            amount,
        });

        Ok(())
    }


    pub fn close_vault(ctx: Context<CloseVault>) -> Result<()> {
        assert_owner(&ctx.accounts.vault, &ctx.accounts.initializer)?;

        let remaining_balance = ctx.accounts.vault.balance;

        if (remaining_balance > 0) {

            let cpi_accounts = anchor_lang::system_program::Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.initializer.to_account_info(),
            };

            let cpi_ctx = CpiContext::new(ctx.accounts.system_program.to_account_info(), cpi_accounts);

            anchor_lang::system_program::transfer(cpi_ctx, remaining_balance)?;

            ctx.accounts.vault.balance = 0;

            emit!(CloseVaultEvent {
                initializer: ctx.accounts.initializer.key(),
                amount: remaining_balance,
            });
        }
        Ok(())
    }

    pub fn transfer_ownership(ctx: Context<TransferOwnership>, new_owner: Pubkey) -> Result<()> {
        assert_owner(&ctx.accounts.vault, &ctx.accounts.initializer)?;
        ctx.accounts.vault.owner = new_owner;

        Ok(())
    }

}


// VaultAccount structure
#[account]
pub struct VaultAccount {
    pub balance: u64,
    pub owner: Pubkey,
}

// Context for initializing the vault
#[derive(Accounts)]
pub struct InitVault<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,

    #[account(
        init,
        seeds = [b"vault", initializer.key().as_ref()],
        bump,
        payer = initializer,
        space = 8 + 8 + 32, // Space for VaultAccount: discriminator + balance + owner
    )]
    pub vault: Account<'info, VaultAccount>,

    pub system_program: Program<'info, System>,
}

// Context for depositing funds
#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,

    #[account(mut,
        seeds = [b"vault", initializer.key().as_ref()],
        bump)]
    pub vault: Account<'info, VaultAccount>,

    #[account(mut)]
    pub fee_collector: SystemAccount<'info>, // The account to collect fees

    pub system_program: Program<'info, System>,
}


// Context for withdrawing funds
#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,

    #[account(mut,
        seeds = [b"vault", initializer.key().as_ref()],
        bump,)]
    pub vault: Account<'info, VaultAccount>,

    #[account(mut)]
    pub fee_collector: SystemAccount<'info>, // The account to collect fees

    pub system_program: Program<'info, System>,

}

// Context for closing the vault
#[derive(Accounts)]
pub struct CloseVault<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,

    #[account(mut, 
        seeds = [b"vault", initializer.key().as_ref()],
        bump,
        close = initializer,)]
    pub vault: Account<'info, VaultAccount>,

    pub system_program: Program<'info, System>,
}


// Context for transferring ownership
#[derive(Accounts)]
pub struct TransferOwnership<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,

    #[account(mut,
        seeds = [b"vault", initializer.key().as_ref()],
        bump)]
    pub vault: Account<'info, VaultAccount>,

    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum CustomError {
    #[msg("You dont have permission to do this")]
    Unauthorized,
    #[msg("Insufficient funds")]
    InsufficientFunds,
}