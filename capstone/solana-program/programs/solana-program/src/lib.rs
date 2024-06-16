use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_lang::solana_program::entrypoint::ProgramResult;
declare_id!("5VpsAtBXYnczC2szekZsYgzJcN3rAMj13iA8YrWQyzCj");
const CAMPAIGN_PREFIX: &'static str = "wishbourne-campaign";
const DONATION_PREFIX: &'static str = "wishbourne-donation";

#[program]
pub mod solana_program {
    use super::*;

    // Campaign Instructions
    pub fn create_campaign(ctx: Context<CreateCampaign>, title: String, celeb: Pubkey) -> ProgramResult {
        let campaign = &mut ctx.accounts.campaign;

        campaign.title = title;
        campaign.celeb = celeb;
        campaign.goal = 0;
        campaign.status = Status::Pending;
        msg!("A new campaign has been created with the following pda: {}", campaign.to_account_info().key.to_string());
        Ok(())
    }

    pub fn transfer_ownership(ctx: Context<TransferOwnership>, title: String, celeb: Pubkey) -> Result<()> {
        let campaign = &mut ctx.accounts.campaign;

        let campaign_info = campaign.to_account_info();
        campaign_info.assign(&celeb);
        let mut account_data = campaign_info.data.borrow_mut();
        let data_len = account_data.len();
        anchor_lang::solana_program::program_memory::sol_memset(*account_data, 0, data_len);
        msg!("new campaign title: {}, new owner: {}", campaign.title, celeb.to_string());
        Ok(())
    }

    pub fn update_campaign(ctx: Context<UpdateCampaign>, goal: u64) -> ProgramResult {
        let campaign = &mut ctx.accounts.campaign;
        campaign.goal = goal;
        
        match campaign.status {
            Status::Pending => {
                campaign.status = Status::Approved;
                ()
            },
            _ => {}
        }

        Ok(())
    }

    pub fn fulfill_campaign(ctx: Context<UpdateCampaign>) -> Result<()> {
        let campaign = &mut ctx.accounts.campaign;

        match campaign.status {
            Status::Approved => {
                campaign.status = Status::Fulfilled;
                ()
            },
            _ => {}
        }

        Ok(())       
    }

    pub fn terminate_campaign(ctx: Context<UpdateCampaign>) -> Result<()> {
        let campaign = &mut ctx.accounts.campaign;

        match campaign.status {
            Status::Pending => {
                campaign.status = Status::Terminated;
                ()
            },
            _ => {}
        }

        Ok(())          
    }

    pub fn collect_earnings(ctx: Context<UpdateCampaign>) -> Result<()> {
        let campaign = &mut ctx.accounts.campaign;
        let celeb = &mut ctx.accounts.celeb;

        let entire_balance = campaign.get_lamports();

        let pda_account_info = campaign.to_account_info();
        **pda_account_info.lamports.borrow_mut() -= entire_balance;
        let celeb_account_info = celeb.to_account_info();
        **celeb_account_info.lamports.borrow_mut() += entire_balance;

        let celeb_balance_after = celeb.get_lamports();
        msg!("Celeb balance: {}", celeb_balance_after);

        require_eq!(celeb_balance_after, entire_balance);
        Ok(())
    }

    // Donator PDA Instructions
    pub fn create_donation_space(ctx: Context<CreateDonationSpace>, fund_lamports: u64) -> Result<()> {
        let space = &mut ctx.accounts.donation_space;
        let signer = &mut ctx.accounts.donator;
        let system_program = &ctx.accounts.system_program;

        let pda_balance_before = space.get_lamports();

        transfer(
            CpiContext::new(
                system_program.to_account_info(),
                Transfer {
                    from: signer.to_account_info(),
                    to: space.to_account_info(),
                },
            ),
            fund_lamports,
        )?;

        let pda_balance_after = space.get_lamports();

        require_eq!(pda_balance_after, pda_balance_before + fund_lamports);
        msg!("Balance of pda after creation: {}", pda_balance_after);

        Ok(())
    }

    pub fn withdraw_donation(ctx: Context<WithdrawDonation>) -> Result<()> {
        let space = &mut ctx.accounts.donation_space;
        let donator = &mut ctx.accounts.donator;

        let entire_balance = space.get_lamports();


        /*let campaign = &ctx.accounts.campaign;
        let system_program = &ctx.accounts.system_program;



        let bump = &[ctx.bumps.donation_space];
        let seeds: &[&[u8]] = &[DONATION_PREFIX.as_bytes().as_ref(), donator.key.as_ref(), campaign.key.as_ref(), bump];
        let signer_seeds = &[&seeds[..]];

        transfer(
            CpiContext::new(
                system_program.to_account_info(),
                Transfer {
                    from: space.to_account_info(),
                    to: donator.to_account_info(),
                },
            ).with_signer(signer_seeds),
            entire_balance,
        )?;*/

        // Since this pda belongs to our program (rather than system program), it can directly modify lamports
        let pda_account_info = space.to_account_info();
        **pda_account_info.lamports.borrow_mut() -= entire_balance;
        let receiver_account_info = donator.to_account_info();
        **receiver_account_info.lamports.borrow_mut() += entire_balance;

        let pda_balance_after = space.get_lamports();
        msg!("Balance of pda after withrawal: {}", pda_balance_after);

        require_eq!(pda_balance_after, 0);

        Ok(())        
    }

