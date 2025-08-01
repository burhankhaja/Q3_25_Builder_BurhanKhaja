use anchor_lang::prelude::*;

#[error_code]
pub enum Errors {
    #[msg(
        "There must be atleast 24 delay before the challenge starts, choose different start_time"
    )]
    ChallengeStartsTooSoon,

    #[msg("Challenge can't start more than one week from now, choose lesser start_time")]
    ChallengeStartsTooFar,

    #[msg("Daily Challenges must be under 2 hours")]
    ChallengeExceedsTwoHours,

    #[msg("Can't join because the challenge has already started")]
    JoinedLate,

    #[msg("Addition caused buffer overflows")]
    IntegerOverflow,
}
