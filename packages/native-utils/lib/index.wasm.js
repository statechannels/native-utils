const {
  getChannelId,

  encodeOutcome,

  hashAppPart,
  hashMessage,
  hashOutcome,
  hashState,

  signState,
  recoverAddress,
} = require('@statechannels/wasm-utils')

module.exports = {
  getChannelId,

  encodeOutcome,

  hashAppPart,
  hashMessage,
  hashOutcome,
  hashState,

  signState: (state, privateKey) => {
    const { hash, signature } = signState(state, privateKey)
    return {
      state,
      hash,
      signature,
    }
  },

  recoverAddress,
}