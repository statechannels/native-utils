const benny = require('benny')
const { hashState: jsHashState } = require('@statechannels/nitro-protocol')
const { hashState } = require('../lib')

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
      jsHashState(DEFAULT_STATE)
    }),

    benny.add('hashState (native)', () => {
      hashState(DEFAULT_STATE)
    }),

    benny.cycle(),
    benny.complete(),
  )
