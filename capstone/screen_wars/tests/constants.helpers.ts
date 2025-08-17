import { PublicKey } from "@solana/web3.js";

export const ONE_HOUR_IN_SECONDS = 60 * 60;
export const ONE_DAY_IN_SECONDS = 24 * ONE_HOUR_IN_SECONDS;
export const TWENTY_ONE_DAY_IN_SECONDS = 21 * (24 * ONE_HOUR_IN_SECONDS);
export const defaultPubkey = new PublicKey("11111111111111111111111111111111");

export const DAILY_LAMPORTS = 10_000_000;
export const txFeeInLamports = 5_000;