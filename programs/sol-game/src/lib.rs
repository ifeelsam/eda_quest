use anchor_lang::prelude::*;

declare_id!("91juefdypagioebacMPA6w1jVqUv7fPRpqqm7D4pM2RW");

#[program]
pub mod sol_game {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
