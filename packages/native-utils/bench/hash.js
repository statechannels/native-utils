const benny = require('benny')
const nitro = require('@statechannels/nitro-protocol')
const wasm = require('@statechannels/wasm-utils')
const native = require('..')

const DEFAULT_STATE = {
  turnNum: 1,
  isFinal: false,
  channel: {
    chainId: '1',
    channelNonce: 1,
    participants: [],
  },
  challengeDuration: 1,
  outcome: [
    {
      assetHolderAddress: '0x0000000000000000000000000000000000000000',
      allocationItems: [
        {
          destination:
            '0x0000000000000000000000000000000000000000000000000000000000000000',
          amount: '1',
        },
      ],
    },
  ],
  appDefinition: '0x0000000000000000000000000000000000000000',
  appData: '0x0000000000000000000000000000000000000000000000000000000000000000',
}

module.exports = () =>
  benny.suite(
    'State hashing',

    benny.add('hashState (js)', () => {
      nitro.hashState(DEFAULT_STATE)
    }),

    benny.add('hashState (native)', () => {
      native.hashState(DEFAULT_STATE)
    }),

    benny.add('hashState (wasm)', () => {
      wasm.hashState(DEFAULT_STATE)
    }),

    benny.cycle(),
    benny.complete(),
  )
