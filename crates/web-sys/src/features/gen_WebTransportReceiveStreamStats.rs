#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WebTransportReceiveStreamStats)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransportReceiveStreamStats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportReceiveStreamStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type WebTransportReceiveStreamStats;
    #[wasm_bindgen(method, setter = "bytesRead")]
    fn bytes_read_shim(this: &WebTransportReceiveStreamStats, val: f64);
    #[wasm_bindgen(method, setter = "bytesReceived")]
    fn bytes_received_shim(this: &WebTransportReceiveStreamStats, val: f64);
    #[wasm_bindgen(method, setter = "timestamp")]
    fn timestamp_shim(this: &WebTransportReceiveStreamStats, val: f64);
}
#[cfg(web_sys_unstable_apis)]
impl WebTransportReceiveStreamStats {
    #[doc = "Construct a new `WebTransportReceiveStreamStats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportReceiveStreamStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `bytesRead` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportReceiveStreamStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn bytes_read(&mut self, val: f64) -> &mut Self {
        self.bytes_read_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `bytesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportReceiveStreamStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn bytes_received(&mut self, val: f64) -> &mut Self {
        self.bytes_received_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportReceiveStreamStats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.timestamp_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for WebTransportReceiveStreamStats {
    fn default() -> Self {
        Self::new()
    }
}
