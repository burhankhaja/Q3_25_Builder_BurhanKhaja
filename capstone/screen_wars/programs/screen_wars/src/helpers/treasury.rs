use crate::{error::Errors, state::Global};
use anchor_lang::prelude::*;

pub fn update_treasury_profits(global_account: &mut Account<Global>, amount: i64) -> Result<()> {
    (global_account.treasury_profits as i64)
        .checked_add(amount)
        .ok_or(Errors::IntegerBoundsExceeded)?;
    Ok(())
}
