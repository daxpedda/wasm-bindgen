#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = FontFaceSetLoadEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FontFaceSetLoadEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetLoadEventInit`*"]
    pub type FontFaceSetLoadEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetLoadEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &FontFaceSetLoadEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles(this: &FontFaceSetLoadEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetLoadEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &FontFaceSetLoadEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable(this: &FontFaceSetLoadEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetLoadEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &FontFaceSetLoadEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed(this: &FontFaceSetLoadEventInit, val: bool);
    #[doc = "Get the `fontfaces` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetLoadEventInit`*"]
    #[wasm_bindgen(method, getter = "fontfaces")]
    pub fn get_fontfaces(this: &FontFaceSetLoadEventInit) -> Option<::js_sys::Array>;
    #[wasm_bindgen(method, setter = "fontfaces")]
    fn set_fontfaces(this: &FontFaceSetLoadEventInit, val: &::wasm_bindgen::JsValue);
}
impl FontFaceSetLoadEventInit {
    #[doc = "Construct a new `FontFaceSetLoadEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetLoadEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetLoadEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetLoadEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetLoadEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[doc = "Change the `fontfaces` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetLoadEventInit`*"]
    pub fn fontfaces(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_fontfaces(val);
        self
    }
}
impl Default for FontFaceSetLoadEventInit {
    fn default() -> Self {
        Self::new()
    }
}
