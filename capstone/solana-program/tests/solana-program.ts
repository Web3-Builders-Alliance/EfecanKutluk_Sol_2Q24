import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaProgram } from "../target/types/solana_program";
import dotenv from "dotenv";
dotenv.config()

describe("solana-program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  //const connection = new anchor.web3.Connection("http://127.0.0.1:8899");
  const connection = new anchor.web3.Connection("https://api.devnet.solana.com");
  const creator = anchor.web3.Keypair.fromSecretKey(Uint8Array.from(JSON.parse(process.env.SECRET_KEY)));
  const donator = anchor.web3.Keypair.generate()
  const celeb = anchor.web3.Keypair.generate();
  const alice = anchor.web3.Keypair.generate()
  const bob = anchor.web3.Keypair.generate()

  connection.requestAirdrop(donator.publicKey, 2000000000)
  connection.requestAirdrop(alice.publicKey, 2000000000)
  connection.requestAirdrop(bob.publicKey, 2000000000)

  const program = anchor.workspace.SolanaProgram as Program<SolanaProgram>;

  const campaign_data = {
    title: "Play Dead by Daylight"
  }

  const campaign_seeds = [Buffer.from("wishbourne-campaign"), Buffer.from(campaign_data.title), celeb.publicKey.toBuffer()]
  const [campaignPDA] = anchor.web3.PublicKey.findProgramAddressSync(campaign_seeds, program.programId)

  it("Create a new campaign", async () => {
    const tx = await program.methods.createCampaign(campaign_data.title, celeb.publicKey)
      .accounts({campaign: campaignPDA, initializer: creator.publicKey})
      .signers([creator])
      .rpc();
    console.log("New campaign signature", tx);
  });

  it("Initial donation", async () => {
    const donation_seeds = [Buffer.from("wishbourne-donation"), creator.publicKey.toBuffer(), campaignPDA.toBuffer()]
    const [donationPDA] = anchor.web3.PublicKey.findProgramAddressSync(donation_seeds, program.programId)    
    const tx = await program.methods.createDonationSpace(new anchor.BN(10000000))
      .accounts({donationSpace: donationPDA, campaign: campaignPDA})
      .signers([creator])
      .rpc()
      console.log("Initial donation signature", tx);
  })
});