use crate::{helpers::helper_errors::ArthemeticErrors, state::Global};
use anchor_lang::prelude::*;

pub fn update_treasury_profits(
    global_account: &mut Account<Global>,
    add: bool,
    amount: u64,
) -> Result<()> {
    if add {
        global_account
            .treasury_profits
            .checked_add(amount)
            .ok_or(ArthemeticErrors::IntegerOverflow)?;
    } else {
        global_account
            .treasury_profits
            .checked_sub(amount)
            .ok_or(ArthemeticErrors::IntegerUnderflow)?;
    }
    Ok(())
}
