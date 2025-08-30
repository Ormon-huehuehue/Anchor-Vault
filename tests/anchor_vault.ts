import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorVault } from "../target/types/anchor_vault";
import { SystemProgram, PublicKey, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { expect } from "chai";

describe("anchor_vault", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  const program = anchor.workspace.anchorVault as Program<AnchorVault>;

  const signer = provider.wallet;

  let vaultPda : PublicKey;
  let vaultBump : number;

  before(async ()=>{
    // derive PDA for signer
    [vaultPda, vaultBump] = PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), signer.publicKey.toBuffer()],
      program.programId
    );
  });

  it("deposits into vault", async () => {
    const amount = 2 * LAMPORTS_PER_SOL;

    await program.methods
    .deposit(new anchor.BN(amount))
    .accounts({
      signer : signer.publicKey,
      vault : vaultPda,
      systemProgram : SystemProgram.programId
    })
    .rpc();

    const vaultAccount = await provider.connection.getAccountInfo(vaultPda);
    expect(vaultAccount.lamports).to.equal(amount);
    
  });


  it("withdraws from vault", async()=>{
    const prebalance = await provider.connection.getBalance(signer.publicKey);
  
    await program.methods
    .withdraw()
    .accounts({
      signer : signer.publicKey,
      vault : vaultPda,
      systemProgram : SystemProgram.programId
    })
    .rpc();
    

    const vaultAccount = await provider.connection.getAccountInfo(vaultPda);
    //withdrawing all funds should close the vault account
    expect(vaultAccount).to.be.null;  

    const postbalance = await provider.connection.getBalance(signer.publicKey);
    expect(postbalance).to.be.greaterThan(prebalance);
  })
  

});


