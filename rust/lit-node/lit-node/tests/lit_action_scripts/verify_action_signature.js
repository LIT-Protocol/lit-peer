(async () => {
  // sign "hello world" and allow all the nodes to combine the signature and return it to the action.
  const result = await Lit.Actions.verifyActionSignature({
    signingScheme,
    actionIpfsCid,
    toSign,
    signOutput,
  });
  Lit.Actions.setResponse({ response: JSON.stringify(result.toString()) });

})();
