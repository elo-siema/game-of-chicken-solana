const anchor = require('@project-serum/anchor');
import {Provider, web3, setProvider, Wallet, BN, workspace} from '@project-serum/anchor';
const assert = require("assert");
const { SystemProgram } = web3;
const spl = require('@solana/spl-token');

describe('chicken', () => {

  // Configure the client to use the local cluster.
  const provider = Provider.env();
  setProvider(provider);

  const program = workspace.Chicken;

  it('Is initialized!', async () => {
    // Add your test here.
      const gameDataAccount = web3.Keypair.generate();
      const tx = await program.rpc.initialize(
        new BN(2635114651), 
        new BN(2635214651), 
        new BN(1), 
        {
        accounts: {
          gameDataAccount: gameDataAccount.publicKey,
          player: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
          tokenProgram: spl.TOKEN_PROGRAM_ID
        },
        signers: [gameDataAccount]
      });

    console.log("Your transaction signature", tx);
  });
  /*
  it('Throws error on start date in the past!', async () => {
    try {
      const gameDataAccount = anchor.web3.Keypair.generate();
      const tx = await program.rpc.initialize(
        new anchor.BN(1635014651), 
        new anchor.BN(2635014651), 
        new anchor.BN(0), 
        {
        accounts: {
          gameDataAccount: gameDataAccount.publicKey,
          player1: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
          tokenProgram: spl.TOKEN_PROGRAM_ID
        },
        signers: [gameDataAccount],
      });
    } catch (ex) {
      console.log(ex);
      assert.ok(true);
    }
  });

  it('Throws error on end date in the past!', async () => {
    try {
      const gameDataAccount = anchor.web3.Keypair.generate();
      const tx = await program.rpc.initialize(
        new anchor.BN(2635014651), 
        new anchor.BN(1635014651), 
        new anchor.BN(0), 
        {
        accounts: {
          gameDataAccount: gameDataAccount.publicKey,
          player1: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
          tokenProgram: spl.TOKEN_PROGRAM_ID
        },
        signers: [gameDataAccount],
      });
    } catch (ex) {
      console.log(ex);
      assert.ok(true);
    }
  });

  it('Throws error on start date after end date!', async () => {
    try {
      const gameDataAccount = anchor.web3.Keypair.generate();
      const tx = await program.rpc.initialize(
        new anchor.BN(2635014651), 
        new anchor.BN(2635014650), 
        new anchor.BN(0), 
        {
        accounts: {
          gameDataAccount: gameDataAccount.publicKey,
          player1: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
          tokenProgram: spl.TOKEN_PROGRAM_ID
        },
        signers: [gameDataAccount],
      });
    } catch (ex) {
      console.log(ex);
      assert.ok(true);
    }
  });*/
});
