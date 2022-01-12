use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod bump_seed_canonicalization_6 {
    use super::*;

    pub fn set_value_insecure(
        ctx: Context<BumpSeedInsecure>,
        key: u64,
        new_value: u64,
        bump: u8,
    ) -> ProgramResult {
        let address = Pubkey::create_program_address();
        Ok(())
    }

    pub fn set_value_secure(ctx: Context<BumpSeedSecure>, new_value: u64) -> ProgramResult {
        //
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BumpSeedInsecure<'info> {
    data: Account<'info, Data>,
}

#[derive(Accounts)]
pub struct BumpSeedSecure<'info> {
    data: Account<'info, Data>,
}

#[account]
pub struct Data {
    value: u64,
}
