use anchor_lang::{accounts::program, prelude::*};
use anchor_spl::{associated_token::{self, AssociatedToken}, token::{Mint, Token, TokenAccount, Transfer, transfer}};

use crate::state::Escrow;

#[derive(Accounts)]
pub struct Take<'info>{
    #[account(mut)]
    pub taker: Signer<'info>,
    pub mint_a: Account<'info, Mint>,
    pub mint_b: Account<'info, Mint>,
    #[account(mut,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
        close = maker,
        has_one = mint_a,
        has_one = mint_b
    )]
    pub escrow: Account<'info, Escrow>,
    pub maker: SystemAccount<'info>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
    )]
    pub ata_taker_a: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
    )]
    pub ata_taker_b: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = taker       
    )]
    pub ata_maker_b: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::authority = escrow,
        associated_token::mint = mint_a

    )]
    pub vault: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>
}

impl<'info>Take<'info>{
    pub fn deposit(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_account = Transfer{
            from: self.ata_taker_b.to_account_info(),
            to: self.ata_maker_b.to_account_info(),
            authority: self.taker.to_account_info()
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_account);
        transfer(cpi_ctx, self.escrow.receive)?;
        Ok(())
    }
}