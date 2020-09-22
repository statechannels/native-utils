const {
  getChannelId,

  encodeOutcome,

  hashAppPart,
  hashMessage,
  hashOutcome,
  hashState,

  signState,
  recoverAddress,
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
}
