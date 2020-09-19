const { utils } = require('ethers')
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
