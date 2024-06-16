use anchor_lang::prelude::*;

declare_id!("3dVu4pTWwRJ8WMzG7aW196GiNjsBPFYDoy5uGw7PZw2J");

#[program]
pub mod vote {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, _url: String) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)?;
        Ok(())
    }

    pub fn upvote(ctx: Context<Vote>, _url: String) -> Result<()>{
        ctx.accounts.upvote()?;
        Ok(())
    }

    pub fn downvote(ctx: Context<Vote>, _url: String) -> Result<()> {
        ctx.accounts.downvote()?;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(_url: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        seeds = [_url.as_bytes().as_ref()],
        bump,
        space = VoteState::INIT_SPACE
    )]
    pub vote_state: Account<'info, VoteState>,
    pub system_program: Program<'info, System>
}

impl <'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        self.vote_state.score = 0;
        self.vote_state.bump = bumps.vote_state;
        Ok(())
    }
}

#[account]
pub struct VoteState {
    pub score: i64,
    pub bump: u8
}

impl Space for VoteState {
    const INIT_SPACE: usize = 8 + 8 + 1;
}

#[derive(Accounts)]
#[instruction(_url: String)]
pub struct Vote<'info> {
    #[account(
        mut,
        seeds = [_url.as_bytes().as_ref()],
        bump = vote_state.bump
    )]
    pub vote_state: Account<'info, VoteState>
}

impl<'info> Vote<'info> {
    pub fn upvote(&mut self) -> Result<()>{
        self.vote_state.score += 1;
        Ok(())
    }

    pub fn downvote(&mut self) -> Result<()>{
        self.vote_state.score -= 1;
        Ok(())
    }
}
