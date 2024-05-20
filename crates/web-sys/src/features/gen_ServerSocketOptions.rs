#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ServerSocketOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ServerSocketOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServerSocketOptions`*"]
    pub type ServerSocketOptions;
    #[cfg(feature = "TcpSocketBinaryType")]
    #[wasm_bindgen(method, setter = "binaryType")]
    fn binary_type_shim(this: &ServerSocketOptions, val: TcpSocketBinaryType);
}
impl ServerSocketOptions {
    #[doc = "Construct a new `ServerSocketOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServerSocketOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "TcpSocketBinaryType")]
    #[doc = "Change the `binaryType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServerSocketOptions`, `TcpSocketBinaryType`*"]
    pub fn binary_type(&mut self, val: TcpSocketBinaryType) -> &mut Self {
        self.binary_type_shim(val);
        self
    }
}
impl Default for ServerSocketOptions {
    fn default() -> Self {
        Self::new()
    }
}
