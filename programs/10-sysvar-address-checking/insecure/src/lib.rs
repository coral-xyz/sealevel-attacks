use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod insecure {
    use super::*;

    pub fn check_sysvar_address(ctx: Context<CheckSysvarAddress>) -> Result<()> {
        msg!("Rent Key -> {}", ctx.accounts.rent.key().to_string());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CheckSysvarAddress<'info> {
    rent: AccountInfo<'info>,
}
