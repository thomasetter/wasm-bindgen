#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCIdentityProvider)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcIdentityProvider` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProvider`*"]
    pub type RtcIdentityProvider;
    #[wasm_bindgen(method, setter = "generateAssertion")]
    fn generate_assertion_shim(this: &RtcIdentityProvider, val: &::js_sys::Function);
    #[wasm_bindgen(method, setter = "validateAssertion")]
    fn validate_assertion_shim(this: &RtcIdentityProvider, val: &::js_sys::Function);
}
impl RtcIdentityProvider {
    #[doc = "Construct a new `RtcIdentityProvider`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProvider`*"]
    pub fn new(
        generate_assertion: &::js_sys::Function,
        validate_assertion: &::js_sys::Function,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.generate_assertion(generate_assertion);
        ret.validate_assertion(validate_assertion);
        ret
    }
    #[doc = "Change the `generateAssertion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProvider`*"]
    pub fn generate_assertion(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.generate_assertion_shim(val);
        self
    }
    #[doc = "Change the `validateAssertion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProvider`*"]
    pub fn validate_assertion(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.validate_assertion_shim(val);
        self
    }
}
