use anchor_lang::prelude::*;

declare_id!("Ho96kn5EhyMq9FGAjrvVqeo3wXwuCKuMzySZiQCBJGzw");

#[program]
pub mod rental_program {
    use anchor_lang::solana_program::{ program::invoke, system_instruction };
    use super::*;

    pub fn create_bounty(ctx: Context<CreateBounty>, id: String, amount: u64) -> Result<()> {
        let bounty = &mut ctx.accounts.bounty;

        bounty.client = *ctx.accounts.client.key;
        bounty.amount = amount;
        bounty.id = id;
        bounty.status = BountyStatus::Open;
        bounty.worker = None;

        let from_pubkey = ctx.accounts.client.to_account_info();
        let to_pubkey = ctx.accounts.bounty.to_account_info();
        let program_info = ctx.accounts.system_program.to_account_info();

        let ix = &system_instruction::transfer(&from_pubkey.key(), &to_pubkey.key(), amount);
        invoke(ix, &[from_pubkey, to_pubkey, program_info])?;
        Ok(())
    }

    pub fn add_worker(ctx: Context<AddWorker>, worker: Pubkey) -> Result<()> {
        msg!("Adding worker: {:?}", worker);
        let bounty = &mut ctx.accounts.bounty;
        bounty.worker = Some(worker);
        msg!("Updated worker: {:?}", bounty.worker);
        bounty.status = BountyStatus::InProgress;
        msg!("Updated status: {:?}", bounty.status);
        Ok(())
    }

    pub fn close_bounty(ctx: Context<CloseBounty>) -> Result<()> {
        let bounty = &mut ctx.accounts.bounty;
        bounty.status = BountyStatus::Complete;
        Ok(())
    }

    //claim the bounty once the bountySattus is marked as Complete
    pub fn claim_bounty(_ctx: Context<ClaimBounty>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(id: String)]
pub struct CreateBounty<'info> {
    #[account(mut)]
    pub client: Signer<'info>,
    #[account(
        init,
        payer = client,
        space = Bounty::INIT_SPACE + id.len(),
        seeds = [b"bounty", client.key().as_ref(), id.as_bytes()],
        bump
    )]
    pub bounty: Account<'info, Bounty>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddWorker<'info> {
    #[account(mut)]
    pub client: Signer<'info>,
    #[account(mut, seeds=[b"bounty",client.key().as_ref(),bounty.id.as_bytes()], bump)]
    pub bounty: Account<'info, Bounty>,
}

#[derive(Accounts)]
pub struct CloseBounty<'info> {
    #[account(mut)]
    pub client: Signer<'info>,
    #[account(mut, seeds=[b"bounty",client.key().as_ref(),bounty.id.as_bytes()], bump)]
    pub bounty: Account<'info, Bounty>,
}

#[derive(Accounts)]
pub struct ClaimBounty<'info> {
    #[account(mut)]
    pub worker: Signer<'info>,
    #[account(
        mut,
        seeds=[b"bounty",bounty.client.key().as_ref(),bounty.id.as_bytes()],
        bump,
        close=worker
    )]
    pub bounty: Account<'info, Bounty>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Bounty {
    pub client: Pubkey,
    pub amount: u64,
    pub id: String,
    pub status: BountyStatus,
    pub worker: Option<Pubkey>,
}

impl Space for Bounty {
    const INIT_SPACE: usize = 8 + 32 + 8 + 4 + 1 + 1 + 32;
    //    + 1 (BountyStatus) + 1 (Option bool) + 32 (Pubkey in Option)
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum BountyStatus {
    Open,
    InProgress,
    Complete,
}
