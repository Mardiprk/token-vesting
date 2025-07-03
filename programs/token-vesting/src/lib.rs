use anchor_lang::prelude::*;

mod state;
mod error;

declare_id!("2mxPnkEdXXVVevKofXooFKGYx8eVqJ6wEWNZz6AY8GWR");

#[program]
pub mod token_vesting {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        start_time: i64,
        release_time: i64,
        amount: u64,
    ) -> Result<()> {
        let vesting_account = &mut ctx.accounts.vesting;

        vesting_account.beneficiary = ctx.accounts.beneficiary.key();
        vesting_account.start_time = start_time;
        vesting_account.release_time = release_time;
        vesting_account.amount = amount;
        vesting_account.claimed = 0;

        Ok(())
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        let vesting = &mut ctx.accounts.vesting;
        let clock = Clock::get()?;

        if clock.unix_timestamp < vesting.release_time{
            return Err(error!(VestingError::TooEarly));
        }

        if vesting.claimed >= vesting.claimed{
            return Err(error!(VestingError::AlreadyClaimed));
        }

        vesting.claimed = vesting.amount;
        msg!("Tokens Claimed successfully");

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = initializer,
        space = 8 + std::mem::size_of::<VestingAccount>(),
    )]
    pub vesting: Account<'info, VestingAccount>,

    #[account(mut)]
    pub initializer: Signer<'info>,

    pub beneficiary: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut, has_one = beneficiary)]
    pub vesting: Account<'info, VestingAccount>,

    #[account(mut)]
    pub beneficiary: Signer<'info>,
}

#[error_code]
pub enum VestingError {
    #[msg("It's not time to release tokens yet.")]
    TooEarly,
    #[msg("All tokens have already been claimed.")]
    AlreadyClaimed,
}