/**
 * Importing the necessary modules and types for the test.
 */

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MyVaultProgram } from "../target/types/my_vault_program";
import { assert } from "chai";
import { SystemProgram, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";

describe("my_vault_program", () => {
  // Configurations for the test
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  // Loading the program 
  const program = anchor.workspace.my_vault_program as Program<MyVaultProgram>;

  // Store common variables used across tests
  let vaultPda: PublicKey;
  let feeCollector: anchor.web3.Keypair;

  /**
   * This runs before all tests to set up common resources.
   */
  before(async () => {
    // Creating a fee collector account to receive fees
    feeCollector = anchor.web3.Keypair.generate();

    // Funding the fee collector with some SOL
    const signature = await provider.connection.requestAirdrop (
      feeCollector.publicKey,
      2 * LAMPORTS_PER_SOL
    );

    await provider.connection.confirmTransaction(signature);
  });

  /**
   * Test 1: initializing the vault
   */
  it ("Initializes the vault", async () => {
    // Derive the PDA for the vault account based on the same seeds used in the program
    const [derivedVaultPda, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );
    // Store it for later tests
    vaultPda = derivedVaultPda;

    console.log("Derived vault PDA: ", vaultPda.toBase58());

    try {
      // Initializing the vault by calling the init_vault instruction
      const tx = await program.methods
        .initVault()
        .accounts({
          initializer: provider.wallet.publicKey,
          vault: vaultPda,
          systemProgram: SystemProgram.programId,
        } as any)
        .rpc();
      console.log("Transaction signature: ", tx);
      // Fetch the vault account to verify its state
      const vaultAccount = await program.account.vaultAccount.fetch(vaultPda);
      console.log("Vault account:", vaultAccount);

      // Assertions to verify the test passed correctly
      assert.equal(vaultAccount.balance.toString(), "0", "Initial balance should be 0");
      assert.equal(
        vaultAccount.owner.toBase58(),
        provider.wallet.publicKey.toBase58(),
        "Owner should be the initializer"
      );
    } catch (error) {
      console.error("Error initializing vault:", error);
      throw error;
    }
  });

  /**
   * Test 2: depositing into the vault
   */
  it("Deposits into the vault", async () => {
    // We'll deposit 0.5 SOL into the vault
    const depositAmount = new anchor.BN(0.5 * LAMPORTS_PER_SOL);

    // Get the initial balance of the vault for later comparison
    const initialVaultAccount = await program.account.vaultAccount.fetch(vaultPda);
    const initialBalance = initialVaultAccount.balance;
    console.log(`Initial vault balance:  ${initialBalance.toString()}`);

    try {
      // Call the deposit instruction
      const tx = await program.methods
        .deposit(depositAmount) // This maps to deposit in your Rust code
        .accounts({
          initializer: provider.wallet.publicKey,
          vault: vaultPda,
          feeCollector: feeCollector.publicKey,
          systemProgram: SystemProgram.programId,
        } as any) // Type assertion to avoid TypeScript error
        .rpc();

      console.log("Deposit Transaction signature: ", tx);
      // Fetch the vault account again to verify the deposit has worked
      const updatedVaultAccount = await program.account.vaultAccount.fetch(vaultPda);
      console.log(`Updated vault balance: ${updatedVaultAccount.balance.toString()} lamports`);

      // Calculate expected balance - note that there's a fee calculation in the program
      const expectedIncrease = depositAmount.toNumber();

      // Getting the actual increase
      const actualIncrease = updatedVaultAccount.balance.sub(initialBalance).toNumber();

      // Check that the balance increased
      assert(
        updatedVaultAccount.balance.gt(initialBalance),
        "balance should increase after deposit"
      );

      console.log(`Balance increased by ${actualIncrease} lamports`);

    } catch (error) {
      console.error("Deposit error: ", error);
      throw error;
    }
  });

  /**
   * Test 3: Withdraw funds from the vault
   */
  // later


});