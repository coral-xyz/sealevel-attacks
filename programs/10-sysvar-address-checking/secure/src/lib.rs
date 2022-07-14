use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod secure {
    use super::*;

    pub fn check_sysvar_address(ctx: Context<CheckSysvarAddress>) -> Result<()> {
        require_eq!(ctx.accounts.rent.key(), sysvar::rent::ID);
        msg!("Rent Key -> {}", ctx.accounts.rent.key().to_string());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CheckSysvarAddress<'info> {
    rent: AccountInfo<'info>,
}
