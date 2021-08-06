const anchor = require('@project-serum/anchor');

describe('events-test', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  it('Is initialized!', async () => {
    // Add your test here.
    const program = anchor.workspace.EventsTest;
    let stoplossState = anchor.web3.Keypair.generate();

    const tx = new anchor.web3.Transaction();
    tx.add(
      anchor.web3.SystemProgram.createAccount({
        fromPubkey: program.provider.wallet.publicKey,
        newAccountPubkey: stoplossState.publicKey,
        space: program.account.stoplossState._size,
        lamports: await program.provider.connection.getMinimumBalanceForRentExemption(
          program.account.stoplossState._size
        ),
        programId: program.programId,
      })
    );
  
    await program.provider.send(tx, [stoplossState]);

    let accounts = {
      stoplossState: stoplossState.publicKey,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    };
    let ownAddress = stoplossState.publicKey;
    const tx2 = await program.rpc.initialize(ownAddress, {accounts:accounts});

    console.log("Your transaction signature", tx2);
  });
});
