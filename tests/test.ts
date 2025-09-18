import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Test } from "../target/types/test";

describe("test", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.test as Program<Test>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    const [configAddr] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("config")],
      program.programId
    );
    const config = await program.account.programConfig.fetch(configAddr);
    console.log("Your transaction signature", tx);
    console.log("Your config account", config);
  });
});
