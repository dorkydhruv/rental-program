use anchor_lang::prelude::*;
declare_id!("CFHjCR3Xe4Ek9rsGeDscS4TtMJwhpRuR4nDkP3B6RmtM");

#[program]
pub mod rental_program {
    use anchor_lang::solana_program::{program::invoke, system_instruction};

    use super::*;
    pub fn create_bounty(ctx:Context<CreateBounty>,description:String, amount:u64)->Result<()>{
        let bounty = &mut ctx.accounts.bounty_account;
        bounty.client = *ctx.accounts.client.key;
        bounty.amount = amount;
        bounty.description = description;
        bounty.status = BountyStatus::Open;
        bounty.worker = None;
        invoke(&system_instruction::transfer(ctx.accounts.client.to_account_info().key, &bounty.to_account_info().key, amount), &[
            ctx.accounts.client.to_account_info(),
            ctx.accounts.bounty_account.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ])?;
        Ok(())
    }
}


#[derive(Accounts)]
#[instruction(description: String)]
pub struct CreateBounty<'info>{
    #[account(mut)]
    pub client: Signer<'info>,
    #[account(
        init,
        payer=client,
        seeds=["bounty".as_bytes(),client.key().as_ref()],
        bump,
        space = Bounty::INIT_SPACE + description.len(),
    )]
    pub bounty_account: Account<'info,Bounty>,
    pub system_program: Program<'info, System>,
}



#[account]
pub struct Bounty{
    pub client: Pubkey,
    pub amount: u64,
    pub description: String,
    pub status: BountyStatus,
    pub worker: Option<Pubkey>, 
}

impl Space for Bounty{
    const INIT_SPACE: usize =8+32+4+8+32+1;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum BountyStatus{
    Open,
    InProgress,
    Claimed,
}