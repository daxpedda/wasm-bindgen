#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GroupedHistoryEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GroupedHistoryEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GroupedHistoryEventInit`*"]
    pub type GroupedHistoryEventInit;
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GroupedHistoryEventInit`*"]
    #[wasm_bindgen(method, getter = "bubbles")]
    pub fn get_bubbles(this: &GroupedHistoryEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles(this: &GroupedHistoryEventInit, val: bool);
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GroupedHistoryEventInit`*"]
    #[wasm_bindgen(method, getter = "cancelable")]
    pub fn get_cancelable(this: &GroupedHistoryEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable(this: &GroupedHistoryEventInit, val: bool);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GroupedHistoryEventInit`*"]
    #[wasm_bindgen(method, getter = "composed")]
    pub fn get_composed(this: &GroupedHistoryEventInit) -> Option<bool>;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed(this: &GroupedHistoryEventInit, val: bool);
    #[cfg(feature = "Element")]
    #[doc = "Get the `otherBrowser` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `GroupedHistoryEventInit`*"]
    #[wasm_bindgen(method, getter = "otherBrowser")]
    pub fn get_other_browser(this: &GroupedHistoryEventInit) -> Option<Element>;
    #[cfg(feature = "Element")]
    #[wasm_bindgen(method, setter = "otherBrowser")]
    fn set_other_browser(this: &GroupedHistoryEventInit, val: Option<&Element>);
}
impl GroupedHistoryEventInit {
    #[doc = "Construct a new `GroupedHistoryEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GroupedHistoryEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GroupedHistoryEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GroupedHistoryEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GroupedHistoryEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed(val);
        self
    }
    #[cfg(feature = "Element")]
    #[doc = "Change the `otherBrowser` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `GroupedHistoryEventInit`*"]
    pub fn other_browser(&mut self, val: Option<&Element>) -> &mut Self {
        self.set_other_browser(val);
        self
    }
}
impl Default for GroupedHistoryEventInit {
    fn default() -> Self {
        Self::new()
    }
}
