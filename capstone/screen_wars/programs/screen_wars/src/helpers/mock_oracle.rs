use anchor_lang::prelude::*;

pub fn mock_offchain_oracle_component(debug: Option<DebugData>) -> Result<(bool, u8, bool)> {
    let user_passed;
    let days_not_synced;
    let synced_today;

    match debug {
        Some(data) => {
            user_passed = data.user_passed;
            days_not_synced = data.days_not_synced;
            synced_today = data.synced_today;
        }

        None => {
            user_passed = true;
            days_not_synced = 0;
            synced_today = false;
        }
    }

    Ok((user_passed, days_not_synced, synced_today))
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct DebugData {
    pub user_passed: bool,
    pub days_not_synced: u8,
    pub synced_today: bool,
}
