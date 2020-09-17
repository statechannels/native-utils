import { Channel, State } from '@statechannels/nitro-protocol'

export function getChannelId(channel: Channel): string

export function encodeOutcome(state: State): string
export function hashAppPart(state: State): string
export function hashOutcome(state: State): string
export function hashState(state: State): string
