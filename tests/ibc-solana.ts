import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { IbcSolana } from "../target/types/ibc_solana";

describe("ibc-solana", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.IbcSolana as Program<IbcSolana>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
