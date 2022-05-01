use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
/// Objective: Authenticating Signer of transaction
///
/// Topics: [AccountInfo](https://docs.rs/anchor-lang/0.24.2/anchor_lang/prelude/struct.AccountInfo.html), [Signer](https://docs.rs/anchor-lang/0.24.2/anchor_lang/accounts/signer/struct.Signer.html)
///
/// The following program example accepts
/// arbitrary solana account hence not safe.
/// and log_message method is also not performing any
/// safety check
#[program]
pub mod signer_authorization_insecure {
    use super::*;

    pub fn log_message(ctx: Context<LogMessage>) -> ProgramResult {
        // no account checks performed here
        msg!("GM {}", ctx.accounts.authority.key().to_string());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LogMessage<'info> {
    /// arbitrary solana account\
    /// read more about [AccountInfo](https://docs.rs/anchor-lang/0.24.2/anchor_lang/prelude/struct.AccountInfo.html)
    authority: AccountInfo<'info>,
}
