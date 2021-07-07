import { State } from '@statechannels/nitro-protocol'
import * as wasm from '@statechannels/wasm-utils'
import * as native from '..'

const OUTCOME = [
  {
    asset: '0x0000000000000000000000000000000000000000',
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

const CURRENT_STATE: State = {
  turnNum: 5,
  isFinal: false,
  channel: {
    chainId: '1',
    channelNonce: 1,
    participants: ['0x63FaC9201494f0bd17B9892B9fae4d52fe3BD377', '0x90F8bf6A479f320ead074411a4B0e7944Ea8c9C1'],
  },
  challengeDuration: 1,
  outcome: OUTCOME,
  appDefinition: '0x0000000000000000000000000000000000000000',
  appData: '0x0000000000000000000000000000000000000000000000000000000000000000',
}

const NEXT_STATE: State = {
  turnNum: 6,
  isFinal: false,
  channel: {
    chainId: '1',
    channelNonce: 1,
    participants: ['0x63FaC9201494f0bd17B9892B9fae4d52fe3BD377', '0x90F8bf6A479f320ead074411a4B0e7944Ea8c9C1'],
  },
  challengeDuration: 1,
  outcome: OUTCOME,
  appDefinition: '0x0000000000000000000000000000000000000000',
  appData: '0x0000000000000000000000000000000000000000000000000000000000000000',
}

const PRIVATE_KEY1 = '0x8da4ef21b864d2cc526dbdb2a120bd2874c36c9d0a1fb7f8c63d7f7a8b41de8f'
const PRIVATE_KEY2 = '0x4f3edf983ac636a65a842ce7c78d9aa706d3b113bce9c46f30d7d21715b23b1d'


describe('Validate state transitions', () => {
  test('Pre fund setup passes', async () => {

    let currentState = {
      ...CURRENT_STATE,
    }

    let peerState = {
      ...NEXT_STATE,
    }

    // First 2 updates
    currentState.turnNum = 1;
    peerState.turnNum = 2;

    const nativeSigned = native.signState(peerState, PRIVATE_KEY2);
    expect(native.validatePeerUpdate(currentState, peerState, nativeSigned.signature)).toEqual("True")
    expect(wasm.validatePeerUpdate(currentState, peerState, nativeSigned.signature)).toEqual("True")
  })

  test('state transition passes', async () => {
    let currentState = {
      ...CURRENT_STATE,
    }

    let peerState = {
      ...NEXT_STATE,
    }

    currentState.turnNum = 5;
    peerState.turnNum = 6;

    const nativeSigned = native.signState(peerState, PRIVATE_KEY2);
    expect(native.validatePeerUpdate(currentState, peerState, nativeSigned.signature)).toEqual("NeedToCheckApp")
    expect(wasm.validatePeerUpdate(currentState, peerState, nativeSigned.signature)).toEqual("NeedToCheckApp")

  })

  test('signer mismatch fails', async () => {
    let currentState = {
      ...CURRENT_STATE,
    }

    let peerState = {
      ...NEXT_STATE,
    }

    const nativeSigned = native.signState(peerState, PRIVATE_KEY1);
    expect(() => native.validatePeerUpdate(currentState, peerState, nativeSigned.signature)).toThrow('Signature verification failed');
    expect(() => wasm.validatePeerUpdate(currentState, peerState, nativeSigned.signature)).toThrow('Signature verification failed');
  });

  test('turn number mismatch fails', async () => {
    let currentState = {
      ...CURRENT_STATE,
    }

    let peerState = {
      ...NEXT_STATE,
    }

    currentState.turnNum = 4;
    peerState.turnNum = 6;
    const nativeSigned4 = native.signState(peerState, PRIVATE_KEY2);
    expect(() => native.validatePeerUpdate(currentState, peerState, nativeSigned4.signature)).toThrow('turnNum must increment by one');
    expect(() => wasm.validatePeerUpdate(currentState, peerState, nativeSigned4.signature)).toThrow('turnNum must increment by one');
  });

  test('final transit to non-final fails fails', async () => {
    let currentState = {
      ...CURRENT_STATE,
    }

    let peerState = {
      ...NEXT_STATE,
    }

    currentState.turnNum = 5;
    currentState.isFinal = true;
    peerState.turnNum = 6;
    peerState.isFinal = false;
    const nativeSigned = native.signState(peerState, PRIVATE_KEY2);
    expect(() => native.validatePeerUpdate(currentState, peerState, nativeSigned.signature)).toThrow('transition from a final state to a non-final state');
    expect(() => wasm.validatePeerUpdate(currentState, peerState, nativeSigned.signature)).toThrow('transition from a final state to a non-final state');
  });

  test('Outcome mismatch mismatch fails', async () => {
    const otheroutcome = [
      {
        asset: '0x0000000000000000000000000000000000000000',
        guarantee: {
          targetChannelId:
            '0x0000000000000000000000000000000000000000000000000000000000000000',
          destinations: [
            '0x0000000000000000000000000000000000000000000000000000000000000000',
            '0x2222222222222222222222222222222222222222222222220222222222222222',
          ],
        },
      },
    ]

    let currentState = {
      ...CURRENT_STATE,
    }

    let peerState = {
      ...NEXT_STATE,
    }

    currentState.isFinal = true
    currentState.turnNum = 5;

    peerState.outcome = otheroutcome
    peerState.isFinal = true
    peerState.turnNum = 6;
 
    const nativeSigned = native.signState(peerState, PRIVATE_KEY2);

    expect(() => native.validatePeerUpdate(currentState, peerState, nativeSigned.signature)).toThrow('Outcome change forbidden');
    expect(() => wasm.validatePeerUpdate(currentState, peerState, nativeSigned.signature)).toThrow('Outcome change forbidden');
  })
})
