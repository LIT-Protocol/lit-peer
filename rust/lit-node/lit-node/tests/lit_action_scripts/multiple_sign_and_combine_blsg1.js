(async () => {
  const sig1 = await Lit.Actions.signAndCombine({
    toSign: 'test message 1',
    publicKey,
    sigName: 'sig1',
    signingScheme: 'Bls12381G1ProofOfPossession',
  });

  const sig2 = await Lit.Actions.signAndCombine({
    toSign: 'test message 2',
    publicKey,
    sigName: 'sig2',
    signingScheme: 'Bls12381G1ProofOfPossession',
  });

  const sigs = {
    sig1: JSON.parse(sig1),
    sig2: JSON.parse(sig2),
  };

  Lit.Actions.setResponse({ response: JSON.stringify(sigs) });
})();
