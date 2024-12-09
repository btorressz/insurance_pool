describe("Insurance Pool Tests", () => {
  // Generate keypairs for test accounts
  const reservePoolKp = new web3.Keypair();
  const policyKp = new web3.Keypair();
  const userPublicKey = pg.wallet.publicKey; // Default wallet public key

  it("Initializes the Reserve Pool", async () => {
    const txHash = await pg.program.methods
      .initializePool()
      .accounts({
        reservePool: reservePoolKp.publicKey,
        user: userPublicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([reservePoolKp])
      .rpc();
    console.log(`Reserve pool initialized with tx: ${txHash}`);

    // Confirm transaction
    await pg.connection.confirmTransaction(txHash);

    // Fetch and verify reserve pool data
    const reservePool = await pg.program.account.reservePool.fetch(
      reservePoolKp.publicKey
    );

    // Assertions using the built-in assert module
    assert.strictEqual(reservePool.lowRiskReserves, 0, "Low risk reserves should be 0");
    assert.strictEqual(reservePool.mediumRiskReserves, 0, "Medium risk reserves should be 0");
    assert.strictEqual(reservePool.highRiskReserves, 0, "High risk reserves should be 0");
    assert.strictEqual(reservePool.totalPolicies, 0, "Total policies should be 0");
    assert.strictEqual(
      reservePool.isPaused,
      false,
      "Reserve pool should not be paused"
    );
    assert.strictEqual(
      reservePool.governance.toString(),
      userPublicKey.toString(),
      "Governance key should match the user public key"
    );
  });

  it("Purchases Insurance", async () => {
    const policyType = { flightDelay: {} }; // PolicyType enum
    const coverageLevel = { basic: {} }; // CoverageLevel enum
    const riskLevel = { low: {} }; // RiskLevel enum
    const baseRate = new BN(100);
    const duration = new BN(30 * 24 * 60 * 60); // 30 days in seconds

    const txHash = await pg.program.methods
      .purchaseInsurance(baseRate, policyType, coverageLevel, duration, riskLevel, null)
      .accounts({
        reservePool: reservePoolKp.publicKey,
        policy: policyKp.publicKey,
        user: userPublicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([policyKp])
      .rpc();
    console.log(`Insurance purchased with tx: ${txHash}`);

    // Confirm transaction
    await pg.connection.confirmTransaction(txHash);

    // Fetch and verify policy data
    const policy = await pg.program.account.policy.fetch(policyKp.publicKey);

    // Assertions using the built-in assert module
    assert.strictEqual(
      policy.user.toString(),
      userPublicKey.toString(),
      "Policy user should match the signer"
    );
    assert.deepEqual(policy.policyType, policyType, "Policy type should match");
    assert.deepEqual(policy.coverage, coverageLevel, "Coverage level should match");
    assert.ok(policy.expiryTime.toNumber() > 0, "Expiry time should be greater than 0");
    assert.strictEqual(policy.referrer, null, "Referrer should be null");
  });
});
