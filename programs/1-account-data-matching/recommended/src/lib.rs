use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod account_data_matching_recommended {
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
