import {
  getChannelId as jsGetChannelId,
  encodeOutcome as jsEncodeOutcome,
  hashAppPart as jsHashAppPart,
  hashOutcome as jsHashOutcome,
  hashState as jsHashState,
  State,
} from '@statechannels/nitro-protocol'
import {
  hashState,
  getChannelId,
  hashAppPart,
  hashOutcome,
  encodeOutcome,
} from '../lib/index'

const DEFAULT_STATE: State = {
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
    expect(getChannelId(channel)).toStrictEqual(jsGetChannelId(channel))
  })

  test('Different nonces', () => {
    const channel1 = {
      chainId: '1',
      channelNonce: 1,
      participants: [],
    }
    expect(getChannelId(channel1)).toStrictEqual(jsGetChannelId(channel1))

    const channel2 = {
      chainId: '1',
      channelNonce: 2,
      participants: [],
    }
    expect(getChannelId(channel2)).toStrictEqual(jsGetChannelId(channel2))

    expect(getChannelId(channel1)).not.toStrictEqual(getChannelId(channel2))
  })

  test('Different chain IDs', () => {
    const channel1 = {
      chainId: '1',
      channelNonce: 1,
      participants: [],
    }
    expect(getChannelId(channel1)).toStrictEqual(jsGetChannelId(channel1))

    const channel2 = {
      chainId: '4',
      channelNonce: 1,
      participants: [],
    }
    expect(getChannelId(channel2)).toStrictEqual(jsGetChannelId(channel2))

    expect(getChannelId(channel1)).not.toStrictEqual(getChannelId(channel2))
  })
})

describe('hashAppPart', () => {
  test('Different app definitions', () => {
    const state1 = {
      ...DEFAULT_STATE,
      appDefinition: '0x0000000000000000000000000000000000000000',
      appData: '0x00',
    }
    expect(hashAppPart(state1)).toStrictEqual(jsHashAppPart(state1))

    const state2 = {
      ...DEFAULT_STATE,
      appDefinition: '0x1111111111111111111111111111111111111111',
      appData: '0x00',
    }
    expect(hashAppPart(state2)).toStrictEqual(jsHashAppPart(state2))

    expect(hashAppPart(state1)).not.toStrictEqual(hashAppPart(state2))
  })

  test('Different app datas', () => {
    const state1 = {
      ...DEFAULT_STATE,
      appDefinition: '0x0000000000000000000000000000000000000000',
      appData: '0x00',
    }
    expect(hashAppPart(state1)).toStrictEqual(jsHashAppPart(state1))

    const state2 = {
      ...DEFAULT_STATE,
      appDefinition: '0x0000000000000000000000000000000000000000',
      appData: '0x01',
    }
    expect(hashAppPart(state2)).toStrictEqual(jsHashAppPart(state2))

    expect(hashAppPart(state1)).not.toStrictEqual(hashAppPart(state2))
  })
})

describe('encodeOutcome', () => {
  test('Empty outcome', () => {
    expect(encodeOutcome(DEFAULT_STATE)).toStrictEqual(
      jsEncodeOutcome(DEFAULT_STATE.outcome),
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
    expect(encodeOutcome(state)).toStrictEqual(jsEncodeOutcome(state.outcome))
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
    expect(encodeOutcome(state)).toStrictEqual(jsEncodeOutcome(state.outcome))
  })

  // FIXME: Something is wrong with the deserialization of (probably) the destinations
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
    expect(encodeOutcome(state)).toStrictEqual(jsEncodeOutcome(state.outcome))
  })
})

describe('hashOutcome', () => {
  test('Empty outcome', () => {
    expect(hashOutcome(DEFAULT_STATE)).toStrictEqual(jsHashOutcome(DEFAULT_STATE.outcome))
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
    expect(hashOutcome(state)).toStrictEqual(jsHashOutcome(state.outcome))
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
    expect(hashOutcome(state)).toStrictEqual(jsHashOutcome(state.outcome))
  })
})

describe('hashState', () => {
  test('Simple state', () => {
    expect(hashState(DEFAULT_STATE)).toStrictEqual(jsHashState(DEFAULT_STATE))
  })
})
