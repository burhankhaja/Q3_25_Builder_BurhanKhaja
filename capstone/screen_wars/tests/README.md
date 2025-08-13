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