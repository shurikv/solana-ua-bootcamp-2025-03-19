import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { Favorites } from "../target/types/favorites";
import { airdropIfRequired, getCustomErrorMessage } from "@solana-developers/helpers";
import { expect, describe } from '@jest/globals';
import { systemProgramErrors } from "./system-program-errors";

describe("favorites", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  it("Writes our favorites to the blockchain!", async () => {
    const user = web3.Keypair.generate();
    const program = anchor.workspace.Favorites as Program<Favorites>;

    console.log(`User public key: ${user.publicKey}`);

    await airdropIfRequired(
        anchor.getProvider().connection,
        user.publicKey,
        0.5 * web3.LAMPORTS_PER_SOL,
        0.5 * web3.LAMPORTS_PER_SOL
    );

// Here's what we want to write to the blockchain
    const favoriteNumber = new anchor.BN(23);
    const favoriteColor = "red";
// Make a transaction to write to the blockchain
    let tx: string | null = null;
    try {
      tx = await program.methods
          // Call the set_favorites instruction handler
          .setFavorites(favoriteNumber, favoriteColor)
          .accounts({
            user: user.publicKey,
            // Note that both `favorites` and `system_program` are added
            // automatically.
          })
          // Sign the transaction
          .signers([user])
          // Send the transaction to the cluster or RPC
          .rpc();
    } catch (thrownObject) {
      // Let's properly log the error, so we can see the program involved
      // and (for well known programs) the full log message.

      const rawError = thrownObject as Error;
      throw new Error(getCustomErrorMessage(systemProgramErrors, rawError.message));
    }

    console.log(`Tx signature: ${tx}`);

    // Calculate the PDA account address that holds the user's favorites
    const [favoritesPda, _favoritesBump] = web3.PublicKey.findProgramAddressSync(
        [Buffer.from("favorites"), user.publicKey.toBuffer()],
        program.programId
    );

    // And make sure it matches!
    const dataFromPda = await program.account.favorites.fetch(favoritesPda);
    expect(dataFromPda.color).toEqual(favoriteColor);
    expect(dataFromPda.number.toNumber()).toEqual(favoriteNumber.toNumber());

  });

  it("Update our favorites in the blockchain!", async () => {
    const user = web3.Keypair.generate();
    const program = anchor.workspace.Favorites as Program<Favorites>;

    console.log(`User public key: ${user.publicKey}`);

    await airdropIfRequired(
        anchor.getProvider().connection,
        user.publicKey,
        0.5 * web3.LAMPORTS_PER_SOL,
        0.5 * web3.LAMPORTS_PER_SOL
    );

// Here's what we want to write to the blockchain
    const favoriteNumber = new anchor.BN(23);
    const favoriteColor = "red";
// Make a transaction to write to the blockchain
    let tx: string | null = null;
    try {
      tx = await program.methods
          // Call the set_favorites instruction handler
          .setFavorites(favoriteNumber, favoriteColor)
          .accounts({
            user: user.publicKey,
            // Note that both `favorites` and `system_program` are added
            // automatically.
          })
          // Sign the transaction
          .signers([user])
          // Send the transaction to the cluster or RPC
          .rpc();
    } catch (thrownObject) {
      // Let's properly log the error, so we can see the program involved
      // and (for well known programs) the full log message.

      const rawError = thrownObject as Error;
      throw new Error(getCustomErrorMessage(systemProgramErrors, rawError.message));
    }

    console.log(`Tx signature: ${tx}`);

    // Calculate the PDA account address that holds the user's favorites
    const [favoritesPda, _favoritesBump] = web3.PublicKey.findProgramAddressSync(
        [Buffer.from("favorites"), user.publicKey.toBuffer()],
        program.programId
    );

    // And make sure it matches!
    const dataFromPda = await program.account.favorites.fetch(favoritesPda);
    expect(dataFromPda.color).toEqual(favoriteColor);
    expect(dataFromPda.number.toNumber()).toEqual(favoriteNumber.toNumber());

    const upFavoriteNumber = new anchor.BN(57);
    const upFavoriteColor = "blue";
    try {
      tx = await program.methods
            // Call the set_favorites instruction handler
            .updateFavorites(upFavoriteNumber, upFavoriteColor)
            .accounts({
                user: user.publicKey,
                // Note that both `favorites` and `system_program` are added
                // automatically.
            })
            // Sign the transaction
            .signers([user])
            // Send the transaction to the cluster or RPC
            .rpc();
    } catch (thrownObject) {
      // Let's properly log the error, so we can see the program involved
      // and (for well known programs) the full log message.

      const rawError = thrownObject as Error;
      throw new Error(getCustomErrorMessage(systemProgramErrors, rawError.message));
    }

    console.log(`Tx signature: ${tx}`);

    // Calculate the PDA account address that holds the user's favorites
    const [favoritesPda1, _favoritesBump1] = web3.PublicKey.findProgramAddressSync(
        [Buffer.from("favorites"), user.publicKey.toBuffer()],
        program.programId
    );

    // And make sure it matches!
    const dataFromPda1 = await program.account.favorites.fetch(favoritesPda1);
    expect(dataFromPda1.color).toEqual(upFavoriteColor);
    expect(dataFromPda1.number.toNumber()).toEqual(upFavoriteNumber.toNumber());
  });

  it("Update only number in favorites in the blockchain!", async () => {
    const user = web3.Keypair.generate();
    const program = anchor.workspace.Favorites as Program<Favorites>;

    console.log(`User public key: ${user.publicKey}`);

    await airdropIfRequired(
        anchor.getProvider().connection,
        user.publicKey,
        0.5 * web3.LAMPORTS_PER_SOL,
        0.5 * web3.LAMPORTS_PER_SOL
    );

// Here's what we want to write to the blockchain
    const favoriteNumber = new anchor.BN(7);
    const favoriteColor = "blue";
// Make a transaction to write to the blockchain
    let tx: string | null = null;
    try {
      tx = await program.methods
          // Call the set_favorites instruction handler
          .setFavorites(favoriteNumber, favoriteColor)
          .accounts({
            user: user.publicKey,
            // Note that both `favorites` and `system_program` are added
            // automatically.
          })
          // Sign the transaction
          .signers([user])
          // Send the transaction to the cluster or RPC
          .rpc();
    } catch (thrownObject) {
      // Let's properly log the error, so we can see the program involved
      // and (for well known programs) the full log message.

      const rawError = thrownObject as Error;
      throw new Error(getCustomErrorMessage(systemProgramErrors, rawError.message));
    }

    console.log(`Tx signature: ${tx}`);

    // Calculate the PDA account address that holds the user's favorites
    const [favoritesPda, _favoritesBump] = web3.PublicKey.findProgramAddressSync(
        [Buffer.from("favorites"), user.publicKey.toBuffer()],
        program.programId
    );

    // And make sure it matches!
    const dataFromPda = await program.account.favorites.fetch(favoritesPda);
    expect(dataFromPda.color).toEqual(favoriteColor);
    expect(dataFromPda.number.toNumber()).toEqual(favoriteNumber.toNumber());

    const upFavoriteNumber = new anchor.BN(109);
    try {
      tx = await program.methods
          // Call the set_favorites instruction handler
          .updateFavorites(upFavoriteNumber, null)
          .accounts({
            user: user.publicKey,
            // Note that both `favorites` and `system_program` are added
            // automatically.
          })
          // Sign the transaction
          .signers([user])
          // Send the transaction to the cluster or RPC
          .rpc();
    } catch (thrownObject) {
      // Let's properly log the error, so we can see the program involved
      // and (for well known programs) the full log message.

      const rawError = thrownObject as Error;
      throw new Error(getCustomErrorMessage(systemProgramErrors, rawError.message));
    }

    console.log(`Tx signature: ${tx}`);

    // Calculate the PDA account address that holds the user's favorites
    const [favoritesPda1, _favoritesBump1] = web3.PublicKey.findProgramAddressSync(
        [Buffer.from("favorites"), user.publicKey.toBuffer()],
        program.programId
    );

    // And make sure it matches!
    const dataFromPda1 = await program.account.favorites.fetch(favoritesPda1);
    expect(dataFromPda1.color).toEqual(favoriteColor);
    expect(dataFromPda1.number.toNumber()).toEqual(upFavoriteNumber.toNumber());
  });


  it("Update only color in favorites in the blockchain!", async () => {
    const user = web3.Keypair.generate();
    const program = anchor.workspace.Favorites as Program<Favorites>;

    console.log(`User public key: ${user.publicKey}`);

    await airdropIfRequired(
        anchor.getProvider().connection,
        user.publicKey,
        0.5 * web3.LAMPORTS_PER_SOL,
        0.5 * web3.LAMPORTS_PER_SOL
    );

// Here's what we want to write to the blockchain
    const favoriteNumber = new anchor.BN(7);
    const favoriteColor = "blue";
// Make a transaction to write to the blockchain
    let tx: string | null = null;
    try {
      tx = await program.methods
          // Call the set_favorites instruction handler
          .setFavorites(favoriteNumber, favoriteColor)
          .accounts({
            user: user.publicKey,
            // Note that both `favorites` and `system_program` are added
            // automatically.
          })
          // Sign the transaction
          .signers([user])
          // Send the transaction to the cluster or RPC
          .rpc();
    } catch (thrownObject) {
      // Let's properly log the error, so we can see the program involved
      // and (for well known programs) the full log message.

      const rawError = thrownObject as Error;
      throw new Error(getCustomErrorMessage(systemProgramErrors, rawError.message));
    }

    console.log(`Tx signature: ${tx}`);

    // Calculate the PDA account address that holds the user's favorites
    const [favoritesPda, _favoritesBump] = web3.PublicKey.findProgramAddressSync(
        [Buffer.from("favorites"), user.publicKey.toBuffer()],
        program.programId
    );

    // And make sure it matches!
    const dataFromPda = await program.account.favorites.fetch(favoritesPda);
    expect(dataFromPda.color).toEqual(favoriteColor);
    expect(dataFromPda.number.toNumber()).toEqual(favoriteNumber.toNumber());

    const upFavoriteColor = "green";
    try {
      tx = await program.methods
          // Call the set_favorites instruction handler
          .updateFavorites(null, upFavoriteColor)
          .accounts({
            user: user.publicKey,
            // Note that both `favorites` and `system_program` are added
            // automatically.
          })
          // Sign the transaction
          .signers([user])
          // Send the transaction to the cluster or RPC
          .rpc();
    } catch (thrownObject) {
      // Let's properly log the error, so we can see the program involved
      // and (for well known programs) the full log message.

      const rawError = thrownObject as Error;
      throw new Error(getCustomErrorMessage(systemProgramErrors, rawError.message));
    }

    console.log(`Tx signature: ${tx}`);

    // Calculate the PDA account address that holds the user's favorites
    const [favoritesPda1, _favoritesBump1] = web3.PublicKey.findProgramAddressSync(
        [Buffer.from("favorites"), user.publicKey.toBuffer()],
        program.programId
    );

    // And make sure it matches!
    const dataFromPda1 = await program.account.favorites.fetch(favoritesPda1);
    expect(dataFromPda1.color).toEqual(upFavoriteColor);
    expect(dataFromPda1.number.toNumber()).toEqual(favoriteNumber.toNumber());
  });
});
