process.argv.push('--experimental-modules', '--experimental-wasm-modules')

import * as nitro from '@statechannels/nitro-protocol'
import * as native from '../native/lib'
import * as wasm from '../wasm/pkg'

const DEFAULT_STATE: nitro.State = {
  turnNum: 1,
  isFinal: false,
  channel: {
    chainId: '1',
    channelNonce: 1,
    participants: [],
  },
  challengeDuration: 1,
  outcome: [],
  appDefinition: '0x0000000000000000000000000000000000000000',
  appData: '0x0000000000000000000000000000000000000000000000000000000000000000',
}

describe('getChannelId', () => {
  test('No participants', () => {
    const channel = {
      chainId: '1',
      channelNonce: 2,
      participants: [],
    }

    expect(native.getChannelId(channel)).toStrictEqual(nitro.getChannelId(channel))
    expect(wasm.getChannelId(channel)).toStrictEqual(nitro.getChannelId(channel))
  })

  test('Different nonces', () => {
    const channel1 = {
      chainId: '1',
      channelNonce: 1,
      participants: [],
    }
    const channel2 = {
      chainId: '1',
      channelNonce: 2,
      participants: [],
    }

    expect(native.getChannelId(channel1)).toStrictEqual(nitro.getChannelId(channel1))
    expect(native.getChannelId(channel2)).toStrictEqual(nitro.getChannelId(channel2))
    expect(native.getChannelId(channel1)).not.toStrictEqual(nitro.getChannelId(channel2))

    expect(wasm.getChannelId(channel1)).toStrictEqual(nitro.getChannelId(channel1))
    expect(wasm.getChannelId(channel2)).toStrictEqual(nitro.getChannelId(channel2))
    expect(wasm.getChannelId(channel1)).not.toStrictEqual(nitro.getChannelId(channel2))
  })

  test('Different chain IDs', () => {
    const channel1 = {
      chainId: '1',
      channelNonce: 1,
      participants: [],
    }

    const channel2 = {
      chainId: '4',
      channelNonce: 1,
      participants: [],
    }

    expect(native.getChannelId(channel1)).toStrictEqual(nitro.getChannelId(channel1))
    expect(native.getChannelId(channel2)).toStrictEqual(nitro.getChannelId(channel2))
    expect(native.getChannelId(channel1)).not.toStrictEqual(nitro.getChannelId(channel2))

    expect(wasm.getChannelId(channel1)).toStrictEqual(nitro.getChannelId(channel1))
    expect(wasm.getChannelId(channel2)).toStrictEqual(nitro.getChannelId(channel2))
    expect(wasm.getChannelId(channel1)).not.toStrictEqual(nitro.getChannelId(channel2))
  })
})

describe('hashAppPart', () => {
  test('Different app definitions', () => {
    const state1 = {
      ...DEFAULT_STATE,
      appDefinition: '0x0000000000000000000000000000000000000000',
      appData: '0x00',
    }
    const state2 = {
      ...DEFAULT_STATE,
      appDefinition: '0x1111111111111111111111111111111111111111',
      appData: '0x00',
    }

    expect(native.hashAppPart(state1)).toStrictEqual(nitro.hashAppPart(state1))
    expect(native.hashAppPart(state2)).toStrictEqual(nitro.hashAppPart(state2))
    expect(native.hashAppPart(state1)).not.toStrictEqual(native.hashAppPart(state2))

    expect(wasm.hashAppPart(state1)).toStrictEqual(nitro.hashAppPart(state1))
    expect(wasm.hashAppPart(state2)).toStrictEqual(nitro.hashAppPart(state2))
    expect(wasm.hashAppPart(state1)).not.toStrictEqual(wasm.hashAppPart(state2))
  })

  test('Different app datas', () => {
    const state1 = {
      ...DEFAULT_STATE,
      appDefinition: '0x0000000000000000000000000000000000000000',
      appData: '0x00',
    }

    const state2 = {
      ...DEFAULT_STATE,
      appDefinition: '0x0000000000000000000000000000000000000000',
      appData: '0x01',
    }

    expect(native.hashAppPart(state1)).toStrictEqual(nitro.hashAppPart(state1))
    expect(native.hashAppPart(state2)).toStrictEqual(nitro.hashAppPart(state2))
    expect(native.hashAppPart(state1)).not.toStrictEqual(native.hashAppPart(state2))

    expect(wasm.hashAppPart(state1)).toStrictEqual(nitro.hashAppPart(state1))
    expect(wasm.hashAppPart(state2)).toStrictEqual(nitro.hashAppPart(state2))
    expect(wasm.hashAppPart(state1)).not.toStrictEqual(wasm.hashAppPart(state2))
  })
})

