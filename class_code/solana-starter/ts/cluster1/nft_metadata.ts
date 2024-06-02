import wallet from "../wba-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

        const image = 'https://arweave.net/vRfjW9DWOxz0Qs102lhfD8-eJNPHIWZsjD1Zu7Ca844'
        const metadata = {
            name: "Eye of Xaos",
            symbol: "XAOS",
            description: "A pixel art carpet, generated from Langston/Eye example set, by Xaos Fractal Application for WBA-Q2 NFT Course",
            image,
            attributes: [
                {trait_type: 'xaos', value: 'eox01'}
            ],
            properties: {
                files: [
                    {
                        type: "image/jpg",
                        uri: image
                    },
                ]
            }
        };
        const myUri = await umi.uploader.uploadJson(metadata);
        console.log("Your image URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();