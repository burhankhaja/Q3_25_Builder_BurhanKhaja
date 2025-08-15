### For debugging transaction failures in liteSvm

```ts
let result = svm.sendTransaction(tx);
console.log("Debugging Transaction status:", "err" in result ? `${result.err().toString()}: ${result.meta().logs()}` : result.logs());
```


#### i64 matches with le 8.... buffer encoding
- for serializing i64 integers you will need 8 byte lil endian formatting
```ts
// typescript test
let startTime = new BN(_startTime).toArrayLike(Buffer, "le", 8);

// assume rust
fn start(ctx: Context<Assume>, startTime: i64) -> Result<()> {}
```

#### Arthematic operation between BigInt type and normal js types lead unexpected results

- here `challengeStartTime` is of type BigInt read from Pda , challengePDAData.start

```ts
      // log : 17553180701641600
      let nineteenthDay = challengeStartTime + (constants.TWENTY_ONE_DAY_IN_SECONDS - (constants.ONE_DAY_IN_SECONDS * 2)); 

    // log : 1756959529
    let nineteenthDay = Number(challengeStartTime) + (constants.TWENTY_ONE_DAY_IN_SECONDS - (constants.ONE_DAY_IN_SECONDS * 2)); 


```