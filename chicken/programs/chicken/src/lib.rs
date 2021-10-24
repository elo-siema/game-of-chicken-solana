use anchor_lang::prelude::*;
use anchor_spl::token::*;
mod error;
declare_id!("x1UTZfeybkzAzTHZxswXBvyLMk3drMNQGsrBMmYtCSN");

#[program]
pub mod chicken {
    use crate::error::ChickenError;

    use super::*;

    /// Initialize the game, setting expected start date,
    /// end date, and amount of SOL staked.
    pub fn initialize(
        ctx: Context<Initialize>,
        start_timestamp: i64,
        end_timestamp: i64,
        amount: u64,
    ) -> ProgramResult {
        let clock = Clock::get()?;
        let timestamp = clock.unix_timestamp;
        msg!("Unix timestamp: {:?}", timestamp);
        msg!("Start timestamp: {:?}", start_timestamp);
        msg!("End timestamp: {:?}", end_timestamp);

        // validate inputs
        if start_timestamp < timestamp {
            return Err(ChickenError::StartDateInThePast.into());
        } 
        if end_timestamp < timestamp {
            return Err(ChickenError::EndDateInThePast.into());
        } 
        if end_timestamp < start_timestamp {
            return Err(ChickenError::StartDateAfterEnd.into());
        } 

        // inputs valid, initialize state
        let gda = &mut ctx.accounts.game_data_account;
        gda.start_timestamp = start_timestamp;
        gda.end_timestamp = end_timestamp;
        gda.entry_amount = amount;
        gda.accepted_at = None;
        gda.withdrawn_at = None;
        gda.initializer = *ctx.accounts.player1.key;

        

        // transfer player1's entry stake
        anchor_spl::token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.player1.to_account_info(),
                    to: ctx.accounts.game_data_account.to_account_info(),
                    authority: ctx.accounts.player1.to_account_info(),
                },
            ),
            amount,
        )?;

        msg!("Transferred {:?} SOL from player1 to the pool", amount);

        Ok(())
    }

    pub fn withdraw(
        ctx: Context<Withdraw>,
    ) -> ProgramResult {
        Ok(())
    }
    
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = player1, space = 64+64+64+64+64+8+8+64)]
    pub game_data_account: Account<'info, GameDataAccount>,
    #[account(mut, signer)]
    pub player1: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(init, payer = user, space = 64 + 64)]
    pub game_data_account: Account<'info, GameDataAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[account]
#[derive(Default)]
pub struct GameDataAccount {
    //When the game is set to start
    pub start_timestamp: i64,
    //When the game is set to end
    pub end_timestamp: i64,
    //Required entry from players
    pub entry_amount: u64,
    pub initializer: Pubkey,

    //When player 2 joined the game
    pub accepted_at: Option<i64>,

    //When any player made their first withdrawal
    pub withdrawn_at: Option<i64>,
}
