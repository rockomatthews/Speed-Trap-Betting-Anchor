use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solana_betting {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let bet_account = &mut ctx.accounts.bet_account;
        bet_account.bets = 0;
        Ok(())
    }

    pub fn place_bet(ctx: Context<PlaceBet>, amount: u64) -> Result<()> {
        let bet_account = &mut ctx.accounts.bet_account;
        bet_account.bets += 1;
        // Here you would handle the actual betting logic
        msg!("Bet placed: {}", amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub bet_account: Account<'info, BetAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub bet_account: Account<'info, BetAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
pub struct BetAccount {
    pub bets: u64,
}