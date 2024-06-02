import { Keypair } from "@solana/web3.js";

let kp = Keypair.generate()

console.log(`A new keypair is created.
Public address: ${kp.publicKey.toBase58()}
Secret: [${kp.secretKey}]`)
