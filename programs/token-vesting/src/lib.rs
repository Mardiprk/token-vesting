use anchor_lang::prelude::*;

mod state;
mod error;

declare_id!("2mxPnkEdXXVVevKofXooFKGYx8eVqJ6wEWNZz6AY8GWR");

#[program]
pub mod token_vesting {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
