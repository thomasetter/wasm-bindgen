#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = FilePickerAcceptType)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FilePickerAcceptType` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePickerAcceptType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type FilePickerAcceptType;
    #[wasm_bindgen(method, setter = "description")]
    fn description_shim(this: &FilePickerAcceptType, val: &str);
}
#[cfg(web_sys_unstable_apis)]
impl FilePickerAcceptType {
    #[doc = "Construct a new `FilePickerAcceptType`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePickerAcceptType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `description` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePickerAcceptType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn description(&mut self, val: &str) -> &mut Self {
        self.description_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for FilePickerAcceptType {
    fn default() -> Self {
        Self::new()
    }
}
