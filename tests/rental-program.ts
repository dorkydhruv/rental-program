import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RentalProgram } from "../target/types/rental_program";
import { expect } from "chai";

describe("rental-program", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.RentalProgram as Program<RentalProgram>;
  const [bountyAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("bounty"), provider.wallet.publicKey.toBuffer()],
    program.programId
  );
  const data = {
    id: "672",
    amount: new anchor.BN(1.2 * anchor.web3.LAMPORTS_PER_SOL),
  };
  it("Creates a bounty", async () => {
    const tx = await program.methods
      .createBounty(data.id, data.amount)
      .accounts({ client: provider.wallet.publicKey })
      .rpc();
    console.log(tx);
    const acc = await program.account.bounty.fetch(bountyAccount);
    // expect(acc.id).to.eq(data.id);
    console.log(acc);
  });
});
