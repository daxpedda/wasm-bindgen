#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUImageCopyTexture)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuImageCopyTexture` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuImageCopyTexture;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureAspect")]
    #[doc = "Get the `aspect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyTexture`, `GpuTextureAspect`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "aspect")]
    pub fn get_aspect(this: &GpuImageCopyTexture) -> Option<GpuTextureAspect>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureAspect")]
    #[doc = "Change the `aspect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyTexture`, `GpuTextureAspect`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "aspect")]
    pub fn set_aspect(this: &GpuImageCopyTexture, val: GpuTextureAspect);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `mipLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "mipLevel")]
    pub fn get_mip_level(this: &GpuImageCopyTexture) -> Option<u32>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `mipLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "mipLevel")]
    pub fn set_mip_level(this: &GpuImageCopyTexture, val: u32);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "origin")]
    pub fn get_origin(this: &GpuImageCopyTexture) -> ::wasm_bindgen::JsValue;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "origin")]
    pub fn set_origin(this: &GpuImageCopyTexture, val: &::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTexture")]
    #[doc = "Get the `texture` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyTexture`, `GpuTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "texture")]
    pub fn get_texture(this: &GpuImageCopyTexture) -> Option<GpuTexture>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTexture")]
    #[doc = "Change the `texture` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyTexture`, `GpuTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "texture")]
    pub fn set_texture(this: &GpuImageCopyTexture, val: &GpuTexture);
}
#[cfg(web_sys_unstable_apis)]
impl GpuImageCopyTexture {
    #[cfg(feature = "GpuTexture")]
    #[doc = "Construct a new `GpuImageCopyTexture`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyTexture`, `GpuTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(texture: &GpuTexture) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.texture(texture);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureAspect")]
    #[deprecated = "Use `set_aspect()` instead."]
    pub fn aspect(&mut self, val: GpuTextureAspect) -> &mut Self {
        self.set_aspect(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_mip_level()` instead."]
    pub fn mip_level(&mut self, val: u32) -> &mut Self {
        self.set_mip_level(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_origin()` instead."]
    pub fn origin(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_origin(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTexture")]
    #[deprecated = "Use `set_texture()` instead."]
    pub fn texture(&mut self, val: &GpuTexture) -> &mut Self {
        self.set_texture(val);
        self
    }
}
