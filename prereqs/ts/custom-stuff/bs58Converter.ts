import wallet from '../Turbin3-wallet.json';
import bs58 from 'bs58';

//@dev @burhankhaja
// This tool helps to convert a secret key from either base58-uint8 array or vice-versa.
// Helpful when importing wallets from Phantom as it uses Base58 encoding for secret keys.

function convertPhantomStyleBase58ToUint8Array() {
  const phantomBase58Key = "REDACTED"; // @note :: Replace with your actual Phantom style Base58 key
  const secretKeyArray = bs58.decode(phantomBase58Key);
   console.log("Secret Key Array (Uint8Array):", JSON.stringify(Array.from(secretKeyArray), null, 2));
}

function convertUint8ArrayToPhantomStyleBase58() {
  const secretKeyArray = Uint8Array.from(new Uint8Array(wallet));
  const phantomBase58Key = bs58.encode(secretKeyArray);
  console.log("Phantom Style Base58 Key:", phantomBase58Key);
}

convertUint8ArrayToPhantomStyleBase58();
convertPhantomStyleBase58ToUint8Array();



