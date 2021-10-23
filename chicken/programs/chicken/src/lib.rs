use anchor_lang::prelude::*;

declare_id!("x1UTZfeybkzAzTHZxswXBvyLMk3drMNQGsrBMmYtCSN");

#[program]
pub mod chicken {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
