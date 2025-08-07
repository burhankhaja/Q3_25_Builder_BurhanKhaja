use anchor_lang::prelude::*;

pub fn mock_offchain_oracle_component(debug: Option<DebugData>) -> Result<(bool, u8)> {
    let user_passed;
    let days_not_synced;

    match debug {
        Some(data) => {
            user_passed = data.user_passed;
            days_not_synced = data.days_not_synced;
        }

        None => {
            user_passed = true;
            days_not_synced = 0;
        }
    }

    Ok((user_passed, days_not_synced))
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct DebugData {
    pub user_passed: bool,
    pub days_not_synced: u8,
}
