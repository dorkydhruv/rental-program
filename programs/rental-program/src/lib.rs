use anchor_lang::prelude::*;

declare_id!("CFHjCR3Xe4Ek9rsGeDscS4TtMJwhpRuR4nDkP3B6RmtM");

#[program]
pub mod rental_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
