import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorVault } from "../target/types/anchor_vault";
import { SystemProgram, PublicKey } from "@solana/web3.js";
import { expect } from "chai";

describe("anchor_vault", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  const program = anchor.workspace.anchorVault as Program<AnchorVault>;

  const signer = provider.wallet;

  let vaultPDA : PublicKey;
  let vaultBump : number;

  before(async ()=>{
    // derive PDA for signer
    [vaultPDA, vaultBump] = PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), signer.publicKey.toBuffer()],
      program.programId
    );
  })

  it("deposits into vault!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
