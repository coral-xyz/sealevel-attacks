use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod type_cosplay_3 {
    use super::*;

    pub fn update_user_insecure(ctx: Context<UpdateUserInsecure>) -> ProgramResult {
        let user = UserInsecure::try_from_slice(&ctx.accounts.user.data.borrow()).unwrap();
        if ctx.accounts.user.owner != ctx.program_id {
            return Err(ProgramError::IllegalOwner);
        }
        if user.authority != ctx.accounts.authority.key() {
            return Err(ProgramError::InvalidAccountData);
        }
        msg!("GM");
        Ok(())
    }

    pub fn update_user_secure(ctx: Context<UpdateUserSecure>) -> ProgramResult {
        let user = UserSecure::try_from_slice(&ctx.accounts.user.data.borrow()).unwrap();
        if ctx.accounts.user.owner != ctx.program_id {
            return Err(ProgramError::IllegalOwner);
        }
        if user.authority != ctx.accounts.authority.key() {
            return Err(ProgramError::InvalidAccountData);
        }
        if user.discriminant != AccountDiscriminant::User {
            return Err(ProgramError::InvalidAccountData);
        }
        msg!("GM");
        Ok(())
    }

    pub fn update_user(ctx: Context<UpdateUser>) -> ProgramResult {
        msg!("GM");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateUserInsecure<'info> {
    user: AccountInfo<'info>,
    authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateUserSecure<'info> {
    user: AccountInfo<'info>,
    authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateUser<'info> {
    #[account(has_one = authority)]
    user: Account<'info, User>,
    authority: Signer<'info>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct UserInsecure {
    authority: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct MetadataInsecure {
    account: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct UserSecure {
    discriminant: AccountDiscriminant,
    authority: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct MetadataSecure {
    discriminant: AccountDiscriminant,
    account: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq)]
pub enum AccountDiscriminant {
    User,
    Metadata,
}

#[account]
pub struct User {
    authority: Pubkey,
}

#[account]
pub struct Metadata {
    account: Pubkey,
}
