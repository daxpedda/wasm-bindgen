#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = IDBFileMetadataParameters)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbFileMetadataParameters` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileMetadataParameters`*"]
    pub type IdbFileMetadataParameters;
    #[doc = "Get the `lastModified` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileMetadataParameters`*"]
    #[wasm_bindgen(method, getter = "lastModified")]
    pub fn get_last_modified(this: &IdbFileMetadataParameters) -> Option<bool>;
    #[wasm_bindgen(method, setter = "lastModified")]
    fn set_last_modified(this: &IdbFileMetadataParameters, val: bool);
    #[doc = "Get the `size` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileMetadataParameters`*"]
    #[wasm_bindgen(method, getter = "size")]
    pub fn get_size(this: &IdbFileMetadataParameters) -> Option<bool>;
    #[wasm_bindgen(method, setter = "size")]
    fn set_size(this: &IdbFileMetadataParameters, val: bool);
}
impl IdbFileMetadataParameters {
    #[doc = "Construct a new `IdbFileMetadataParameters`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileMetadataParameters`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `lastModified` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileMetadataParameters`*"]
    pub fn last_modified(&mut self, val: bool) -> &mut Self {
        self.set_last_modified(val);
        self
    }
    #[doc = "Change the `size` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileMetadataParameters`*"]
    pub fn size(&mut self, val: bool) -> &mut Self {
        self.set_size(val);
        self
    }
}
impl Default for IdbFileMetadataParameters {
    fn default() -> Self {
        Self::new()
    }
}
