use anchor_lang::prelude::*;
use std::mem::size_of;


declare_id!("9cUu6VnHH1D2jbW7vRYXtw43XGqueB16JGBBNhcxGet8");

#[program]
pub mod escrow {
    use super::*;

    pub fn create_escrow(ctx: Context<CreateEscrow>, amount: u64) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;

        escrow.from = ctx.accounts.from.key();
        escrow.to = ctx.accounts.to.key();
        escrow.amount = amount;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateEscrow<'info> {
    // PDA
    #[account(
        init,
        seeds = [b"escrow".as_ref(), from.key().as_ref(), to.key().as_ref()],
        bump, 
        payer = from,
        // If adding strings, add space 
        space = size_of::<EscrowAccount>() + 16
    )]
    pub escrow: Account<'info, EscrowAccount>,

    #[account(mut)]
    pub from: Signer<'info>,
    /// CHECK: This is safe
    #[account(mut)]
    pub to: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct EscrowAccount {
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
}

