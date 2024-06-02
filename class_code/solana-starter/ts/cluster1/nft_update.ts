import { PublicKey } from "@solana/web3.js"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createSignerFromKeypair, signerIdentity, generateSigner, percentAmount, publicKey } from "@metaplex-foundation/umi"
import {
    updateV1,
    fetchMetadataFromSeeds,
    mplTokenMetadata
} from '@metaplex-foundation/mpl-token-metadata'

import wallet from "../wba-wallet.json"
import mintAddressObj from "../mandelbrugged.json";
import base58 from "bs58";

const RPC_ENDPOINT = "https://api.devnet.solana.com";
const umi = createUmi(RPC_ENDPOINT);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const updateAuthority = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(updateAuthority));
umi.use(mplTokenMetadata())

const mint = publicKey(mintAddressObj.mint);

(async () => {
    const initialMetadata = await fetchMetadataFromSeeds(umi, { mint })
    const tx = await updateV1(umi, {
        mint,
        authority: updateAuthority,
        data: { ...initialMetadata, symbol: 'XAOS' },
    }).sendAndConfirm(umi)
    const signature = base58.encode(tx.signature);
    
    console.log(`Succesfully Updated the asset! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`)
})()