#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AudioDecoderInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioDecoderInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDecoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type AudioDecoderInit;
    #[wasm_bindgen(method, setter = "error")]
    fn error_shim(this: &AudioDecoderInit, val: &::js_sys::Function);
    #[wasm_bindgen(method, setter = "output")]
    fn output_shim(this: &AudioDecoderInit, val: &::js_sys::Function);
}
#[cfg(web_sys_unstable_apis)]
impl AudioDecoderInit {
    #[doc = "Construct a new `AudioDecoderInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDecoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(error: &::js_sys::Function, output: &::js_sys::Function) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.error(error);
        ret.output(output);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDecoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn error(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.error_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `output` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDecoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn output(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.output_shim(val);
        self
    }
}
