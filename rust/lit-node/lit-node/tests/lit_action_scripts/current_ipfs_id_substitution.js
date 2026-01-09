(async () => {
  const resp = await Lit.Actions.decryptAndCombine({
    accessControlConditions,
    ciphertext,
    dataToEncryptHash,
    authSig: null,
    chain: 'ethereum',
    keySetId,
  });
  Lit.Actions.setResponse({ response: JSON.stringify(resp) });
})();
