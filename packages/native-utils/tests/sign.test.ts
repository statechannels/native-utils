import { utils } from 'ethers'
import { arrayify } from 'ethers/lib/utils'
import { State } from '@statechannels/nitro-protocol'
import * as wasm from '@statechannels/wasm-utils'
import * as native from '..'
import * as nitro from '@statechannels/nitro-protocol'

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

const PRIVATE_KEY = '0x1111111111111111111111111111111111111111111111111111111111111111'

describe('Hash message', () => {
  test('Hash a message', async () => {
    const hash = native.hashState(DEFAULT_STATE)

    const oldHashedMessage = utils.hashMessage(arrayify(hash))
    const nativeHashedMessage = native.hashMessage(hash)
    const wasmHashedMessage = wasm.hashMessage(hash)

    expect(nativeHashedMessage).toStrictEqual(oldHashedMessage)
    expect(wasmHashedMessage).toStrictEqual(oldHashedMessage)
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

    const state = {
      ...DEFAULT_STATE,
      outcome,
    }
    const oldSignature = utils.joinSignature(
      (await nitro.signState(state, PRIVATE_KEY)).signature,
    )

    // Native
    const nativeSignature = native.signState(state, PRIVATE_KEY).signature

    // WASM
    const wasmSignature = wasm.signState(state, PRIVATE_KEY).signature

    expect(nativeSignature).toStrictEqual(oldSignature)
    expect(wasmSignature).toStrictEqual(oldSignature)
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

    const state = {
      ...DEFAULT_STATE,
      outcome,
    }

    const oldSignature = utils.joinSignature(
      (await nitro.signState(state, PRIVATE_KEY)).signature,
    )

    // Native
    const nativeSignature = native.signState(state, PRIVATE_KEY).signature

    // WASM
    const wasmSignature = wasm.signState(state, PRIVATE_KEY).signature

    expect(nativeSignature).toStrictEqual(oldSignature)
    expect(wasmSignature).toStrictEqual(oldSignature)
  })

  test('Catches invalid private key', async () => {
    // Invalid signature length
    expect(() => native.signState(DEFAULT_STATE, '0x00')).toThrow('invalid private key')
    expect(() => wasm.signState(DEFAULT_STATE, '0x00')).toThrow('invalid private key')
  })
})
