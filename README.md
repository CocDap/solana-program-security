# Solana Fellowship Program 2024 - Module 5 - Introduction to Program Security

Write a simple blog, or a README about the issues and how to fix them in the Anchor program below, and submit it in the airtable form shared.


## Integer Overflow Security


Rust integers have fixed sizes. This means they can only support a specific range of numbers. An arithmetic operation that results in a higher or lower value than what is supported by the range will cause the resulting value to wrap around. For example, a u8 only supports numbers 0-255

In this example, we have `Counter` program that define `Counter` account have `count` with `u8` . It means that `count` have maximum value is `255` 

### Program with overflow issue 

Context: 
+ Initialize `count` with an initial value of `100`
+ The `increment` function has one input, which is `additional_count`


```rust
use anchor_lang::prelude::*;

declare_id!("6hRAggxb9v4RNqWBW891tqjXPD4rjFui7Ndh3XHxXKWX");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 100;
        msg!("Counter account created! Current count: {}", counter.count);
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>, additional_count: u8) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        msg!("Previous counter: {}", counter.count);
        
        // increase with additional count -> maybe happen overflow issue
        
        counter.count = counter.count + additional_count;
        msg!("Counter incremented! Current count: {}", counter.count);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,         
        payer = user,
        space = 8 + 8 
    )]
    pub counter: Account<'info, Counter>, 
    pub system_program: Program<'info, System>, 
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)] 
    pub counter: Account<'info, Counter>, 
}


#[account]
pub struct Counter {
    pub count: u8, // define count value type as u8
}

```

### Run test with overflow issue

Context: 
+ Increase the value by 200 -> Overflow Issue


```javascript
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";
import {
  Keypair,
  sendAndConfirmTransaction,
  Transaction,
} from "@solana/web3.js";

describe("counter", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  const wallet = provider.wallet as anchor.Wallet;
  const connection = provider.connection;
  anchor.setProvider(provider);

  const program = anchor.workspace.Counter as Program<Counter>;

  // Generate a new keypair to use as the address the counter account
  const counterAccount = new Keypair();

  it("Is initialized!", async () => {
    const instruction = await program.methods
      .initialize()
      .accounts({
        user: wallet.publicKey,
        counter: counterAccount.publicKey,
      })
      .instruction();

    const transaction = new Transaction().add(instruction);

    await sendAndConfirmTransaction(connection, transaction, [
      wallet.payer,
      counterAccount,
    ]);
  });

  it("Is initialized!", async () => {
    // Invoke the initialize instruction
    const transactionSignature = await program.methods
      .initialize()
      .accounts({
        counter: counterAccount.publicKey,
      })
      .signers([counterAccount]) // include counter keypair as additional signer
      .rpc({ skipPreflight: true });

    // Fetch the counter account data
    const accountData = await program.account.counter.fetch(
      counterAccount.publicKey
    );

    console.log(`Transaction Signature: ${transactionSignature}`);
    console.log(`Count: ${accountData.count}`);
  });

  it("Increment", async () => {
    // Invoke the increment instruction
    const transactionSignature = await program.methods
      .increment(200)
      .accounts({
        counter: counterAccount.publicKey,
      })
      .rpc();

    // Fetch the counter account data
    const accountData = await program.account.counter.fetch(
      counterAccount.publicKey
    );

    console.log(`Transaction Signature: ${transactionSignature}`);
    console.log(`Count: ${accountData.count}`);
  });
});


```

### Result after running test -> attempt to add with overflow

```  counter
    ✔ Is initialized! (1765ms)
    1) Is initialized!
    2) Increment


  1 passing (3s)
  2 failing

  1) counter
       Is initialized!:
     Error: Unknown action 'undefined'
      at AnchorProvider.sendAndConfirm (node_modules/@coral-xyz/anchor/src/provider.ts:186:31)
      at processTicksAndRejections (node:internal/process/task_queues:95:5)
      at MethodsBuilder.rpc [as _rpcFn] (node_modules/@coral-xyz/anchor/src/program/namespace/rpc.ts:29:16)

  2) counter
       Increment:
     Error: Simulation failed. 
Message: Transaction simulation failed: Error processing Instruction 0: Program failed to complete. 
Logs: 
[
  "Program 6hRAggxb9v4RNqWBW891tqjXPD4rjFui7Ndh3XHxXKWX invoke [1]",
  "Program log: Instruction: Increment",
  "Program log: Previous counter: 100",
  "Program log: panicked at programs/counter/src/lib.rs:22:25:\nattempt to add with overflow",
  "Program 6hRAggxb9v4RNqWBW891tqjXPD4rjFui7Ndh3XHxXKWX consumed 2652 of 200000 compute units",
  "Program 6hRAggxb9v4RNqWBW891tqjXPD4rjFui7Ndh3XHxXKWX failed: SBF program panicked"
]. 
Catch the `SendTransactionError` and call `getLogs()` on it for full details.
      at Connection.sendEncodedTransaction (node_modules/@solana/web3.js/src/connection.ts:6043:13)
      at processTicksAndRejections (node:internal/process/task_queues:95:5)
      at Connection.sendRawTransaction (node_modules/@solana/web3.js/src/connection.ts:5999:20)
      at sendAndConfirmRawTransaction (node_modules/@coral-xyz/anchor/src/provider.ts:377:21)
      at AnchorProvider.sendAndConfirm (node_modules/@coral-xyz/anchor/src/provider.ts:163:14)
      at MethodsBuilder.rpc [as _rpcFn] (node_modules/@coral-xyz/anchor/src/program/namespace/rpc.ts:29:16)
```

-> Result : panic error overflow 


### Fix overflow issue 

Update `increment` function logic -> unwrap safely

```rust
    pub fn increment(ctx: Context<Increment>, additional_count: u8) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        msg!("Previous counter: {}", counter.count);
        
        // increase with additional count -> maybe happen overflow issue
        // using saturating_add method -> overflow -> return 0 
        counter.count = counter.count.saturating_add(additional_count);
        msg!("Counter incremented! Current count: {}", counter.count);
        Ok(())
    }
```

### Build program again and run test

```
  counter
    ✔ Is initialized! (4065ms)
    1) Is initialized!
Transaction Signature: 3now2d7Tkb2rT1RhxMR4mN8qC6fB6moGyLF8C3j1EbXpdjupRdKA5WDgddy4U7QJNmQduXJnqy3ToRg12x16GnuB
Count: 0
    ✔ Increment (697ms)


  2 passing (5s)
```

Result -> 0 
