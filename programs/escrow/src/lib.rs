#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

mod instructions;
use instructions::*;
pub mod state;

declare_id!("Gu3b6ymsi8Q2aJCTR7J5PR7sJzQaAupkzzfRzd2RkxKZ");

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result<()> {
        ctx.accounts.deposit(deposit)?;
        ctx.accounts.init_escrow(seed, receive, &ctx.bumps)
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund_and_close_vault()
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.deposit()?;
        ctx.accounts.withdraw_and_close_vault()
    }

}

#[derive(Accounts)]
pub struct Initialize {}
