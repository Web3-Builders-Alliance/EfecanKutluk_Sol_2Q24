import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Vote } from "../target/types/vote";

describe("vote", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.Vote as Program<Vote>;

  const url = "https://wba.dev";

  const voteAccount = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(url)],
    program.programId
  )[0];

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
    .initialize(url)
    .accounts({
      payer: provider.wallet.publicKey,
      voteState: voteAccount,
      systemProgram: anchor.web3.SystemProgram.programId
    })
    .rpc();
    console.log("Init transaction signature: ", tx);

    const voteState = await program.account.voteState.fetch(voteAccount);
    console.log("\nYour vote score is: ", voteState.score.toString())
  });

  it("Upvote!", async() => {
    const tx = await program.methods
    .upvote(url)
    .accounts({
      voteState: voteAccount
    })
    .rpc();

    console.log("Upvote transaction signature: ", tx);

    const voteState = await program.account.voteState.fetch(voteAccount);
    console.log("\nYour vote score is: ", voteState.score.toString());
  })
});
