### For debugging transaction failures in liteSvm

```ts
let result = svm.sendTransaction(tx);
console.log("Debugging Transaction status:", "err" in result ? `${result.err().toString()}: ${result.meta().logs()}` : result.logs());
```


#### i64 && u64 matches with le 8.... buffer encoding ... since 8*8 = 64 ?
- for serializing i64 integers you will need 8 byte lil endian formatting
```ts
// typescript test
let startTime = new BN(_startTime).toArrayLike(Buffer, "le", 8);

// assume rust
fn start(ctx: Context<Assume>, startTime: i64) -> Result<()> {}
```

- similarly for u128 or i128 ::: `(Buffer, "le", "16")` // 16*8 == 128!

#### Arthematic operation between BigInt type and normal js types lead unexpected results

- here `challengeStartTime` is of type BigInt read from Pda , challengePDAData.start

```ts
      // log : 17553180701641600
      let nineteenthDay = challengeStartTime + (constants.TWENTY_ONE_DAY_IN_SECONDS - (constants.ONE_DAY_IN_SECONDS * 2)); 

    // log : 1756959529
    let nineteenthDay = Number(challengeStartTime) + (constants.TWENTY_ONE_DAY_IN_SECONDS - (constants.ONE_DAY_IN_SECONDS * 2)); 


```