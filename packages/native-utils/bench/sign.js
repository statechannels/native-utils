const benny = require('benny')
const wasm = require('@statechannels/wasm-utils')
const native = require('..')

const PRIVATE_KEY = '0x1111111111111111111111111111111111111111111111111111111111111111'

const OUTCOME = [
  {
    asset: '0x0000000000000000000000000000000000000000',
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

module.exports = () =>
  benny.suite(
    'State signing',

    benny.add('signState (native)', () => {
      native.signState(DEFAULT_STATE, PRIVATE_KEY)
    }),

    benny.add('signState (wasm)', () => {
      wasm.signState(DEFAULT_STATE, PRIVATE_KEY)
    }),

    benny.cycle(),
    benny.complete(),
  )
