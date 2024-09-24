import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RentalProgram } from "../target/types/rental_program";
import { expect } from "chai";

describe("rental-program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.RentalProgram as Program<RentalProgram>;
  const amount = 1000;
  const [bountyAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("bounty"), provider.wallet.publicKey.toBuffer()],
    program.programId
  );
  const description = "Bounty";
  it("Create bounty", async () => {
    const tx = await program.methods.createBounty().rpc();
    const acc = await program.account.bounty.fetch(bountyAccount);
    console.log(acc);
    expect(acc.amount).to.equal(amount);
  });
});
