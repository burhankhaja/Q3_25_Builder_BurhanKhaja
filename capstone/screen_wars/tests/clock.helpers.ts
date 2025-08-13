import { LiteSVM, Clock} from "litesvm";

export async function setInitialClock(svm: LiteSVM) {
    // Manually create a Clock with desired timestamp (in BigInt)
    const newClock = new Clock(
        BigInt(12345),                      // slot
        BigInt(1234000),                    // epochStartTimestamp (you need a value here)
        BigInt(1),                         // epoch
        BigInt(1),                         // leaderScheduleEpoch
        BigInt(Math.floor(Date.now() / 1000)) // unixTimestamp
    );

    // Set the clock sysvar
    svm.setClock(newClock);
}

// get current unix_timestamp of svm
export async function now(svm : LiteSVM) : Promise<number> {
   let timestamp = svm.getClock().unixTimestamp;
   return Number(timestamp);
}