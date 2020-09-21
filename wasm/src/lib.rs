extern crate wasm_bindgen;

use std::ops::Deref;

use wasm_bindgen::prelude::*;

mod encode;
mod serde;
mod state;
mod tokenize;
mod types;
mod utils;

use encode::*;
use state::*;
use types::*;
use utils::*;

#[wasm_bindgen]
pub fn getChannelId(channel: &JsValue) -> JsValue {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    channel
        .into_serde::<Channel>()
        .unwrap()
        .id()
        .to_hex_string()
        .into()
}

#[wasm_bindgen]
pub fn hashAppPart(state: &JsValue) -> JsValue {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    state
        .into_serde::<State>()
        .unwrap()
        .hash_app_part()
        .to_hex_string()
        .into()
}

#[wasm_bindgen]
pub fn hashOutcome(state: &JsValue) -> JsValue {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    state
        .into_serde::<State>()
        .unwrap()
        .outcome
        .hash()
        .to_hex_string()
        .into()
}

#[wasm_bindgen]
pub fn hashState(state: &JsValue) -> JsValue {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    state
        .into_serde::<State>()
        .unwrap()
        .hash()
        .to_hex_string()
        .into()
}

#[wasm_bindgen]
pub fn hashMessage(msg: &JsValue) -> JsValue {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let msg = msg.into_serde::<Bytes>().unwrap();
    Bytes32::from(hash_message(&msg.deref()))
        .to_hex_string()
        .into()
}

#[wasm_bindgen]
pub fn signState(state: &JsValue, private_key: &JsValue) -> JsValue {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let state: State = state.into_serde().unwrap();
    let private_key: Bytes = private_key.into_serde().unwrap();

    JsValue::from_serde(&state.sign(private_key).unwrap()).unwrap()
}

#[wasm_bindgen]
pub fn recoverAddress(state: &JsValue, signature: &JsValue) -> JsValue {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let state: State = state.into_serde().unwrap();
    let signature: Bytes = signature.into_serde().unwrap();

    JsValue::from_serde(&state.recover_address(signature).unwrap()).unwrap()
}
