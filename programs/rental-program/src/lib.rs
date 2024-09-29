use anchor_lang::prelude::*;
declare_id!("CFHjCR3Xe4Ek9rsGeDscS4TtMJwhpRuR4nDkP3B6RmtM");

#[program]
pub mod rental_program {
    use anchor_lang::solana_program::{program::invoke, system_instruction};
    use super::*;
    pub fn create_bounty(ctx:Context<CreateBounty>,id:String, amount:u64)->Result<()>{
        //get ref of bounty account
        let bounty= &mut ctx.accounts.bounty;

        //initialize bounty account
        bounty.client = *ctx.accounts.client.key;
        bounty.amount = amount;
        bounty.id = id;
        bounty.status = BountyStatus::Open;
        bounty.worker = None;

        //transfer amount to bounty account
        let from_pubkey = ctx.accounts.client.to_account_info();
        let to_pubkey = ctx.accounts.bounty.to_account_info();
        let program_info = ctx.accounts.system_program.to_account_info();

        let ix = &system_instruction::transfer(&from_pubkey.key(), &to_pubkey.key(), amount);
        invoke(ix, &[from_pubkey, to_pubkey, program_info])?;
        Ok(())
    }
}

//Create bounty instruction
#[derive(Accounts)]
#[instruction(id: String)]
pub struct CreateBounty<'info>{
    #[account(mut)]
    pub client: Signer<'info>,
    #[account(init, payer=client, space=Bounty::INIT_SPACE, seeds=[b"bounty",client.key().as_ref(),id.as_bytes()],bump)]
    pub bounty: Account<'info,Bounty>,
    pub system_program:Program <'info,System>,
}


#[account]
pub struct Bounty{
    pub client: Pubkey,
    pub amount: u64,
    pub id: String,
    pub status: BountyStatus,
    pub worker: Option<Pubkey>, 
}

impl Space for Bounty{
    const INIT_SPACE: usize =8+32+4+8+1+32; // determinant + client size + amount size + string size + status size + worker size
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum BountyStatus{
    Open,
    InProgress,
    Claimed,
}