## Development Mistakes

- using system transfer on pdas while only program can decrease balance ... update it
- closing vault directly without decreasing sol balance by the action of program .... since close=owner only transfers rent exemption amounts beyound that nothing is transferred///// always verify what things are doing

- `maybe using close=signer along with transferring all lamports in closeAccount which makes anchor to silently fail without decreasing or increasing account balance >>>>` not maybe

- **confirmed::: withrawing sol from pda by program but forgetting to out them from program... that is program takes sol from pda but holds them as hostage... maybe**

- forgetting await in calling deposit, confused hell out of me of what the hell is happening with the vault balance, unexpected behaviour like sometimes working sometimes not if func below had await?
- forgetting adding .rpc() in helper function for program method calls