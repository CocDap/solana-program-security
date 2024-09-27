use anchor_lang::prelude::*;

declare_id!("voCqptKux5ai71YySiW3sE9Ze9mJJi5oBo55V68YddU");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 100;
        msg!("Counter account created! Current count: {}", counter.count);
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>, additional_count: u8) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        msg!("Previous counter: {}", counter.count);
        
        // increase with additional count -> maybe happen overflow issue
        // let counter.count += additional_count;

        // Fix overflow issue -> return default value if none 
        let new_count = counter.count.checked_add(additional_count).unwrap_or_default();
        counter.count = new_count;

        msg!("Counter incremented! Current count: {}", counter.count);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,         
        payer = user,
        space = 8 + 8 
    )]
    pub counter: Account<'info, Counter>, 
    pub system_program: Program<'info, System>, 
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)] 
    pub counter: Account<'info, Counter>, 
}


#[account]
pub struct Counter {
    pub count: u8, // define count value type as u8
}
