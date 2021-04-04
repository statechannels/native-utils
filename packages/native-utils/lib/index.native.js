const {
  getChannelId,

  encodeOutcome,

  hashAppPart,
  hashMessage,
  hashOutcome,
  hashState,

  signState,
  recoverAddress,
  verifySignature,
} = require('../native/index.node')

function unwrapResult({ Ok, Err }) {
  if (Err) {
    throw Err
  } else {
    return Ok
  }
}

module.exports = {
  getChannelId,

  encodeOutcome,

  hashAppPart,
  hashMessage,
  hashOutcome,
  hashState,

  signState: (state, privateKey) => {
    const { hash, signature } = unwrapResult(signState(state, privateKey))
    return {
      state,
      hash,
      signature,
    }
  },

  recoverAddress: (state, signature) => unwrapResult(recoverAddress(state, signature)),

  verifySignature: (state, signature) => unwrapResult(verifySignature(state, signature)),
}
