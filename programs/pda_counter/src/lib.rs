use anchor_lang::prelude::*;

declare_id!("3mQdn2TX5x4sX5Niisy6Q5B8yHDqtGxvYUXj58DCcuJ7");

#[program]
pub mod pda_counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter; //instance created
        counter.count = 0; //initialied an account using that instance
        counter.user = *ctx.accounts.user.key;

        emit!(CounterInit{
            user: counter.user,
            count: counter.count,
        });

        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        if counter.user != *ctx.accounts.user.key {
            return err!(CounterError::InvalidUser);
        }
        counter.count += 1;

        emit!(CounterIncrease {
            user: counter.user,
            count: counter.count,
        });

        Ok(())
    }

    pub fn reset (ctx: Context<Reset>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        if counter.user != *ctx.accounts.user.key {
            return err!(CounterError::InvalidUser);
        }
        counter.count = 0;

        emit!(CounterReset{
            user: counter.user,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 8,
        seeds = [b"counte", user.key().as_ref()], bump
    )]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(
        mut,
        seeds = [b"counte", user.key().as_ref()], bump
    )]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>
}

#[derive(Accounts)]
pub struct Reset<'info> {
    #[account(mut,
        seeds = [b"counte", user.key().as_ref()], bump,
        close = user,
    )]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
}

#[account]
pub struct Counter {
    pub user: Pubkey,
    pub count: u64,
}

#[event]
pub struct CounterInit {
    pub user: Pubkey,
    pub count: u64,

}
#[event]
pub struct CounterIncrease {
    pub user: Pubkey,
    pub count: u64,
}

#[event]
pub struct CounterReset {
    pub user: Pubkey,
}

#[error_code]
pub enum CounterError {
    #[msg("Unauthorised User")]
    InvalidUser,
}