#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCIceServer)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcIceServer` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceServer`*"]
    pub type RtcIceServer;
    #[doc = "Get the `credential` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceServer`*"]
    #[wasm_bindgen(method, getter = "credential")]
    pub fn get_credential(this: &RtcIceServer) -> Option<String>;
    #[wasm_bindgen(method, setter = "credential")]
    fn set_credential(this: &RtcIceServer, val: &str);
    #[cfg(feature = "RtcIceCredentialType")]
    #[doc = "Get the `credentialType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCredentialType`, `RtcIceServer`*"]
    #[wasm_bindgen(method, getter = "credentialType")]
    pub fn get_credential_type(this: &RtcIceServer) -> Option<RtcIceCredentialType>;
    #[cfg(feature = "RtcIceCredentialType")]
    #[wasm_bindgen(method, setter = "credentialType")]
    fn set_credential_type(this: &RtcIceServer, val: RtcIceCredentialType);
    #[doc = "Get the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceServer`*"]
    #[wasm_bindgen(method, getter = "url")]
    pub fn get_url(this: &RtcIceServer) -> Option<String>;
    #[wasm_bindgen(method, setter = "url")]
    fn set_url(this: &RtcIceServer, val: &str);
    #[doc = "Get the `urls` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceServer`*"]
    #[wasm_bindgen(method, getter = "urls")]
    pub fn get_urls(this: &RtcIceServer) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "urls")]
    fn set_urls(this: &RtcIceServer, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `username` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceServer`*"]
    #[wasm_bindgen(method, getter = "username")]
    pub fn get_username(this: &RtcIceServer) -> Option<String>;
    #[wasm_bindgen(method, setter = "username")]
    fn set_username(this: &RtcIceServer, val: &str);
}
impl RtcIceServer {
    #[doc = "Construct a new `RtcIceServer`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceServer`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `credential` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceServer`*"]
    pub fn credential(&mut self, val: &str) -> &mut Self {
        self.set_credential(val);
        self
    }
    #[cfg(feature = "RtcIceCredentialType")]
    #[doc = "Change the `credentialType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCredentialType`, `RtcIceServer`*"]
    pub fn credential_type(&mut self, val: RtcIceCredentialType) -> &mut Self {
        self.set_credential_type(val);
        self
    }
    #[doc = "Change the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceServer`*"]
    pub fn url(&mut self, val: &str) -> &mut Self {
        self.set_url(val);
        self
    }
    #[doc = "Change the `urls` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceServer`*"]
    pub fn urls(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_urls(val);
        self
    }
    #[doc = "Change the `username` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceServer`*"]
    pub fn username(&mut self, val: &str) -> &mut Self {
        self.set_username(val);
        self
    }
}
impl Default for RtcIceServer {
    fn default() -> Self {
        Self::new()
    }
}
