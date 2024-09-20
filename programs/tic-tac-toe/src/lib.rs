use anchor_lang::prelude::*;

declare_id!("Evt3uMLxb6VtK8UtLnCnKSp5yZ8jBDC8Z4KsVHXL2xiT");

#[program]
pub mod tic_tac_toe {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
