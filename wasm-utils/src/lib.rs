use std::ops::Deref;

use js_sys::{JsString};
use wasm_bindgen::prelude::*;

use statechannels_native_utils_common::prelude::{hash_message as do_hash_message, *};

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
import { Channel, State } from '@statechannels/nitro-protocol';

interface StateSignature {
    hash: string
    signature: string
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "State")]
    pub type JsState;

    #[wasm_bindgen(typescript_type = "Channel")]
    pub type JsChannel;

    #[wasm_bindgen(typescript_type = "StateSignature")]
    pub type JsStateSignature;
}

#[wasm_bindgen(js_name = "getChannelId")]
pub fn get_channel_id(channel: &JsChannel) -> JsString {
    let channel: Channel = channel.into_serde().unwrap();
    channel.id().to_hex_string().into()
}

#[wasm_bindgen(js_name = "encodeOutcome")]
pub fn encode_outcome(state: &JsState) -> JsString {
    let state: State = state.into_serde().unwrap();
    state.outcome.encode().to_hex_string().into()
}

#[wasm_bindgen(js_name = "hashAppPart")]
pub fn hash_app_part(state: &JsState) -> JsString {
    let state: State = state.into_serde().unwrap();
    state.hash_app_part().to_hex_string().into()
}

#[wasm_bindgen(js_name = "hashOutcome")]
pub fn hash_outcome(state: &JsState) -> JsString {
    let state: State = state.into_serde().unwrap();
    state.outcome.hash().to_hex_string().into()
}

#[wasm_bindgen(js_name = "hashState")]
pub fn hash_state(state: &JsState) -> JsString {
    let state: State = state.into_serde().unwrap();
    state.hash().to_hex_string().into()
}

#[wasm_bindgen(js_name = "hashMessage")]
pub fn hash_message(msg: &JsString) -> JsString {
    let msg = msg.into_serde::<Bytes>().unwrap();
    Bytes32::from(do_hash_message(&msg.deref()))
        .to_hex_string()
        .into()
}

#[wasm_bindgen(js_name = "signState")]
pub fn sign_state(state: &JsState, private_key: &JsString) -> Result<JsStateSignature, JsValue> {
    let state: State = state.into_serde().unwrap();
    let private_key: Bytes = private_key.into_serde().unwrap();
    let signature = state.sign(private_key).map_err(JsValue::from)?;
    Ok(JsValue::from_serde(&signature).unwrap().into())
}

#[wasm_bindgen(js_name = "recoverAddress")]
pub fn recover_address(state: &JsState, signature: &JsString) -> Result<JsString, JsValue> {
    let state: State = state.into_serde().unwrap();
    let signature: Bytes = signature.into_serde().unwrap();
    let address = state.recover_address(signature).map_err(JsValue::from)?;
    Ok(JsValue::from_serde(&address).unwrap().into())
}

#[wasm_bindgen(js_name = "validatePeerUpdate")]
pub fn validate_peer_update(state: &JsState, peer_update: &JsState, signature: &JsString) -> Result<JsString, JsValue> {
    let state: State = state.into_serde().unwrap();
    let peer_update: State = peer_update.into_serde().unwrap();
    let signature: Bytes = signature.into_serde().unwrap();
    let result = state.validate_peer_update(peer_update, signature).map_err(JsValue::from)?;
    Ok(JsValue::from_serde(&result).unwrap().into())
}