#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = FileSystemEntriesCallback)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemEntriesCallback` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemEntriesCallback`*"]
    pub type FileSystemEntriesCallback;
    #[wasm_bindgen(method, setter = "handleEvent")]
    fn handle_event_shim(this: &FileSystemEntriesCallback, val: &::js_sys::Function);
}
impl FileSystemEntriesCallback {
    #[doc = "Construct a new `FileSystemEntriesCallback`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemEntriesCallback`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `handleEvent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemEntriesCallback`*"]
    pub fn handle_event(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.handle_event_shim(val);
        self
    }
}
impl Default for FileSystemEntriesCallback {
    fn default() -> Self {
        Self::new()
    }
}
