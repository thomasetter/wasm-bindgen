#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DOMWindowResizeEventDetail)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomWindowResizeEventDetail` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomWindowResizeEventDetail`*"]
    pub type DomWindowResizeEventDetail;
    #[wasm_bindgen(method, setter = "height")]
    fn height_shim(this: &DomWindowResizeEventDetail, val: i32);
    #[wasm_bindgen(method, setter = "width")]
    fn width_shim(this: &DomWindowResizeEventDetail, val: i32);
}
impl DomWindowResizeEventDetail {
    #[doc = "Construct a new `DomWindowResizeEventDetail`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomWindowResizeEventDetail`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomWindowResizeEventDetail`*"]
    pub fn height(&mut self, val: i32) -> &mut Self {
        self.height_shim(val);
        self
    }
    #[doc = "Change the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomWindowResizeEventDetail`*"]
    pub fn width(&mut self, val: i32) -> &mut Self {
        self.width_shim(val);
        self
    }
}
impl Default for DomWindowResizeEventDetail {
    fn default() -> Self {
        Self::new()
    }
}