describe('encodeOutcome', () => {
  test('Empty outcome', () => {
    expect(native.encodeOutcome(DEFAULT_STATE)).toStrictEqual(
      nitro.encodeOutcome(DEFAULT_STATE.outcome),
    )
  })

  test('Single allocation asset outcome and allocation item', () => {
    const state = {
      ...DEFAULT_STATE,
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
    }
    expect(native.encodeOutcome(state)).toStrictEqual(nitro.encodeOutcome(state.outcome))
    expect(wasm.encodeOutcome(state)).toStrictEqual(nitro.encodeOutcome(state.outcome))
  })

  test('Single allocation asset outcome and two allocation items', () => {
    const state = {
      ...DEFAULT_STATE,
      outcome: [
        {
          assetHolderAddress: '0x0000000000000000000000000000000000000000',
          allocationItems: [
            {
              destination:
                '0x0000000000000000000000000000000000000000000000000000000000000000',
              amount: '1',
            },
            {
              destination:
                '0x1111111111111111111111111111111111111111111111111111111111111111',
              amount: '2',
            },
          ],
        },
      ],
    }
    expect(native.encodeOutcome(state)).toStrictEqual(nitro.encodeOutcome(state.outcome))
    expect(wasm.encodeOutcome(state)).toStrictEqual(nitro.encodeOutcome(state.outcome))
  })

  test('Single guarantee outcome', () => {
    const state = {
      ...DEFAULT_STATE,
      outcome: [
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
      ],
    }
    expect(native.encodeOutcome(state)).toStrictEqual(nitro.encodeOutcome(state.outcome))
    expect(wasm.encodeOutcome(state)).toStrictEqual(nitro.encodeOutcome(state.outcome))
  })
})

describe('hashOutcome', () => {
  test('Empty outcome', () => {
    expect(native.hashOutcome(DEFAULT_STATE)).toStrictEqual(
      nitro.hashOutcome(DEFAULT_STATE.outcome),
    )
  })

  test('Single allocation asset outcome and allocation item', () => {
    const state = {
      ...DEFAULT_STATE,
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
    }
    expect(native.hashOutcome(state)).toStrictEqual(nitro.hashOutcome(state.outcome))
    expect(wasm.hashOutcome(state)).toStrictEqual(nitro.hashOutcome(state.outcome))
  })

  test('Single allocation asset outcome and two allocation items', () => {
    const state = {
      ...DEFAULT_STATE,
      outcome: [
        {
          assetHolderAddress: '0x0000000000000000000000000000000000000000',
          allocationItems: [
            {
              destination:
                '0x0000000000000000000000000000000000000000000000000000000000000000',
              amount: '1',
            },
            {
              destination:
                '0x1111111111111111111111111111111111111111111111111111111111111111',
              amount: '2',
            },
          ],
        },
      ],
    }
    expect(native.hashOutcome(state)).toStrictEqual(nitro.hashOutcome(state.outcome))
    expect(wasm.hashOutcome(state)).toStrictEqual(nitro.hashOutcome(state.outcome))
  })
})

describe('hashState', () => {
  test('Simple state', () => {
    expect(native.hashState(DEFAULT_STATE)).toStrictEqual(nitro.hashState(DEFAULT_STATE))
    expect(wasm.hashState(DEFAULT_STATE)).toStrictEqual(nitro.hashState(DEFAULT_STATE))
  })
})
