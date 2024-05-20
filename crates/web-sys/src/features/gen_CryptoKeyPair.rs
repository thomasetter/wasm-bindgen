#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = CryptoKeyPair)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CryptoKeyPair` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKeyPair`*"]
    pub type CryptoKeyPair;
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(method, setter = "privateKey")]
    fn private_key_shim(this: &CryptoKeyPair, val: &CryptoKey);
    #[cfg(feature = "CryptoKey")]
    #[wasm_bindgen(method, setter = "publicKey")]
    fn public_key_shim(this: &CryptoKeyPair, val: &CryptoKey);
}
impl CryptoKeyPair {
    #[cfg(feature = "CryptoKey")]
    #[doc = "Construct a new `CryptoKeyPair`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `CryptoKeyPair`*"]
    pub fn new(private_key: &CryptoKey, public_key: &CryptoKey) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.private_key(private_key);
        ret.public_key(public_key);
        ret
    }
    #[cfg(feature = "CryptoKey")]
    #[doc = "Change the `privateKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `CryptoKeyPair`*"]
    pub fn private_key(&mut self, val: &CryptoKey) -> &mut Self {
        self.private_key_shim(val);
        self
    }
    #[cfg(feature = "CryptoKey")]
    #[doc = "Change the `publicKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `CryptoKeyPair`*"]
    pub fn public_key(&mut self, val: &CryptoKey) -> &mut Self {
        self.public_key_shim(val);
        self
    }
}
