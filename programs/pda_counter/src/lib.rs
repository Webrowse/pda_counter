use anchor_lang::prelude::*;

//program ID, prewritten with anchor init, but it can also be retrieve from target/deploy/*.json
declare_id!("3mQdn2TX5x4sX5Niisy6Q5B8yHDqtGxvYUXj58DCcuJ7");

#[program]
pub mod pda_counter {
    use super::*;

    //Initializing a new counter PDA, setting the count = 0
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter; //instance created
        counter.count = 0; //initialied an account using that instance
        counter.user = *ctx.accounts.user.key;

        //emit! emits the event (that counter has been inited), written far below this program
        emit!(CounterInit {
            user: counter.user,
            count: counter.count,
        });

        Ok(())
    }

    //Increments the counter, after verify if user is the owner.
    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;

        //ownership check
        if counter.user != *ctx.accounts.user.key {
            return err!(CounterError::InvalidUser);
        }
        counter.count += 1;

        //Emits CounterIncrease event
        emit!(CounterIncrease {
            user: counter.user,
            count: counter.count,
        });

        Ok(())
    }

    //Reset the counter to 0 and closes the account, returning the SOL.
    pub fn reset(ctx: Context<Reset>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;

        //ownership checking
        if counter.user != *ctx.accounts.user.key {
            return err!(CounterError::InvalidUser);
        }
        counter.count = 0;

        //emit reset event
        emit!(CounterReset { user: counter.user });

        Ok(())
    }
}

// ──・❥ ────・❥ ────・❥ ────・❥ ────Account Contexts────・❥ ────・❥ ────・❥ ────・❥ ────・❥ ──

//Context for initialising the PDA counter
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 8,
        seeds = [b"counte", user.key().as_ref()], bump //the [boxed] item wasn't mandatory, anchor provide bump
    )]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

//Context for incrementing the counter
#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(
        mut,
        seeds = [b"counte", user.key().as_ref()], bump
    )]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
}

//Context for reset and close the account
#[derive(Accounts)]
pub struct Reset<'info> {
    #[account(mut,
        seeds = [b"counte", user.key().as_ref()], bump,
        close = user,
    )]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
}

// ──・❥ ────・❥ ────・❥ ────・❥ ────Account Data────・❥ ────・❥ ────・❥ ────・❥ ────・❥ ──

//the final data structure stored in the PDA
#[account]
pub struct Counter {
    pub user: Pubkey,
    pub count: u64,
}

// ──・❥ ────・❥ ────・❥ ────・❥ ────Events────・❥ ────・❥ ────・❥ ────・❥ ────・❥ ──

//Init event
#[event]
pub struct CounterInit {
    pub user: Pubkey,
    pub count: u64,
}

//Increment event
#[event]
pub struct CounterIncrease {
    pub user: Pubkey,
    pub count: u64,
}

//Reset event
#[event]
pub struct CounterReset {
    pub user: Pubkey,
}

// ──・❥ ────・❥ ────・❥ ────・❥ ────Custom Errors────・❥ ────・❥ ────・❥ ────・❥ ────・❥ ──

//If user is not the owner, this error will be used.
#[error_code]
pub enum CounterError {
    #[msg("Unauthorised User")]
    InvalidUser,
}
