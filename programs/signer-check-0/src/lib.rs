use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod signer_check_0 {
    use super::*;

    pub fn log_message_insecure(ctx: Context<LogMessageInsecure>) -> ProgramResult {
        msg!("GM");
        Ok(())
    }

    pub fn log_message_secure(ctx: Context<LogMessageSecure>) -> ProgramResult {
        if !ctx.accounts.authority.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
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
pub struct LogMessageInsecure {}

#[derive(Accounts)]
pub struct LogMessageSecure<'info> {
    authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct LogMessage<'info> {
    authority: Signer<'info>,
}
