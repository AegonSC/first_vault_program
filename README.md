# Considerations: I'm still working on the tests

# My Vault Program

A Solana program that implements a secure vault system for storing and managing SOL tokens with ownership controls and a fee structure.

## Overview

My Vault Program is a Solana blockchain program built using the Anchor framework. It enables users to:

- Create personal vaults for secure token storage
- Deposit SOL into vaults with a small fee
- Withdraw SOL from vaults (owner only)
- Transfer vault ownership to another user
- Close vaults and recover remaining funds

Each vault is associated with the user who created it and can only be modified by its owner.

## Features

- **Secure Ownership**: All operations that modify vault state require owner authentication
- **Fee Structure**: A 1% fee is applied to all deposits and withdrawals
- **Event Emission**: Key actions emit events for off-chain tracking
- **Proper Error Handling**: Clear error messages for unauthorized access and insufficient funds
- **Program Derived Addresses (PDAs)**: Vaults are created as PDAs for secure state management

## Technical Architecture

### Accounts

- **VaultAccount**: Stores the vault balance and owner's public key

### Instructions

1. **init_vault**: Create a new vault associated with the caller
2. **deposit**: Add funds to a vault (1% fee applied)
3. **withdraw**: Remove funds from a vault (owner only, 1% fee applied)
4. **transfer_ownership**: Change the vault owner to another public key
5. **close_vault**: Close the vault and recover any remaining funds

### Events

- **DepositEvent**: Emitted when funds are deposited into a vault
- **WithdrawEvent**: Emitted when funds are withdrawn from a vault
- **CloseVaultEvent**: Emitted when a vault is closed

## Code Structure

```
my_vault_program/
├── programs/
│   └── my_vault_program/
│       ├── src/
│       │   ├── lib.rs           # Main program logic and account structures
│       │   ├── events.rs        # Event definitions
│       │   └── helpers.rs       # Helper functions for fees and validation
│       └── Cargo.toml
└── tests/
    └── my_vault_program.ts      # TypeScript tests
```

## Usage

### Creating a Vault

```typescript
const tx = await program.methods
  .initVault()
  .accounts({
    initializer: wallet.publicKey,
    vault: vaultPda,
    systemProgram: anchor.web3.SystemProgram.programId,
  })
  .rpc();
```

### Depositing Funds

```typescript
const amount = new anchor.BN(1_000_000); // 1 SOL in lamports
const tx = await program.methods
  .deposit(amount)
  .accounts({
    initializer: wallet.publicKey,
    vault: vaultPda,
    feeCollector: feeCollectorPubkey,
    systemProgram: anchor.web3.SystemProgram.programId,
  })
  .rpc();
```

### Withdrawing Funds

```typescript
const amount = new anchor.BN(500_000); // 0.5 SOL in lamports
const tx = await program.methods
  .withdraw(amount)
  .accounts({
    initializer: wallet.publicKey,
    vault: vaultPda,
    feeCollector: feeCollectorPubkey,
    systemProgram: anchor.web3.SystemProgram.programId,
  })
  .rpc();
```

### Transferring Ownership

```typescript
const tx = await program.methods
  .transferOwnership(newOwnerPubkey)
  .accounts({
    initializer: wallet.publicKey,
    vault: vaultPda,
    systemProgram: anchor.web3.SystemProgram.programId,
  })
  .rpc();
```

### Closing a Vault

```typescript
const tx = await program.methods
  .closeVault()
  .accounts({
    initializer: wallet.publicKey,
    vault: vaultPda,
    systemProgram: anchor.web3.SystemProgram.programId,
  })
  .rpc();
```

## Security Considerations

1. All state-changing operations require owner authentication
2. PDA derivation ensures vaults can only be created in the intended way
3. Balance checks prevent withdrawals exceeding available funds
4. Close instruction properly handles remaining funds

## Development and Testing

### Prerequisites

- Rust and Cargo
- Solana CLI tools
- Anchor Framework
- Node.js and npm/yarn

## License

This project is licensed under the MIT License - see the LICENSE file for details.
