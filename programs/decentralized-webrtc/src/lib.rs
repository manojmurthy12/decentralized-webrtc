use anchor_lang::prelude::*;

declare_id!("9VZTmG79FSDSfuJFi5SgVd4FdyJBX5gB6SDvN6QF95Ki");

#[program]
pub mod decentralized_webrtc {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
