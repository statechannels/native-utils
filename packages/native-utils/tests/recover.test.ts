import { utils } from 'ethers'
import { State } from '@statechannels/nitro-protocol'
import * as nitro from '@statechannels/nitro-protocol'
import * as wasm from '@statechannels/wasm-utils'
import * as native from '..'

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
      asset: '0x0000000000000000000000000000000000000000',
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

const PRIVATE_KEY = '0x1111111111111111111111111111111111111111111111111111111111111111'

describe('Recover address', () => {
  test('Recover from signed state', async () => {
    const signedState = native.signState(DEFAULT_STATE, PRIVATE_KEY)

    // Old
    const oldAddress = nitro.getStateSignerAddress({
      state: DEFAULT_STATE,
      signature: utils.splitSignature(signedState.signature),
    })

    // Native
    const nativeAddress = native.recoverAddress(DEFAULT_STATE, signedState.signature)

    // WASM
    const wasmAddress = wasm.recoverAddress(DEFAULT_STATE, signedState.signature)

    expect(nativeAddress).toStrictEqual(oldAddress)
    expect(wasmAddress).toStrictEqual(oldAddress)
  })

  test('Catches invalid signatures', async () => {
    // Invalid signature length
    expect(() => native.recoverAddress(DEFAULT_STATE, '0x00')).toThrow(
      'invalid signature length',
    )
    expect(() => wasm.recoverAddress(DEFAULT_STATE, '0x00')).toThrow(
      'invalid signature length',
    )

    // Signature with invalid recovery ID
    expect(() =>
      native.recoverAddress(
        DEFAULT_STATE,
        '0x0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000',
      ),
    ).toThrow('invalid recovery ID')
    expect(() =>
      wasm.recoverAddress(
        DEFAULT_STATE,
        '0x0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000',
      ),
    ).toThrow('invalid recovery ID')

    // Altogether invalid signature
    const signedState = native.signState(DEFAULT_STATE, PRIVATE_KEY)
    expect(() =>
      native.recoverAddress(DEFAULT_STATE, `0xf${signedState.signature.substr(13)}`),
    ).toThrow('invalid signature')
    expect(() =>
      wasm.recoverAddress(DEFAULT_STATE, `0xf${signedState.signature.substr(13)}`),
    ).toThrow('invalid signature')
  })
})
