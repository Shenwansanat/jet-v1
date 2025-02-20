use anchor_lang::error;

#[error]
pub enum ErrorCode {
    #[msg("failed to perform some math operation safely")]
    ArithmeticError,

    #[msg("oracle account provided is not valid")]
    InvalidOracle,

    #[msg("no free space left to add a new reserve in the market")]
    NoFreeReserves,

    #[msg("no free space left to add the new loan or collateral in an obligation")]
    NoFreeObligation,

    #[msg("the obligation account doesn't have any record of the loan or collateral account")]
    UnregisteredPosition,

    #[msg("the oracle price account has an invalid price value")]
    InvalidOraclePrice,

    #[msg("there is not enough collateral deposited to borrow against")]
    InsufficientCollateral,

    #[msg("cannot both deposit collateral to and borrow from the same reserve")]
    SimultaneousDepositAndBorrow,

    #[msg("cannot liquidate a healthy position")]
    ObligationHealthy,

    #[msg("cannot perform an action that would leave the obligation unhealthy")]
    ObligationUnhealthy,

    #[msg("reserve requires special action; call refresh_reserve until up to date")]
    ExceptionalReserveState,

    #[msg("the units provided in the amount are not valid for the instruction")]
    InvalidAmountUnits,

    #[msg("the tokens in the DEX market don't match the reserve and lending market quote token")]
    InvalidDexMarketMints,

    #[msg("the market authority provided doesn't match the market account")]
    InvalidMarketAuthority,

    #[msg("the quote token account provided cannot be used for liquidations")]
    InvalidLiquidationQuoteTokenAccount,

    #[msg("the obligation account doesn't have the collateral/loan registered")]
    ObligationAccountMismatch,

    #[msg("unknown instruction")]
    UnknownInstruction,

    #[msg("current conditions prevent an action from being performed")]
    Disallowed,

    #[msg("the actual slipped amount on the DEX trade exceeded the threshold configured")]
    LiquidationSwapSlipped,

    #[msg("the collateral value is too small for a DEX trade")]
    CollateralValueTooSmall,

    #[msg("the collateral returned by the liquidation is smaller than requested")]
    LiquidationLowCollateral,

    #[msg("this action is currently not supported by this version of the program")]
    NotSupported,
}

impl From<jet_math::Error> for ErrorCode {
    fn from(_: jet_math::Error) -> ErrorCode {
        ErrorCode::ArithmeticError
    }
}
