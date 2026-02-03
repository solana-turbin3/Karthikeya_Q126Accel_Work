import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  TOKEN_2022_PROGRAM_ID,
  getAssociatedTokenAddressSync,
  createInitializeMintInstruction,
  getMintLen,
  ExtensionType,
  createTransferCheckedWithTransferHookInstruction,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createInitializeTransferHookInstruction,
  createAssociatedTokenAccountInstruction,
  createMintToInstruction,
  createTransferCheckedInstruction,
} from "@solana/spl-token";
import {
  SendTransactionError,
  SystemProgram,
  Transaction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import { WhitelistedTransfers } from "../target/types/whitelisted_transfers";

describe("whitelisted-transfers", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const wallet = provider.wallet as anchor.Wallet;

  const program = anchor.workspace
    .whitelisted_transfers as Program<WhitelistedTransfers>;

  const mint2022 = anchor.web3.Keypair.generate();

  // Sender token account address
  const sourceTokenAccount = getAssociatedTokenAddressSync(
    mint2022.publicKey,
    wallet.publicKey,
    false,
    TOKEN_2022_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

  // Recipient token account address
  const recipient = anchor.web3.Keypair.generate();
  const destinationTokenAccount = getAssociatedTokenAddressSync(
    mint2022.publicKey,
    recipient.publicKey,
    false,
    TOKEN_2022_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

  // ExtraAccountMetaList address
  // Store extra accounts required by the custom transfer hook instruction
  const [extraAccountMetaListPDA] =
    anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("extra-account-metas"), mint2022.publicKey.toBuffer()],
      program.programId,
    );

  const destination_whitelist = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("whitelist-status"), destinationTokenAccount.toBuffer()],
    program.programId,
  )[0];

  const source_whitelist = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("whitelist-status"), sourceTokenAccount.toBuffer()],
    program.programId,
  )[0];

  it("Whitelist user", async () => {
    // Whitelist the source (sender) account
    const tx1 = await program.methods
      .whitelistUser(sourceTokenAccount)
      .accountsPartial({
        admin: provider.publicKey,
        whitelistStatus: source_whitelist,
      })
      .rpc();

    // // console.log(
    //   "\nSource account added to whitelist:",
    //   wallet.publicKey.toBase58(),
    // );
    // console.log("Transaction signature:", tx1);

    // Also whitelist the destination account
    const tx2 = await program.methods
      .whitelistUser(destinationTokenAccount)
      .accountsPartial({
        admin: provider.publicKey,
        whitelistStatus: destination_whitelist,
      })
      .rpc();

    // console.log(
    //   "\nDestination account added to whitelist:",
    //   recipient.publicKey.toBase58(),
    // );
    // console.log("Transaction signature:", tx2);
  });

  // Comment this test to get the successful transfer scenario
  it("Blacklist user", async () => {
    const tx = await program.methods
      .blacklistUser (destinationTokenAccount)
      .accountsPartial({
        admin: provider.publicKey,
        whitelistStatus: destination_whitelist,
      })
      .rpc();

    console.log(
      "\nUser removed from whitelist:",
      provider.publicKey.toBase58(),
    );
    console.log("Transaction signature:", tx);
  });

  it("Create Mint Account with Transfer Hook Extension", async () => {
    const extensions = [ExtensionType.TransferHook];
    const mintLen = getMintLen(extensions);
    const lamports =
      await provider.connection.getMinimumBalanceForRentExemption(mintLen);

    const transaction = new Transaction().add(
      SystemProgram.createAccount({
        fromPubkey: wallet.publicKey,
        newAccountPubkey: mint2022.publicKey,
        space: mintLen,
        lamports: lamports,
        programId: TOKEN_2022_PROGRAM_ID,
      }),
      createInitializeTransferHookInstruction(
        mint2022.publicKey,
        wallet.publicKey,
        program.programId, // Transfer Hook Program ID
        TOKEN_2022_PROGRAM_ID,
      ),
      createInitializeMintInstruction(
        mint2022.publicKey,
        9,
        wallet.publicKey,
        null,
        TOKEN_2022_PROGRAM_ID,
      ),
    );

    const txSig = await sendAndConfirmTransaction(
      provider.connection,
      transaction,
      [wallet.payer, mint2022],
      {
        skipPreflight: true,
        commitment: "finalized",
      },
    );

    const txDetails = await program.provider.connection.getTransaction(txSig, {
      maxSupportedTransactionVersion: 0,
      commitment: "confirmed",
    });
    //console.log(txDetails.meta.logMessages);

    // console.log("\nTransaction Signature: ", txSig);
  });

  it("Create Token Accounts and Mint Tokens", async () => {
    // 100 tokens
    const amount = 100 * 10 ** 9;

    const transaction = new Transaction().add(
      createAssociatedTokenAccountInstruction(
        wallet.publicKey,
        sourceTokenAccount,
        wallet.publicKey,
        mint2022.publicKey,
        TOKEN_2022_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID,
      ),
      createAssociatedTokenAccountInstruction(
        wallet.publicKey,
        destinationTokenAccount,
        recipient.publicKey,
        mint2022.publicKey,
        TOKEN_2022_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID,
      ),
      createMintToInstruction(
        mint2022.publicKey,
        sourceTokenAccount,
        wallet.publicKey,
        amount,
        [],
        TOKEN_2022_PROGRAM_ID,
      ),
    );

    const txSig = await sendAndConfirmTransaction(
      provider.connection,
      transaction,
      [wallet.payer],
      { skipPreflight: true },
    );

    // console.log("\nTransaction Signature: ", txSig);
  });

  // Account to store extra accounts required by the transfer hook instruction
  it("Create ExtraAccountMetaList Account", async () => {
    const initializeExtraAccountMetaListInstruction = await program.methods
      .initTransferHook()
      .accountsPartial({
        signer: wallet.publicKey,
        mint: mint2022.publicKey,
        extraAccMetaList: extraAccountMetaListPDA,
        systemProgram: SystemProgram.programId,
      })
      //.instruction();
      .rpc();

    //const transaction = new Transaction().add(initializeExtraAccountMetaListInstruction);

    //const txSig = await sendAndConfirmTransaction(provider.connection, transaction, [wallet.payer], { skipPreflight: true, commitment: 'confirmed' });
    // console.log(
    //   "\nExtraAccountMetaList Account created:",
    //   extraAccountMetaListPDA.toBase58(),
    // );
    // console.log(
    //   "Transaction Signature:",
    //   initializeExtraAccountMetaListInstruction,
    // );
  });

  it("Transfer Hook with Manual Extra Accounts", async () => {
    const amount = 1 * 10 ** 9;
    const amountBigInt = BigInt(amount);

    // Create the base transfer instruction
    const transferInstruction = createTransferCheckedInstruction(
      sourceTokenAccount,
      mint2022.publicKey,
      destinationTokenAccount,
      wallet.publicKey,
      amountBigInt,
      9,
      [],
      TOKEN_2022_PROGRAM_ID,
    );

    // Add the extra accounts in the correct order
    transferInstruction.keys.push(
      {
        pubkey: extraAccountMetaListPDA,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: source_whitelist,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: destination_whitelist,
        isSigner: false,
        isWritable: false,
      },
      {
        pubkey: program.programId,
        isSigner: false,
        isWritable: false,
      },
    );

    const transaction = new Transaction().add(transferInstruction);

    try {
      const txSig = await sendAndConfirmTransaction(
        provider.connection,
        transaction,
        [wallet.payer],
        { skipPreflight: true },
      );
      // console.log("\n✅ Transfer Signature:", txSig);
    } catch (error) {
      if (error instanceof SendTransactionError) {
        console.error("\n❌ Transaction failed. Full logs:");
        error.logs?.forEach((log, i) => console.error(`  ${i}: ${log}`));
      } else {
        console.error("\n❌ Unexpected error:", error);
      }
    }
  });
});
