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
  validatePeerUpdate,
  getMyState,
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

  verifySignature: (hash, address, signature) => unwrapResult(verifySignature(hash, address, signature)),

  validatePeerUpdate: (state, peer_update, signature) => unwrapResult(validatePeerUpdate(state, peer_update, signature)),

  getMyState: (last_state) => unwrapResult(getMyState(last_state)),
}
