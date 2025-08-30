use anchor_lang::prelude::*;

declare_id!("7YpwH7S3qVa3UnmeMs8m2bjq8Nqt9nbjVcbifsfZWQgK");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn deposit(ctx : Context<VaultAction>, amount : u64)-> Result<()>{
        //check if vault is empty
        require_eq!(ctx.accounts.vault.lamports, 0, VaultError::VaultAlreadyExists);
        
        //Ensure amount exceeds rent-exempt minimum
        require_eq!(amount, Rent::get()?.minimum_balance(0), VaultError::InvalidAmount);
        
        Ok(())

    }

    pub fn withdraw(ctx : Context<VaultAction>, amount : u64)-> Result<()>{
        Ok(())
    }

    // pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    //     msg!("Greetings from: {:?}", ctx.program_id);
    //     Ok(())
    // }
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
