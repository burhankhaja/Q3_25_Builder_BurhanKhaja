use crate::{error::Errors, state::Global};
use anchor_lang::prelude::*;

pub fn update_treasury_profits(global_account: &mut Account<Global>, amount: i64) -> Result<()> {
    let updated_profits = (global_account.treasury_profits as i64)
        .checked_add(amount)
        .ok_or(Errors::IntegerBoundsExceeded)?;

    global_account.treasury_profits = if updated_profits >= 0 {
        updated_profits as u64
    } else {
        -(updated_profits) as u64
    };

    Ok(())
}
