
const anchor = require("@project-serum/anchor");
const idl = require('./idl.json');

async function run() {
    anchor.setProvider(anchor.Provider.local());

    const program = new anchor.Program(idl, "Fvk1gv6DmGbnwwafWPj1LqSRiCoVhqLa2TzRrProugWP", anchor.provider);

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

    let res = await program.provider.send(tx, [stoplossState]);
    console.log("result", res);


    let accounts = {
        stoplossState: stoplossState.publicKey,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    };
    let ownAddress = stoplossState.publicKey;
    const tx2 = await program.rpc.initialize(ownAddress, { accounts: accounts });

    console.log("initialized", tx2);
}

run();