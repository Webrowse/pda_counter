const anchor = require("@coral-xyz/anchor");
const assert = require("assert");

describe("pda_counter", () => {
  //uses ~/.config/solana/id.json to get local provider via anchor.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  //load the program from Anchor.toml and IDL
  const program = anchor.workspace.pdaCounter;

  //public key of the user running the test
  let user = provider.wallet.publicKey;

  //for storing the PDA address
  let counterPda;


// ──・❥ ────・❥ ────・❥ ────・❥ ────Test 1: Initialization────・❥ ────・❥ ────・❥ ────・❥ ────・❥ ──

  it("Initialise the counter", async () => {
    //Deriving the address using the seed and public key
    [counterPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("counte"), user.toBuffer()],
      program.programId
    );

    //calling the "initialize" instruction
    await program.methods.initialize().rpc({
      accounts: {
        counter: counterPda,
        user: user,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
    });

    //Fetch the account data to verify
    const counterAccount = await program.account.counter.fetch(counterPda);

    //assert that initial count is 0
    assert.strictEqual(counterAccount.count.toNumber(), 0);
    
    //Assert that the user is correctly stored
    assert.ok(counterAccount.user.equals(user));
    
    console.log("Initialised: count = ", counterAccount.count.toNumber());
  });


// ──・❥ ────・❥ ────・❥ ────・❥ ────Test 2: Increment────・❥ ────・❥ ────・❥ ────・❥ ────・❥ ──

  it("Increment the counter ", async () => {
    // Call the "increment" instruction
    await program.methods.increment().rpc({
      accounts: {
        counter: counterPda,
        user,
      },
    });

    //fetch updated account data
    const counterAccount = await program.account.counter.fetch(counterPda);
    
    //assert the count incremented to 1
    assert.strictEqual(counterAccount.count.toNumber(), 1);
    
    console.log("Incremented: count = ", counterAccount.count.toNumber());
  });


// ──・❥ ────・❥ ────・❥ ────・❥ ────Test 3: Reset and Close────・❥ ────・❥ ────・❥ ────・❥ ────・❥ ──

  it("Reset and close the counter", async () => {
    //calls the "reset" instruction
    await program.methods.reset().rpc({
      accounts: {
        counter: counterPda,
        user,
      },
    });

    //Trying to fetch the account to check if it is closed successfully
    let didThrow = false;
    try {
      await program.account.counter.fetch(counterPda);
    } catch (_) {
      didThrow = true;
    }

    //confirm that fetching failed (account closed successfully)
    assert.ok(didThrow);
    
    console.log("Counter Reset and Closed");
  });
});
