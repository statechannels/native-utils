import { AllocationAssetOutcome, State } from '@statechannels/nitro-protocol'
import {
  Destination,
  State as WalletCoreState,
  Uint256,
} from '@statechannels/wallet-core'
import {
  fastRecoverAddress,
  fastSignState,
} from '@statechannels/server-wallet/lib/src/utilities/signatures'
import { hashState, recoverAddress, signState } from '../lib/index'
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

const DEFAULT_WALLET_CORE_STATE: WalletCoreState = {
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
  outcome: {
    type: 'SimpleAllocation',
    assetHolderAddress: DEFAULT_STATE.outcome[0].assetHolderAddress,
    allocationItems: (DEFAULT_STATE
      .outcome[0] as AllocationAssetOutcome).allocationItems.map(ai => ({
      destination: ai.destination as Destination,
      amount: ai.amount as Uint256,
    })),
  },
  appData: DEFAULT_STATE.appData,
  appDefinition: DEFAULT_STATE.appDefinition,
}

const PRIVATE_KEY = '0x1111111111111111111111111111111111111111111111111111111111111111'

describe('Recover address', () => {
  test('Recover from signed state', async () => {
    const newSignedState = signState(DEFAULT_STATE, PRIVATE_KEY)

    // This is needed so that secp256k1 is properly initialized in
    // https://github.com/statechannels/statechannels/blob/master/packages/server-wallet/src/utilities/signatures.ts#L54
    //
    // This initialization is missing in `fastRecoverAddress` at the moment.
    const oldSignedState = await fastSignState(
      { ...DEFAULT_WALLET_CORE_STATE, stateHash: newSignedState.hash },
      PRIVATE_KEY,
    )

    // Old
    const oldAddress = fastRecoverAddress(
      DEFAULT_WALLET_CORE_STATE,
      newSignedState.signature,
      newSignedState.hash,
    )

    // New
    const newAddress = recoverAddress(DEFAULT_STATE, newSignedState.signature)

    expect(newAddress).toStrictEqual(oldAddress)
  })
})
