use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};
use std::ops::DerefMut;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod reinitialization_4 {
    use super::*;

    pub fn initialize_insecure(ctx: Context<InitializeInsecure>) -> ProgramResult {
        let mut user = UserInsecure::try_from_slice(&ctx.accounts.user.data.borrow()).unwrap();

        user.authority = ctx.accounts.authority.key();

        let mut storage = ctx.accounts.user.try_borrow_mut_data()?;
        user.serialize(storage.deref_mut()).unwrap();
        Ok(())
    }

    pub fn initialize_secure(ctx: Context<InitializeSecure>) -> ProgramResult {
        let mut user = UserSecure::try_from_slice(&ctx.accounts.user.data.borrow()).unwrap();
        if !user.discriminator {
            return Err(ProgramError::InvalidAccountData);
        }

        user.authority = ctx.accounts.authority.key();

        let mut storage = ctx.accounts.user.try_borrow_mut_data()?;
        user.serialize(storage.deref_mut()).unwrap();

        msg!("GM");
        Ok(())
    }

    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        msg!("GM");
        Ok(())
    }

    pub fn initialize_preferred(ctx: Context<InitializePreferred>) -> ProgramResult {
        msg!("GM");
        Ok(())
    }

    pub fn init(ctx: Context<Init>) -> ProgramResult {
        msg!("GM");
        Ok(())
    }
}

/*
- reinitialize
- create and dont initialize
- passing previously initialzed accounts from other programs
  (e.g. token program => need to check delegate and authority)
*/

#[derive(Accounts)]
pub struct InitializeInsecure<'info> {
    user: AccountInfo<'info>,
    authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct InitializeSecure<'info> {
    user: AccountInfo<'info>,
    authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(zero)]
    user: Account<'info, User>,
    authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct InitializePreferred<'info> {
    #[account(zero, signer)]
    user: Account<'info, User>,
    authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(init, payer = authority, space = 8+32)]
    user: Account<'info, User>,
    authority: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct UserInsecure {
    authority: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct UserSecure {
    discriminator: bool,
    authority: Pubkey,
}

#[account]
pub struct User {
    authority: Pubkey,
}
