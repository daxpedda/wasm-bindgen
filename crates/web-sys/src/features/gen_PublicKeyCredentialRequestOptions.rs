#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PublicKeyCredentialRequestOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PublicKeyCredentialRequestOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    pub type PublicKeyCredentialRequestOptions;
    #[doc = "Get the `allowCredentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    #[wasm_bindgen(method, getter = "allowCredentials")]
    pub fn get_allow_credentials(
        this: &PublicKeyCredentialRequestOptions,
    ) -> Option<::js_sys::Array>;
    #[doc = "Change the `allowCredentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    #[wasm_bindgen(method, setter = "allowCredentials")]
    pub fn set_allow_credentials(
        this: &PublicKeyCredentialRequestOptions,
        val: &::wasm_bindgen::JsValue,
    );
    #[doc = "Get the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    #[wasm_bindgen(method, getter = "challenge")]
    pub fn get_challenge(this: &PublicKeyCredentialRequestOptions) -> ::js_sys::Object;
    #[doc = "Change the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    #[wasm_bindgen(method, setter = "challenge")]
    pub fn set_challenge(this: &PublicKeyCredentialRequestOptions, val: &::js_sys::Object);
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    #[doc = "Get the `extensions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`, `PublicKeyCredentialRequestOptions`*"]
    #[wasm_bindgen(method, getter = "extensions")]
    pub fn get_extensions(
        this: &PublicKeyCredentialRequestOptions,
    ) -> Option<AuthenticationExtensionsClientInputs>;
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    #[doc = "Change the `extensions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`, `PublicKeyCredentialRequestOptions`*"]
    #[wasm_bindgen(method, setter = "extensions")]
    pub fn set_extensions(
        this: &PublicKeyCredentialRequestOptions,
        val: &AuthenticationExtensionsClientInputs,
    );
    #[doc = "Get the `rpId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    #[wasm_bindgen(method, getter = "rpId")]
    pub fn get_rp_id(this: &PublicKeyCredentialRequestOptions) -> Option<String>;
    #[doc = "Change the `rpId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    #[wasm_bindgen(method, setter = "rpId")]
    pub fn set_rp_id(this: &PublicKeyCredentialRequestOptions, val: &str);
    #[doc = "Get the `timeout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    #[wasm_bindgen(method, getter = "timeout")]
    pub fn get_timeout(this: &PublicKeyCredentialRequestOptions) -> Option<u32>;
    #[doc = "Change the `timeout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    #[wasm_bindgen(method, setter = "timeout")]
    pub fn set_timeout(this: &PublicKeyCredentialRequestOptions, val: u32);
    #[cfg(feature = "UserVerificationRequirement")]
    #[doc = "Get the `userVerification` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`, `UserVerificationRequirement`*"]
    #[wasm_bindgen(method, getter = "userVerification")]
    pub fn get_user_verification(
        this: &PublicKeyCredentialRequestOptions,
    ) -> Option<UserVerificationRequirement>;
    #[cfg(feature = "UserVerificationRequirement")]
    #[doc = "Change the `userVerification` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`, `UserVerificationRequirement`*"]
    #[wasm_bindgen(method, setter = "userVerification")]
    pub fn set_user_verification(
        this: &PublicKeyCredentialRequestOptions,
        val: UserVerificationRequirement,
    );
}
impl PublicKeyCredentialRequestOptions {
    #[doc = "Construct a new `PublicKeyCredentialRequestOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    pub fn new(challenge: &::js_sys::Object) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.challenge(challenge);
        ret
    }
    #[deprecated = "Use `set_allow_credentials()` instead."]
    pub fn allow_credentials(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_allow_credentials(val);
        self
    }
    #[deprecated = "Use `set_challenge()` instead."]
    pub fn challenge(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_challenge(val);
        self
    }
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    #[deprecated = "Use `set_extensions()` instead."]
    pub fn extensions(&mut self, val: &AuthenticationExtensionsClientInputs) -> &mut Self {
        self.set_extensions(val);
        self
    }
    #[deprecated = "Use `set_rp_id()` instead."]
    pub fn rp_id(&mut self, val: &str) -> &mut Self {
        self.set_rp_id(val);
        self
    }
    #[deprecated = "Use `set_timeout()` instead."]
    pub fn timeout(&mut self, val: u32) -> &mut Self {
        self.set_timeout(val);
        self
    }
    #[cfg(feature = "UserVerificationRequirement")]
    #[deprecated = "Use `set_user_verification()` instead."]
    pub fn user_verification(&mut self, val: UserVerificationRequirement) -> &mut Self {
        self.set_user_verification(val);
        self
    }
}
