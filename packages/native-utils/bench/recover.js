const benny = require('benny')
const wasm = require('@statechannels/wasm-utils')
const native = require('..')

const PRIVATE_KEY = '0x1111111111111111111111111111111111111111111111111111111111111111'

const OUTCOME = [
  {
    assetHolderAddress: '0x0000000000000000000000000000000000000000',
    allocationItems: [
      {
        destination: '0x0000000000000000000000000000000000000000000000000000000000000000',
        amount: '1',
      },
    ],
  },
]

const DEFAULT_STATE = {
  turnNum: 1,
  isFinal: false,
  channel: {
    chainId: '1',
    channelNonce: 1,
    participants: ['0x19E7E376E7C213B7E7e7e46cc70A5dD086DAff2A'],
  },
  challengeDuration: 1,
  outcome: OUTCOME,
  appDefinition: '0x0000000000000000000000000000000000000000',
  appData: '0x0000000000000000000000000000000000000000000000000000000000000000',
}

module.exports = async () => {
  const newSignedState = native.signState(DEFAULT_STATE, PRIVATE_KEY)

  return benny.suite(
    'Recover address',

    benny.add('recoverAddress (native)', () => {
      native.recoverAddress(newSignedState.state, newSignedState.signature)
    }),

    benny.add('recoverAddress (wasm)', () => {
      wasm.recoverAddress(newSignedState.state, newSignedState.signature)
    }),

    benny.cycle(),
    benny.complete(),
  )
}
