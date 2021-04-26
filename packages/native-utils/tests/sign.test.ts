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
    participants: ['0x63FaC9201494f0bd17B9892B9fae4d52fe3BD377', '0x90F8bf6A479f320ead074411a4B0e7944Ea8c9C1'],
  },
  challengeDuration: 1,
  outcome: [],
  appDefinition: '0x0000000000000000000000000000000000000000',
  appData: '0x0000000000000000000000000000000000000000000000000000000000000000',
}

const PRIVATE_KEY1 = '0x8da4ef21b864d2cc526dbdb2a120bd2874c36c9d0a1fb7f8c63d7f7a8b41de8f'
const PRIVATE_KEY2 = '0x4f3edf983ac636a65a842ce7c78d9aa706d3b113bce9c46f30d7d21715b23b1d'

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
      (await nitro.signState(state, PRIVATE_KEY1)).signature,
    )

    // Native
    const nativeSignature = native.signState(state, PRIVATE_KEY1).signature

    // WASM
    const wasmSignature = wasm.signState(state, PRIVATE_KEY1).signature

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
      (await nitro.signState(state, PRIVATE_KEY1)).signature,
    )

    // Native
    const nativeSigned = native.signState(state, PRIVATE_KEY1)

    // WASM
    const wasmSigned = wasm.signState(state, PRIVATE_KEY1)

    expect(nativeSigned.signature).toStrictEqual(oldSignature)
    expect(wasmSigned.signature).toStrictEqual(oldSignature)
  })

  test('Peer states validate as expected', async () => {
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

    const currentState = {
      ...DEFAULT_STATE,
      outcome,
    }

    const peerState = {
      ...DEFAULT_STATE,
      outcome,
    }

    // First 2 updates
    currentState.turnNum = 1;
    peerState.turnNum = 2;

    const nativeSigned1 = native.signState(peerState, PRIVATE_KEY2);
    expect(native.validatePeerUpdate(currentState, peerState, nativeSigned1.signature)).toEqual("True")
    expect(wasm.validatePeerUpdate(currentState, peerState, nativeSigned1.signature)).toEqual("True")

    // Additional updates
    currentState.turnNum = 3;
    peerState.turnNum = 4;

    const nativeSigned2 = native.signState(peerState, PRIVATE_KEY2);
    expect(native.validatePeerUpdate(currentState, peerState, nativeSigned2.signature)).toEqual("NeedToCheckApp")
    expect(wasm.validatePeerUpdate(currentState, peerState, nativeSigned2.signature)).toEqual("NeedToCheckApp")

    // Signer mismatch
    const nativeSigned3 = native.signState(peerState, PRIVATE_KEY1);
    expect(() => native.validatePeerUpdate(currentState, peerState, nativeSigned3.signature)).toThrow('Signature verification failed');
    expect(() => wasm.validatePeerUpdate(currentState, peerState, nativeSigned3.signature)).toThrow('Signature verification failed');

    // turn number
    currentState.turnNum = 2;
    peerState.turnNum = 4;
    const nativeSigned4 = native.signState(peerState, PRIVATE_KEY2);
    expect(() => native.validatePeerUpdate(currentState, peerState, nativeSigned4.signature)).toThrow('turnNum must increment by one');
    expect(() => wasm.validatePeerUpdate(currentState, peerState, nativeSigned4.signature)).toThrow('turnNum must increment by one');

    // Nonce
    currentState.turnNum = 3;
    peerState.turnNum = 4;
    peerState.channel = {
      chainId: '1',
      channelNonce: 2,
      participants: ['0x63FaC9201494f0bd17B9892B9fae4d52fe3BD377', '0x90F8bf6A479f320ead074411a4B0e7944Ea8c9C1'],
    }
    const nativeSigned5 = native.signState(peerState, PRIVATE_KEY2);

    expect(() => native.validatePeerUpdate(currentState, peerState, nativeSigned5.signature)).toThrow('channelNonce must not change');
  })


  test('Catches invalid private key', async () => {
    // Invalid signature length
    expect(() => native.signState(DEFAULT_STATE, '0x00')).toThrow('invalid private key')
    expect(() => wasm.signState(DEFAULT_STATE, '0x00')).toThrow('invalid private key')
  })
})
