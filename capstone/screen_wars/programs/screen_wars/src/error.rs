use anchor_lang::prelude::*;

#[error_code]
pub enum Errors {
    #[msg("Admin has disabled the creation of any more challenges")]
    ChallengeCreationPaused,

    #[msg("Challenge creation state is already set to desired value")]
    ChallengeStateAlreadySet,

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

    #[msg("Your streak is lower than current temporary winner")]
    LowerStreak,

    #[msg("Challenge has not ended yet")]
    ChallengeNotEnded,

    #[msg("The 5-day window to claim the winner position has expired")]
    ContentionExpired,

    #[msg("You are not enrolled in the given challenge")]
    NotEnrolled,

    #[msg("Oops... You ain't winner")]
    NotWinner,

    #[msg("The protocol is in 5 day winner contention phase, try claiming winnership")]
    ContentionPhase,

    #[msg("Admin cant withdraw more than treasury profits")]
    OverClaim,

    #[msg("Operation caused buffer overflows")]
    IntegerOverflow,

    #[msg("Operation caused buffer underflows")]
    IntegerUnderflow,
}
