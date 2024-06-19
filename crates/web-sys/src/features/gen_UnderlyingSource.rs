#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = UnderlyingSource)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UnderlyingSource` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
    pub type UnderlyingSource;
    #[doc = "Get the `autoAllocateChunkSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
    #[wasm_bindgen(method, getter = "autoAllocateChunkSize")]
    pub fn get_auto_allocate_chunk_size(this: &UnderlyingSource) -> Option<f64>;
    #[wasm_bindgen(method, setter = "autoAllocateChunkSize")]
    fn set_auto_allocate_chunk_size(this: &UnderlyingSource, val: f64);
    #[doc = "Get the `cancel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
    #[wasm_bindgen(method, getter = "cancel")]
    pub fn get_cancel(this: &UnderlyingSource) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter = "cancel")]
    fn set_cancel(this: &UnderlyingSource, val: &::js_sys::Function);
    #[doc = "Get the `pull` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
    #[wasm_bindgen(method, getter = "pull")]
    pub fn get_pull(this: &UnderlyingSource) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter = "pull")]
    fn set_pull(this: &UnderlyingSource, val: &::js_sys::Function);
    #[doc = "Get the `start` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
    #[wasm_bindgen(method, getter = "start")]
    pub fn get_start(this: &UnderlyingSource) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter = "start")]
    fn set_start(this: &UnderlyingSource, val: &::js_sys::Function);
    #[cfg(feature = "ReadableStreamType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamType`, `UnderlyingSource`*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &UnderlyingSource) -> Option<ReadableStreamType>;
    #[cfg(feature = "ReadableStreamType")]
    #[wasm_bindgen(method, setter = "type")]
    fn set_type(this: &UnderlyingSource, val: ReadableStreamType);
}
impl UnderlyingSource {
    #[doc = "Construct a new `UnderlyingSource`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `autoAllocateChunkSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
    pub fn auto_allocate_chunk_size(&mut self, val: f64) -> &mut Self {
        self.set_auto_allocate_chunk_size(val);
        self
    }
    #[doc = "Change the `cancel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
    pub fn cancel(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_cancel(val);
        self
    }
    #[doc = "Change the `pull` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
    pub fn pull(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_pull(val);
        self
    }
    #[doc = "Change the `start` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
    pub fn start(&mut self, val: &::js_sys::Function) -> &mut Self {
        self.set_start(val);
        self
    }
    #[cfg(feature = "ReadableStreamType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamType`, `UnderlyingSource`*"]
    pub fn type_(&mut self, val: ReadableStreamType) -> &mut Self {
        self.set_type(val);
        self
    }
}
impl Default for UnderlyingSource {
    fn default() -> Self {
        Self::new()
    }
}
