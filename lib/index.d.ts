import { Channel, State } from '@statechannels/nitro-protocol'

export interface StateWithHashAndSignature {
  state: State
  hash: string
  signature: string
}

export function getChannelId(channel: Channel): string

export function encodeOutcome(state: State): string
export function hashAppPart(state: State): string
export function hashMessage(msg: String): String
export function hashOutcome(state: State): string
export function hashState(state: State): string
export function signState(state: State, privateKey: String): StateWithHashAndSignature
