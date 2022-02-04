use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("Po2hjSPEQmN9e1YLiZwwL3tCkqMCo2wYyLqAkF7ZmQn");

#[program]
pub mod owner_checks_secure {
    use super::*;

    pub fn log_message(ctx: Context<LogMessage>) -> ProgramResult {
        msg!("Your account balance is: {}", ctx.accounts.token.amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LogMessage<'info> {
    #[account(constraint = authority.key == &token.owner)]
    token: Account<'info, TokenAccount>,
    authority: Signer<'info>,
}