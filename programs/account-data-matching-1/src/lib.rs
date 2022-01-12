use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_pack::Pack;
use anchor_spl::token::TokenAccount;
use spl_token::state::Account as SplTokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod account_data_matching_1 {
    use super::*;

    pub fn log_message_insecure(ctx: Context<LogMessageInsecure>) -> ProgramResult {
        let _token = SplTokenAccount::unpack(&ctx.accounts.token.data.borrow())?;
        msg!("GM");
        Ok(())
    }

    pub fn log_message_secure(ctx: Context<LogMessageSecure>) -> ProgramResult {
        let token = SplTokenAccount::unpack(&ctx.accounts.token.data.borrow())?;
        if ctx.accounts.authority.key != &token.owner {
            return Err(ProgramError::InvalidAccountData);
        }
        msg!("GM");
        Ok(())
    }

    pub fn log_message(ctx: Context<LogMessage>) -> ProgramResult {
        msg!("GM");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LogMessageInsecure<'info> {
    token: AccountInfo<'info>,
    authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct LogMessageSecure<'info> {
    token: AccountInfo<'info>,
    authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct LogMessage<'info> {
    #[account(constraint = authority.key == &token.owner)]
    token: Account<'info, TokenAccount>,
    authority: Signer<'info>,
}
