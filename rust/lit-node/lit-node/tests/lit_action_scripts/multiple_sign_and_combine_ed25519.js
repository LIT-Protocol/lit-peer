(async () => {
  const sig1 = await Lit.Actions.signAndCombine({
    toSign: 'test message 1',
    publicKey,
    sigName: 'sig1',
    signingScheme: 'SchnorrEd25519Sha512',
  });

  const sig2 = await Lit.Actions.signAndCombine({
    toSign: 'test message 2',
    publicKey,
    sigName: 'sig2',
    signingScheme: 'SchnorrEd25519Sha512',
  });

  const sigs = {
    sig1: JSON.parse(sig1),
    sig2: JSON.parse(sig2),
  };

  Lit.Actions.setResponse({ response: JSON.stringify(sigs) });
})();
