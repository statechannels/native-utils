import { Bytes32, Channel, State } from '@statechannels/nitro-protocol'

/**
 * A Nitro state with its state hash and signature from signing the state.
 */
export interface StateWithHashAndSignature {
  /**
   * The original Nitro state.
   */
  state: State

  /**
   * A 32-byte hash of the state, computed with `hashState`.
   */
  hash: string

  /**
   * A signature, resulting from signing the `state` using `signState`.
   */
  signature: string
}

/**
 * Computes the ID for the given channel.
 *
 * @param channel Channel data.
 */
export function getChannelId(channel: Channel): string

/**
 * Encodes the outcome part of a Nitro state.
 *
 * @param state A Nitro state.
 */
export function encodeOutcome(state: State): string

/**
 * Hashes the app part of a Nitro state.
 *
 * @param state A Nitro state.
 */
export function hashAppPart(state: State): string

/**
 * Hashes an arbitrary message by computing the keccak256 hash of
 *
 *     \x19Ethereum Signed Message:\n${msg.length}${msg}
 *
 * @param msg An arbitrary string.
 */
export function hashMessage(msg: string): string

/**
 * Hashes the outcome part of a Nitro state.
 *
 * @param state A Nitro state.
 */
export function hashOutcome(state: State): string

/**
 * Hashes a Nitro state.
 *
 * @param state A Nitro state.
 */
export function hashState(state: State): string

/**
 * Signs the state with the given private key.
 *
 * @param state A Nitro state.
 * @param privateKey A private Ethereum key.
 */
export function signState(state: State, privateKey: string): StateWithHashAndSignature

/**
 * Recovers the signer address from a signed Nitro state.
 *
 * @param state A Nitro state.
 * @param signature A signature resulting from a previous call to `signState`.
 */
export function recoverAddress(state: State, signature: string): string

/**
 * Verifies a signature.
 *
 * @param state A Nitro state.
 * @param signature A signature resulting from a previous call to `signState`.
 */
 export function verifySignature(hash: Bytes32, address: string, signature: string): boolean
