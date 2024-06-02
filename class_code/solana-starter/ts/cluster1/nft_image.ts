import wallet from "../wba-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"
import { readFile } from "fs/promises"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        //1. Load image
        const imageBuffer = await readFile("./assets/eyeofxaos01.jpg")

        //2. Convert image to generic file.
        const imageFile = createGenericFile(imageBuffer, "eyeofxaos01-cover.jpg", {
            contentType: "image/jpg"
        })

        //3. Upload image
        const uris = await umi.uploader.upload([imageFile], {
            onProgress: (args) => {
                console.log("[info] uploading...", args);
            }
        })

        // image url: "https://arweave.net/pjRzlaBh8YWwvvSMfkyBRgGRLRVl4KJdRwwR7sJ8Xqc"
        console.log("Your image URI: ", uris);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
