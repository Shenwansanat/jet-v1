use anchor_lang::prelude::*;
use anchor_lang::Key;
use anchor_spl::token::TokenAccount;

use crate::state::*;

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitializeLoanAccount<'info> {
    /// The relevant market this loan is for
    #[account(has_one = market_authority)]
    pub market: Loader<'info, Market>,

    /// The market's authority account
    pub market_authority: AccountInfo<'info>,

    /// The obligation the loan account is used for
    #[account(mut,
              has_one = market,
              has_one = owner)]
    pub obligation: Loader<'info, Obligation>,

    /// The reserve that the loan comes from
    #[account(has_one = market,
              has_one = loan_note_mint)]
    pub reserve: Loader<'info, Reserve>,

    /// The mint for the loan notes being used as loan
    pub loan_note_mint: AccountInfo<'info>,

    /// The user/authority that owns the loan
    #[account(mut, signer)]
    pub owner: AccountInfo<'info>,

    /// The account that will store the loan notes
    #[account(init,
              seeds = [
                  b"loan".as_ref(),
                  reserve.key().as_ref(),
                  obligation.key().as_ref(),
                  owner.key.as_ref()
              ],
              bump = bump,
              token::mint = loan_note_mint,
              token::authority = market_authority,
              payer = owner)]
    pub loan_account: CpiAccount<'info, TokenAccount>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
}

/// Initialize an account that can be used to store loan notes to represent debt in an obligation
pub fn handler(ctx: Context<InitializeLoanAccount>, _bump: u8) -> ProgramResult {
    // Anchor would have already initialized the new loan token account,
    // so all that's left is to register it in the obligation account. This
    // provides an exact record of which accounts are associated with the
    // obligation.

    let mut obligation = ctx.accounts.obligation.load_mut()?;
    let reserve = &ctx.accounts.reserve.load()?;
    let account = ctx.accounts.loan_account.key();

    obligation.register_loan(&account, reserve.index)?;

    msg!("initialized loan account");
    Ok(())
}
