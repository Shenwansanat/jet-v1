#![cfg_attr(feature = "no-entrypoint", allow(dead_code))]

use anchor_lang::prelude::*;

extern crate static_assertions;

pub mod errors;
mod instructions;
pub mod state;
pub mod utils;

use errors::ErrorCode;
use instructions::*;
use state::*;

pub const LIQUIDATE_DEX_INSTR_ID: [u8; 8] = [28, 129, 253, 125, 243, 52, 11, 162];

#[program]
mod jet {
    use super::*;

    /// Initialize a new empty market with a given owner.
    pub fn init_market(
        ctx: Context<InitializeMarket>,
        owner: Pubkey,
        quote_currency: String,
        quote_token_mint: Pubkey,
    ) -> ProgramResult {
        instructions::init_market::handler(ctx, owner, quote_currency, quote_token_mint)
    }

    /// Initialize a new reserve in a market with some initial liquidity.
    pub fn init_reserve(
        ctx: Context<InitializeReserve>,
        bump: InitReserveBumpSeeds,
        config: ReserveConfig,
    ) -> ProgramResult {
        instructions::init_reserve::handler(ctx, bump, config)
    }

    /// Initialize an account that can be used to store deposit notes
    pub fn init_deposit_account(ctx: Context<InitializeDepositAccount>, bump: u8) -> ProgramResult {
        instructions::init_deposit_account::handler(ctx, bump)
    }

    /// Initialize an account that can be used to store deposit notes as collateral
    pub fn init_collateral_account(
        ctx: Context<InitializeCollateralAccount>,
        bump: u8,
    ) -> ProgramResult {
        instructions::init_collateral_account::handler(ctx, bump)
    }

    /// Initialize an account that can be used to store deposit notes as collateral
    pub fn init_loan_account(ctx: Context<InitializeLoanAccount>, bump: u8) -> ProgramResult {
        instructions::init_loan_account::handler(ctx, bump)
    }

    /// Initialize an account that can be used to borrow from a reserve
    pub fn init_obligation(ctx: Context<InitializeObligation>, bump: u8) -> ProgramResult {
        instructions::init_obligation::handler(ctx, bump)
    }

    /// Close a deposit account
    pub fn close_deposit_account(ctx: Context<CloseDepositAccount>, bump: u8) -> ProgramResult {
        instructions::close_deposit_account::handler(ctx, bump)
    }

    /// Deposit tokens into a reserve
    pub fn deposit(ctx: Context<Deposit>, bump: u8, amount: Amount) -> ProgramResult {
        instructions::deposit::handler(ctx, bump, amount)
    }

    /// Deposit tokens from a reserve
    pub fn withdraw(ctx: Context<Withdraw>, bump: u8, amount: Amount) -> ProgramResult {
        instructions::withdraw::handler(ctx, bump, amount)
    }

    /// Deposit notes as collateral in an obligation
    pub fn deposit_collateral(
        ctx: Context<DepositCollateral>,
        bump: DepositCollateralBumpSeeds,
        amount: Amount,
    ) -> ProgramResult {
        instructions::deposit_collateral::handler(ctx, bump, amount)
    }

    /// Withdraw notes previously deposited as collateral in an obligation
    pub fn withdraw_collateral(
        ctx: Context<WithdrawCollateral>,
        bump: WithdrawCollateralBumpSeeds,
        amount: Amount,
    ) -> ProgramResult {
        instructions::withdraw_collateral::handler(ctx, bump, amount)
    }

    /// Borrow tokens from a reserve
    pub fn borrow(ctx: Context<Borrow>, bump: u8, amount: Amount) -> ProgramResult {
        instructions::borrow::handler(ctx, bump, amount)
    }

    /// Repay a loan
    pub fn repay(ctx: Context<Repay>, amount: Amount) -> ProgramResult {
        instructions::repay::handler(ctx, amount)
    }

    /// Liquidate an unhealthy loan
    pub fn liquidate(
        ctx: Context<Liquidate>,
        amount: Amount,
        min_collateral: u64,
    ) -> ProgramResult {
        instructions::liquidate::handler(ctx, amount, min_collateral)
    }

    /// Liquidate an unhealthy loan
    pub fn mock_liquidate_dex(_ctx: Context<MockLiquidateDex>) -> ProgramResult {
        panic!("not supported")
    }

    /// Refresh a reserve's market price and interest owed
    ///
    /// If the reserve is extremely stale, only a partial update will be
    /// performed. It may be necessary to call refresh_reserve multiple
    /// times to get the reserve up to date.
    pub fn refresh_reserve(ctx: Context<RefreshReserve>) -> ProgramResult {
        instructions::refresh_reserve::handler(ctx)
    }

    /// Route super special instructions
    pub fn default<'info>(
        program_id: &Pubkey,
        accounts: &[AccountInfo<'info>],
        ix_data: &[u8],
    ) -> ProgramResult {
        if &ix_data[..8] == &LIQUIDATE_DEX_INSTR_ID {
            instructions::liquidate_dex::handler_raw(program_id, accounts, &ix_data[8..])?;
        } else {
            return Err(ErrorCode::UnknownInstruction.into());
        }

        Ok(())
    }
}

/// Specifies the units of some amount of value
#[derive(AnchorDeserialize, AnchorSerialize, Eq, PartialEq, Debug, Clone, Copy)]
pub enum AmountUnits {
    Tokens,
    DepositNotes,
    LoanNotes,
}

/// Represent an amount of some value (like tokens, or notes)
#[derive(AnchorDeserialize, AnchorSerialize, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Amount {
    pub units: AmountUnits,
    pub value: u64,
}

impl Amount {
    /// Get the amount represented in tokens
    pub fn tokens(&self, reserve_info: &CachedReserveInfo) -> u64 {
        match self.units {
            AmountUnits::Tokens => self.value,
            AmountUnits::DepositNotes => reserve_info.deposit_notes_to_tokens(self.value),
            AmountUnits::LoanNotes => reserve_info.loan_notes_to_tokens(self.value),
        }
    }

    /// Get the amount represented in deposit notes
    pub fn as_deposit_notes(&self, reserve_info: &CachedReserveInfo) -> Result<u64, ErrorCode> {
        match self.units {
            AmountUnits::Tokens => Ok(reserve_info.deposit_notes_from_tokens(self.value)),
            AmountUnits::DepositNotes => Ok(self.value),
            AmountUnits::LoanNotes => Err(ErrorCode::InvalidAmountUnits),
        }
    }

    /// Get the amount represented in loan notes
    pub fn as_loan_notes(&self, reserve_info: &CachedReserveInfo) -> Result<u64, ErrorCode> {
        match self.units {
            AmountUnits::Tokens => Ok(reserve_info.loan_notes_from_tokens(self.value)),
            AmountUnits::LoanNotes => Ok(self.value),
            AmountUnits::DepositNotes => Err(ErrorCode::InvalidAmountUnits),
        }
    }

    pub fn from_tokens(value: u64) -> Amount {
        Amount {
            units: AmountUnits::Tokens,
            value,
        }
    }

    pub fn from_deposit_notes(value: u64) -> Amount {
        Amount {
            units: AmountUnits::DepositNotes,
            value,
        }
    }

    pub fn from_loan_notes(value: u64) -> Amount {
        Amount {
            units: AmountUnits::LoanNotes,
            value,
        }
    }
}
