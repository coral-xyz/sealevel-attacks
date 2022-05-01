use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

/// Utilizes anchor's Signer to authenticate
/// the transaction signer
///
/// [Signer](https://docs.rs/anchor-lang/0.24.2/anchor_lang/accounts/signer/struct.Signer.html)
#[program]
pub mod signer_authorization_recommended {
    use super::*;

    pub fn log_message(ctx: Context<LogMessage>) -> ProgramResult {
        // Signer verifies authenticity no need of explicit checks here
        msg!("GM {}", ctx.accounts.authority.key().to_string());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct LogMessage<'info> {
    /// signer performs Signer.info.is_signer == true check
    authority: Signer<'info>,
}
