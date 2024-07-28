#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ConsoleProfileEvent)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConsoleProfileEvent` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleProfileEvent`*"]
    pub type ConsoleProfileEvent;
    #[doc = "Get the `action` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleProfileEvent`*"]
    #[wasm_bindgen(method, getter = "action")]
    pub fn get_action(this: &ConsoleProfileEvent) -> Option<String>;
    #[wasm_bindgen(method, setter = "action")]
    fn set_action(this: &ConsoleProfileEvent, val: &str);
    #[doc = "Get the `arguments` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleProfileEvent`*"]
    #[wasm_bindgen(method, getter = "arguments")]
    pub fn get_arguments(this: &ConsoleProfileEvent) -> Option<::js_sys::Array>;
    #[wasm_bindgen(method, setter = "arguments")]
    fn set_arguments(this: &ConsoleProfileEvent, val: &::wasm_bindgen::JsValue);
}
impl ConsoleProfileEvent {
    #[doc = "Construct a new `ConsoleProfileEvent`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleProfileEvent`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `action` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleProfileEvent`*"]
    pub fn action(&mut self, val: &str) -> &mut Self {
        self.set_action(val);
        self
    }
    #[doc = "Change the `arguments` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleProfileEvent`*"]
    pub fn arguments(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_arguments(val);
        self
    }
}
impl Default for ConsoleProfileEvent {
    fn default() -> Self {
        Self::new()
    }
}
