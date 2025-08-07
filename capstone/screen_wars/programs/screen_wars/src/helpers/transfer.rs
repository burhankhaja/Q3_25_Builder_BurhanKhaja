use crate::helpers::helper_errors::{ArthemeticErrors, TransferErrors};
use anchor_lang::prelude::*;
use std::io::Write;
use std::ops::DerefMut;

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

//// For Closing PDA securely
pub const CLOSED_ACCOUNT_DISCRIMINATOR: [u8; 8] = [255; 8];

/// @dev Prevents re-initialization attacks by zeroing out account data and writing a special "closed" discriminator.
///      This follows best practices for secure account closing in Solana:
///      https://solana.com/developers/courses/program-security/closing-accounts#secure-account-closing
///
/// @notice Be aware of the "limbo account" issue, where a closed account may linger in memory without being fully reaped.
///         This is a minor concern for most programs, but you can learn more here:
///         https://[...]#manual-force-defund
pub fn close_pda(pda: &AccountInfo, to: &AccountInfo) -> Result<()> {
    let pda_initial_lamports = pda.lamports();
    let to_initial_lamports = to.lamports();

    **pda.lamports.borrow_mut() = 0;

    **to.lamports.borrow_mut() = to_initial_lamports
        .checked_add(pda_initial_lamports)
        .ok_or(ArthemeticErrors::IntegerOverflow)?;

    // zero out data
    let mut data = pda.try_borrow_mut_data()?;
    for byte in data.deref_mut().iter_mut() {
        *byte = 0;
    }

    let dst: &mut [u8] = &mut data;
    let mut cursor = std::io::Cursor::new(dst);
    cursor.write_all(&CLOSED_ACCOUNT_DISCRIMINATOR).unwrap();
    ////

    Ok(())
}
