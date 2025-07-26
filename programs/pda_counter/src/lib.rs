use anchor_lang::prelude::*;

declare_id!("3mQdn2TX5x4sX5Niisy6Q5B8yHDqtGxvYUXj58DCcuJ7");

#[program]
pub mod pda_counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
