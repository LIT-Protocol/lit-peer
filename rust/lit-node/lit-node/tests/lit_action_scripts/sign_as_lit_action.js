(async () => {
  // sign "hello world" and allow all the nodes to combine the signature and return it to the action.
  const signature = await Lit.Actions.signAsAction({
    toSign,
    sigName,
    signingScheme,
  });
  Lit.Actions.setResponse({ response: JSON.stringify(signature) });

})();
