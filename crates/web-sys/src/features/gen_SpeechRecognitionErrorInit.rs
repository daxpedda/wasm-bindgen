#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = SpeechRecognitionErrorInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechRecognitionErrorInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorInit`*"]
    pub type SpeechRecognitionErrorInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &SpeechRecognitionErrorInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles(this: &SpeechRecognitionErrorInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &SpeechRecognitionErrorInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable(this: &SpeechRecognitionErrorInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &SpeechRecognitionErrorInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed(this: &SpeechRecognitionErrorInit, val: bool);
    #[cfg(feature = "SpeechRecognitionErrorCode")]
    #[doc = "Get the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorCode`, `SpeechRecognitionErrorInit`*"]
    #[wasm_bindgen(method, getter = "error")]
    pub fn get_error(this: &SpeechRecognitionErrorInit) -> Option<SpeechRecognitionErrorCode>;
    #[cfg(feature = "SpeechRecognitionErrorCode")]
    #[wasm_bindgen(method, setter = "error")]
    fn set_error(this: &SpeechRecognitionErrorInit, val: SpeechRecognitionErrorCode);
    #[doc = "Get the `message` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorInit`*"]
    #[wasm_bindgen(method, getter = "message")]
    pub fn get_message(this: &SpeechRecognitionErrorInit) -> Option<String>;
    #[wasm_bindgen(method, setter = "message")]
    fn set_message(this: &SpeechRecognitionErrorInit, val: &str);
}
impl SpeechRecognitionErrorInit {
    #[doc = "Construct a new `SpeechRecognitionErrorInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[cfg(feature = "SpeechRecognitionErrorCode")]
    #[doc = "Change the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorCode`, `SpeechRecognitionErrorInit`*"]
    pub fn error(&mut self, val: SpeechRecognitionErrorCode) -> &mut Self {
        self.set_error(val);
        self
    }
    #[doc = "Change the `message` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionErrorInit`*"]
    pub fn message(&mut self, val: &str) -> &mut Self {
        self.set_message(val);
        self
    }
}
impl Default for SpeechRecognitionErrorInit {
    fn default() -> Self {
        Self::new()
    }
}
