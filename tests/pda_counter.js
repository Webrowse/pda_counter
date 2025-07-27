const anchor = require("@coral-xyz/anchor");
const assert = require("assert");

describe("pda_counter", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.pdaCounter;

  let user = provider.wallet.publicKey;
  let counterPda;

  it("Initialise the counter", async () => {
    [counterPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("counte"), user.toBuffer()],
      program.programId
    );

    await program.methods.initialize().rpc({
      accounts: {
        counter: counterPda,
        user: user,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
    });

    const counterAccount = await program.account.counter.fetch(counterPda);
    assert.strictEqual(counterAccount.count.toNumber(), 0);
    assert.ok(counterAccount.user.equals(user));
    console.log("Initialised: count = ", counterAccount.count.toNumber());
  });

  it("Increment the counter ", async () => {
    await program.methods.increment().rpc({
      accounts: {
        counter: counterPda,
        user,
      },
    });

    const counterAccount = await program.account.counter.fetch(counterPda);
    assert.strictEqual(counterAccount.count.toNumber(), 1);
    console.log("Incremented: count = ", counterAccount.count.toNumber());
  });

  it("Reset and close the counter", async () => {
    await program.methods.reset().rpc({
      accounts: {
        counter: counterPda,
        user,
      },
    });

    let didThrow = false;
    try {
      await program.account.counter.fetch(counterPda);
    } catch (_) {
      didThrow = true;
    }
    assert.ok(didThrow);
    console.log("Counter Reset and Closed");
  });
});
