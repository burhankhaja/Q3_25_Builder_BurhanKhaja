use crate::helpers::helper_errors::{ArthemeticErrors, TransferErrors};
use anchor_lang::prelude::*;

pub fn transfer_from_pda(pda: &AccountInfo, to: &AccountInfo, amount: u64) -> Result<()> {
    let pda_initial_lamports = pda.lamports();
    let to_initial_lamports = to.lamports();

    is_pda_rent_exempt_after_transfer(pda, pda_initial_lamports, amount)?;

    **pda.lamports.borrow_mut() = pda_initial_lamports
        .checked_sub(amount)
        .ok_or(ArthemeticErrors::IntegerUnderflow)?;

    **to.lamports.borrow_mut() = to_initial_lamports
        .checked_add(amount)
        .ok_or(ArthemeticErrors::IntegerOverflow)?;

    Ok(())
}

fn is_pda_rent_exempt_after_transfer(
    pda: &AccountInfo,
    current_balance: u64,
    transfer_amount: u64,
) -> Result<()> {
    let rent = Rent::get()?.minimum_balance(pda.data_len());
    let post_transfer_balance = current_balance
        .checked_sub(transfer_amount)
        .ok_or(ArthemeticErrors::IntegerUnderflow)?;

    require!(
        post_transfer_balance >= rent,
        TransferErrors::PDAInsufficientRent
    );

    Ok(())
}
