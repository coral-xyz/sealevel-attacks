use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod owner_checks_1 {
    use super::*;

    pub fn create_user(ctx: Context<CreateUser>) -> ProgramResult {
        let user = &mut ctx.accounts.user;
        user.authority = ctx.accounts.authority.key();
        Ok(())
    }

    pub fn log_message(ctx: Context<LogMessage>) -> ProgramResult {
        let token = &ctx.accounts.token;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateUser<'info> {
    #[account(init, payer = authority, space = 8+32)]
    user: Account<'info, User>,
    #[account(init, payer = authority, space = TokenAccount::LEN)]
    token: Account<'info, TokenAccount>,
    #[account(mut)]
    authority: Signer<'info>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct LogMessage<'info> {
    token: AccountInfo<'info>,
}

#[account]
pub struct User {
    authority: Pubkey,
}
