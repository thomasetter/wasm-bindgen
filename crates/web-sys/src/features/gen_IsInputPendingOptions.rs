#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = IsInputPendingOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IsInputPendingOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IsInputPendingOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type IsInputPendingOptions;
    #[wasm_bindgen(method, setter = "includeContinuous")]
    fn include_continuous_shim(this: &IsInputPendingOptions, val: bool);
}
#[cfg(web_sys_unstable_apis)]
impl IsInputPendingOptions {
    #[doc = "Construct a new `IsInputPendingOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IsInputPendingOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `includeContinuous` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IsInputPendingOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn include_continuous(&mut self, val: bool) -> &mut Self {
        self.include_continuous_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for IsInputPendingOptions {
    fn default() -> Self {
        Self::new()
    }
}
