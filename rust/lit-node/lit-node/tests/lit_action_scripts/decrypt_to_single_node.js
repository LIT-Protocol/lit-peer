(async () => {

  const resp = await Lit.Actions.decryptToSingleNode({
    accessControlConditions,
    ciphertext,
    dataToEncryptHash,
    authSig,
    chain: 'ethereum',
    keySetId,
  });
  Lit.Actions.setResponse({ response: JSON.stringify(resp) });
})();