    pub fn commit_donation(ctx: Context<CommitDonation>) -> Result<()> {
        let space = &mut ctx.accounts.donation_space;
        let campaign = &ctx.accounts.campaign;

        let entire_balance = space.get_lamports();

        let pda_account_info = space.to_account_info();
        **pda_account_info.lamports.borrow_mut() -= entire_balance;
        let receiver_account_info = campaign;
        **receiver_account_info.lamports.borrow_mut() += entire_balance;

        let pda_balance_after = space.get_lamports();
        msg!("Balance of pda after withrawal: {}", pda_balance_after);

        require_eq!(pda_balance_after, 0);

        Ok(())             
    }
    
}

#[derive(Accounts)]
#[instruction(title: String, celeb: Pubkey)]
pub struct CreateCampaign<'info> {
    #[account(
        init,
        seeds = [CAMPAIGN_PREFIX.as_bytes().as_ref(), title.as_bytes().as_ref(), celeb.as_ref()],
        bump,
        payer = initializer,
        space = 8 + 8 + 4 + title.len() + 32 + 8 + 1 // initial-space: 8, id: 8, celeb: 32, goal: 8, status: 1 
    )]
    pub campaign: Account<'info, Campaign>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateCampaign<'info>{
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
    #[account(mut)]
    pub celeb: Signer<'info>
}

#[derive(Accounts)]
#[instruction(title: String, celeb: Pubkey)]
pub struct TransferOwnership<'info> {
    #[account(
        mut,
        seeds = [CAMPAIGN_PREFIX.as_bytes().as_ref(), title.as_bytes().as_ref(), celeb.as_ref()],
        bump,
        realloc = 8 + 8 + 4 + title.len() + 32 + 8 + 1, // initial-space: 8, id: 8, celeb: 32, goal: 8, status: 1
        realloc::payer = initializer,
        realloc::zero = true
    )]
    pub campaign: Account<'info, Campaign>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(fund_lamports: u64)]
pub struct CreateDonationSpace<'info> {
    #[account(
        init,
        seeds = [DONATION_PREFIX.as_bytes().as_ref(), donator.key.as_ref(), campaign.key.as_ref()],
        bump,
        payer = donator,
        space = 8 + 32 + 32
    )]
    pub donation_space: Account<'info, DonationSpace>,
    #[account(mut)]
    pub donator: Signer<'info>,
    /// CHECK: Campaign info is only provided as seed
    pub campaign: AccountInfo<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct WithdrawDonation<'info> {
    #[account(
        mut,
        seeds = [DONATION_PREFIX.as_bytes().as_ref(), donator.key.as_ref(), campaign.key.as_ref()],
        bump
    )]
    pub donation_space: Account<'info, DonationSpace>,
    #[account(mut)]
    pub donator: Signer<'info>,
    /// CHECK: Campaign info is only provided as seed
    pub campaign: AccountInfo<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct CommitDonation<'info> {
    #[account(
        mut,
        seeds = [DONATION_PREFIX.as_bytes().as_ref(), donator.key.as_ref(), campaign.key.as_ref()],
        bump
    )]
    pub donation_space: Account<'info, DonationSpace>,
    #[account(mut)]
    pub donator: Signer<'info>,
    /// CHECK: Campaign info is only provided as seed
    pub campaign: AccountInfo<'info>,
    pub system_program: Program<'info, System>
}

// Campaign status is dependent on the celebrity decision. Pending until noticed and updated; Approved when celeb updates and sets conditions for the campaign; Fulfilled when celeb shares the desired content; Terminated if celeb rejects the campaign
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum Status {
    Pending,
    Approved,
    Fulfilled,
    Terminated
}

impl Default for Status {
    fn default() -> Self {
        Status::Pending
    }
}

#[account]
pub struct Campaign {
    pub id: u64,
    pub title: String,
    pub celeb: Pubkey,
    pub goal: u64,
    pub status: Status
}

#[account]
pub struct DonationSpace {
    pub donator: Pubkey,
    pub campaign: Pubkey
}
