use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

declare_id!("22222222222222222222222222222222222222222222");

#[program]
pub mod anchor_vault {
    use anchor_lang::system_program::transfer;

    use super::*;

    pub fn deposit(ctx : Context<VaultAction>, amount : u64)-> Result<()>{
        //check if vault is empty
        require_eq!(ctx.accounts.vault.lamports(), 0, VaultError::VaultAlreadyExists);
        
        //Ensure amount exceeds rent-exempt minimum
        require_gt!(amount, Rent::get()?.minimum_balance(0), VaultError::InvalidAmount);

        let cpi_accounts = Transfer{
            from : ctx.accounts.signer.to_account_info(),
            to : ctx.accounts.vault.to_account_info()
        };

        let cpi_program = ctx.accounts.system_program.to_account_info();    //system program in this case

        let cpi_context = CpiContext::new(
            cpi_program, 
            cpi_accounts
        );

        let _ = transfer(
            cpi_context,
            amount
        );
        
        Ok(())

    }

    pub fn withdraw(ctx : Context<VaultAction>)-> Result<()>{

        let amount = ctx.accounts.vault.lamports();
        //check if vault has any lamports
        require!(amount > 0, VaultError::InvalidAmount);

        
        let signer_key = ctx.accounts.signer.key();
        let seeds = &[b"vault", signer_key.as_ref(), &[ctx.bumps.vault]];
        let signer_seeds = &[&seeds[..]];

        let cpi_accounts = Transfer{
            from: ctx.accounts.vault.to_account_info(),
            to : ctx.accounts.signer.to_account_info()
        };

        let cpi_program = ctx.accounts.system_program.to_account_info();

        let cpi_context = CpiContext::new_with_signer(
            cpi_program,
            cpi_accounts,
            signer_seeds
        );

        let _ = transfer(
            cpi_context,
            amount
        )?;


        Ok(())
    }
}

#[derive(Accounts)]
pub struct VaultAction<'info>{
    #[account(mut)]
    pub signer : Signer<'info>,
    #[account(
        mut, 
        seeds = [b"vault", signer.key().as_ref()],
        bump
    )]
    pub vault : SystemAccount<'info>,
    pub system_program : Program<'info, System>
}

#[error_code]
pub enum VaultError{
    //error 
    #[msg("Vault Already Exists")]
    VaultAlreadyExists,

    #[msg("Invalid Amount")]
    InvalidAmount
}

