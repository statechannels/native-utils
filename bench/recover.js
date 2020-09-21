const benny = require('benny')
const serverWallet = require('@statechannels/server-wallet/lib/src/utilities/signatures')
const native = require('../lib')
const nitro = require('@statechannels/nitro-protocol')
const wasm = require('../wasm/pkg')

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

const WALLET_CORE_DEFAULT_STATE = {
  turnNum: DEFAULT_STATE.turnNum,
  isFinal: DEFAULT_STATE.isFinal,
  chainId: DEFAULT_STATE.channel.chainId,
  channelNonce: DEFAULT_STATE.channel.channelNonce,
  participants: DEFAULT_STATE.channel.participants.map(p => ({
    participantId: p,
    destination: p,
    signingAddress: p,
  })),
  challengeDuration: DEFAULT_STATE.challengeDuration,
  outcome: {
    type: 'SimpleAllocation',
    assetHolderAddress: OUTCOME[0].assetHolderAddress,
    allocationItems: OUTCOME[0].allocationItems.map(ai => ({
      destination: ai.destination,
      amount: ai.amount,
    })),
  },
  appData: DEFAULT_STATE.appData,
  appDefinition: DEFAULT_STATE.appDefinition,
}

module.exports = async () => {
  const stateHash = nitro.hashState(DEFAULT_STATE)

  const oldSignedState = await serverWallet.fastSignState(
    {
      ...WALLET_CORE_DEFAULT_STATE,
      stateHash,
    },
    PRIVATE_KEY,
  )

  const newSignedState = native.signState(DEFAULT_STATE, PRIVATE_KEY)

  return benny.suite(
    'Recover address',

    benny.add('fastRecoverAddress (nitro, wasm)', async () => {
      // We include the hashing here again, because `recoverAddress` does it internally;
      // it wouldn't be fair to hash the state outside this benchmark
      const stateHash = nitro.hashState(DEFAULT_STATE)

      serverWallet.fastRecoverAddress(
        WALLET_CORE_DEFAULT_STATE,
        oldSignedState.signature,
        stateHash,
      )
    }),

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
