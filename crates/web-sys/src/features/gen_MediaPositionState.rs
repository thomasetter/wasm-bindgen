#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MediaPositionState)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaPositionState` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaPositionState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type MediaPositionState;
    #[wasm_bindgen(method, setter = "duration")]
    fn duration_shim(this: &MediaPositionState, val: f64);
    #[wasm_bindgen(method, setter = "playbackRate")]
    fn playback_rate_shim(this: &MediaPositionState, val: f64);
    #[wasm_bindgen(method, setter = "position")]
    fn position_shim(this: &MediaPositionState, val: f64);
}
#[cfg(web_sys_unstable_apis)]
impl MediaPositionState {
    #[doc = "Construct a new `MediaPositionState`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaPositionState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaPositionState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn duration(&mut self, val: f64) -> &mut Self {
        self.duration_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `playbackRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaPositionState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn playback_rate(&mut self, val: f64) -> &mut Self {
        self.playback_rate_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `position` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaPositionState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn position(&mut self, val: f64) -> &mut Self {
        self.position_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for MediaPositionState {
    fn default() -> Self {
        Self::new()
    }
}
