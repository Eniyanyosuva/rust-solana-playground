use anchor_lang::prelude::*;

declare_id!("So1Sp1iT111111111111111111111111111111111");

#[program]
pub mod solsplit {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        percent_a: u8,
        percent_b: u8,
    ) -> Result<()> {
        require!(percent_a + percent_b == 100, ErrorCode::InvalidPercentages);

        let state = &mut ctx.accounts.state;
        state.sender = ctx.accounts.sender.key();
        state.recipient_a = ctx.accounts.recipient_a.key();
        state.recipient_b = ctx.accounts.recipient_b.key();
        state.percent_a = percent_a;
        state.percent_b = percent_b;
        state.executed = false;
        Ok(())
    }

    pub fn execute(ctx: Context<Execute>, amount: u64) -> Result<()> {
        let state = &mut ctx.accounts.state;
        require!(!state.executed, ErrorCode::AlreadyExecuted);

        let lamports_a = amount * state.percent_a as u64 / 100;
        let lamports_b = amount - lamports_a;

        **ctx.accounts.sender.try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.recipient_a.try_borrow_mut_lamports()? += lamports_a;
        **ctx.accounts.recipient_b.try_borrow_mut_lamports()? += lamports_b;

        state.executed = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    /// CHECK:
    pub recipient_a: UncheckedAccount<'info>,

    /// CHECK:
    pub recipient_b: UncheckedAccount<'info>,

    #[account(
        init,
        payer = sender,
        space = 8 + SplitState::LEN,
        seeds = [b"split", sender.key().as_ref()],
        bump
    )]
    pub state: Account<'info, SplitState>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Execute<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    #[account(mut)]
    pub recipient_a: UncheckedAccount<'info>,

    #[account(mut)]
    pub recipient_b: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"split", sender.key().as_ref()],
        bump
    )]
    pub state: Account<'info, SplitState>,
}

#[account]
pub struct SplitState {
    pub sender: Pubkey,
    pub recipient_a: Pubkey,
    pub recipient_b: Pubkey,
    pub percent_a: u8,
    pub percent_b: u8,
    pub executed: bool,
}

impl SplitState {
    pub const LEN: usize = 32 + 32 + 32 + 1 + 1 + 1;
}

#[error_code]
pub enum ErrorCode {
    #[msg("Percentages must total 100")]
    InvalidPercentages,
    #[msg("Split already executed")]
    AlreadyExecuted,
}
