(async () => {
  // sign "hello world" and allow all the nodes to combine the signature and return it to the action.
  const publicKey = await Lit.Actions.getActionPublicKey({
    signingScheme,
    actionIpfsCid,
  });
  Lit.Actions.setResponse({ response: JSON.stringify(publicKey) });

})();
