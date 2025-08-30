use anchor_lang::prelude::*;

declare_id!("7YpwH7S3qVa3UnmeMs8m2bjq8Nqt9nbjVcbifsfZWQgK");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
