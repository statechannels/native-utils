import { State } from '@statechannels/nitro-protocol'
import {
  Destination,
  State as WalletCoreState,
  Uint256,
} from '@statechannels/wallet-core'
import { fastSignState } from '@statechannels/server-wallet/lib/src/utilities/signatures'
import { hashState, signState } from '../lib/index'
import { hashMessage } from '../lib'
import { utils } from 'ethers'
import { arrayify, concat, keccak256, toUtf8Bytes } from 'ethers/lib/utils'

const DEFAULT_STATE: State = {
  turnNum: 1,
  isFinal: false,
  channel: {
    chainId: '1',
    channelNonce: 1,
    participants: ['0x19E7E376E7C213B7E7e7e46cc70A5dD086DAff2A'],
  },
  challengeDuration: 1,
  outcome: [],
  appDefinition: '0x0000000000000000000000000000000000000000',
  appData: '0x0000000000000000000000000000000000000000000000000000000000000000',
}

const DEFAULT_WALLET_CORE_STATE: Omit<WalletCoreState, 'outcome'> = {
  turnNum: DEFAULT_STATE.turnNum,
  isFinal: DEFAULT_STATE.isFinal,
  chainId: DEFAULT_STATE.channel.chainId,
  channelNonce: DEFAULT_STATE.channel.channelNonce,
  participants: DEFAULT_STATE.channel.participants.map(p => ({
    participantId: p,
    destination: p as Destination,
    signingAddress: p,
  })),
  challengeDuration: DEFAULT_STATE.challengeDuration,
  appData: DEFAULT_STATE.appData,
  appDefinition: DEFAULT_STATE.appDefinition,
}

const PRIVATE_KEY = '0x1111111111111111111111111111111111111111111111111111111111111111'

describe('Hash message', () => {
  test('Hash a message', async () => {
    const hash = hashState(DEFAULT_STATE)

    const newHashedMessage = hashMessage(hash)
    const oldHashedMessage = utils.hashMessage(arrayify(hash))

    expect(newHashedMessage).toStrictEqual(oldHashedMessage)
  })
})

describe('Sign state', () => {
  test('State with asset allocation outcome', async () => {
    const outcome = [
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
    ]

    // New
    const state = {
      ...DEFAULT_STATE,
      outcome,
    }
    const stateHash = hashState(state)
    const newSignature = signState(state, PRIVATE_KEY).signature

    // Old
    const walletCoreState: WalletCoreState = {
      ...DEFAULT_WALLET_CORE_STATE,
      outcome: {
        type: 'SimpleAllocation',
        assetHolderAddress: outcome[0].assetHolderAddress,
        allocationItems: outcome[0].allocationItems.map(ai => ({
          destination: ai.destination as Destination,
          amount: ai.amount as Uint256,
        })),
      },
    }
    const oldSignature = (
      await fastSignState({ ...walletCoreState, stateHash }, PRIVATE_KEY)
    ).signature

    expect(newSignature).toStrictEqual(oldSignature)
  })

  test('State with guarantee outcome', async () => {
    const outcome = [
      {
        assetHolderAddress: '0x0000000000000000000000000000000000000000',
        guarantee: {
          targetChannelId:
            '0x0000000000000000000000000000000000000000000000000000000000000000',
          destinations: [
            '0x0000000000000000000000000000000000000000000000000000000000000000',
            '0x1111111111111111111111111111111111111111111111111111111111111111',
          ],
        },
      },
    ]

    // New
    const state = {
      ...DEFAULT_STATE,
      outcome,
    }
    const stateHash = hashState(state)
    const newSignature = signState(state, PRIVATE_KEY).signature

    // Old
    const walletCoreState: WalletCoreState = {
      ...DEFAULT_WALLET_CORE_STATE,
      outcome: {
        type: 'SimpleGuarantee',
        targetChannelId: outcome[0].guarantee.targetChannelId,
        assetHolderAddress: outcome[0].assetHolderAddress,
        destinations: outcome[0].guarantee.destinations,
      },
    }
    const oldSignature = (
      await fastSignState({ ...walletCoreState, stateHash }, PRIVATE_KEY)
    ).signature

    expect(newSignature).toStrictEqual(oldSignature)
  })
})
