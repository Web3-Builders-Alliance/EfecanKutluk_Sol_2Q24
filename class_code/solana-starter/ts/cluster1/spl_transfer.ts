import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "../wba-wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("HY3hvVrfx4VUnmEQ2GzKV5rgniYt56rSNac9JvVH3dEb");

// Recipient address
const to = new PublicKey("CgzsSy3ZadYD6gSnkEcKdeHMUoPqGtMxb3gr2FS6pE3k");

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const fromWallet = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            keypair.publicKey
        )

        // Get the token account of the toWallet address, and if it does not exist, create it
        const toWallet = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            to
        )

        // Transfer the new token to the "toTokenAccount" we just created
        const txn = await transfer(
            connection,
            keypair,
            fromWallet.address,
            toWallet.address,
            keypair,
            1e3
        )
        console.log(txn);
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();