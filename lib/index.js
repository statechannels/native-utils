const {
  encodeOutcome,
  getChannelId,
  hashAppPart,
  hashMessage,
  hashOutcome,
  hashState,
  signState,
} = require('../native/index.node')

module.exports = {
  encodeOutcome,
  getChannelId,
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
}
