use crate::{
    helpers::helper_errors::{ArthemeticErrors, TreasuryErrors},
    state::Global,
};
use anchor_lang::prelude::*;

pub fn update_treasury_balance(
    global_account: &mut Account<Global>,
    add: bool,
    amount: u64,
) -> Result<()> {
    if add {
        global_account
            .treasury_balance
            .checked_add(amount)
            .ok_or(ArthemeticErrors::IntegerOverflow)?;
    } else {
        require!(
            global_account.treasury_balance >= amount,
            TreasuryErrors::OverClaim
        );
        global_account
            .treasury_balance
            .checked_sub(amount)
            .ok_or(ArthemeticErrors::IntegerUnderflow)?;
    }
    Ok(())
}
