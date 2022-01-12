use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod duplicate_mutable_accounts_6 {
    use super::*;
    pub fn update_insecure(ctx: Context<UpdateInsecure>, a: u64, b: u64) -> ProgramResult {
        let user_a = &mut ctx.accounts.user_a;
        let user_b = &mut ctx.accounts.user_b;

        user_a.data = a;
        user_b.data = b;
        Ok(())
    }

    pub fn update_secure(ctx: Context<UpdateInsecure>, a: u64, b: u64) -> ProgramResult {
        let user_a = &mut ctx.accounts.user_a;
        let user_b = &mut ctx.accounts.user_b;

        user_a.data = a;
        user_b.data = b;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateInsecure<'info> {
    user_a: Account<'info, User>,
    user_b: Account<'info, User>,
}

#[derive(Accounts)]
pub struct UpdateSecure<'info> {
    #[account(constraint = user_a.key() != user_b.key())]
    user_a: Account<'info, User>,
    user_b: Account<'info, User>,
}

#[account]
pub struct User {
    data: u64,
}
