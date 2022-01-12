use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod bump_seed_canonicalization_recommended {
    use super::*;

    pub fn set_value(ctx: Context<BumpSeed>, key: u64, new_value: u64) -> ProgramResult {
        ctx.accounts.data.value = new_value;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(key: u64)]
pub struct BumpSeed<'info> {
    #[account(seeds = [key.to_le_bytes().as_ref()], bump = data.bump)]
    data: Account<'info, Data>,
}

#[account]
pub struct Data {
    value: u64,
    bump: u8,
}
