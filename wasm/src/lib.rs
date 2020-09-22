#![allow(non_snake_case)]
use std::ops::Deref;

use wasm_bindgen::prelude::*;

use statechannels_native_utils_common::prelude::*;

#[wasm_bindgen]
pub fn getChannelId(channel: &JsValue) -> JsValue {
    let channel: Channel = channel.into_serde().unwrap();
    channel.id().to_hex_string().into()
}

#[wasm_bindgen]
pub fn encodeOutcome(state: &JsValue) -> JsValue {
    let state: State = state.into_serde().unwrap();
    state.outcome.encode().to_hex_string().into()
}

#[wasm_bindgen]
pub fn hashAppPart(state: &JsValue) -> JsValue {
    let state: State = state.into_serde().unwrap();
    state.hash_app_part().to_hex_string().into()
}

#[wasm_bindgen]
pub fn hashOutcome(state: &JsValue) -> JsValue {
    let state: State = state.into_serde().unwrap();
    state.outcome.hash().to_hex_string().into()
}

#[wasm_bindgen]
pub fn hashState(state: &JsValue) -> JsValue {
    let state: State = state.into_serde().unwrap();
    state.hash().to_hex_string().into()
}

#[wasm_bindgen]
pub fn hashMessage(msg: &JsValue) -> JsValue {
    let msg = msg.into_serde::<Bytes>().unwrap();
    Bytes32::from(hash_message(&msg.deref()))
        .to_hex_string()
        .into()
}

#[wasm_bindgen]
pub fn signState(state: &JsValue, private_key: &JsValue) -> Result<JsValue, JsValue> {
    let state: State = state.into_serde().unwrap();
    let private_key: Bytes = private_key.into_serde().unwrap();
    let signature = state.sign(private_key).map_err(JsValue::from)?;
    Ok(JsValue::from_serde(&signature).unwrap())
}

#[wasm_bindgen]
pub fn recoverAddress(state: &JsValue, signature: &JsValue) -> Result<JsValue, JsValue> {
    let state: State = state.into_serde().unwrap();
    let signature: Bytes = signature.into_serde().unwrap();
    let address = state.recover_address(signature).map_err(JsValue::from)?;
    Ok(JsValue::from_serde(&address).unwrap())
}
