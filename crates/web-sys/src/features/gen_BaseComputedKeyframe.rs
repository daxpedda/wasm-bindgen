#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = BaseComputedKeyframe)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BaseComputedKeyframe` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
    pub type BaseComputedKeyframe;
    #[cfg(feature = "CompositeOperation")]
    #[doc = "Get the `composite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`, `CompositeOperation`*"]
    #[wasm_bindgen(method, getter = "composite")]
    pub fn get_composite(this: &BaseComputedKeyframe) -> Option<CompositeOperation>;
    #[cfg(feature = "CompositeOperation")]
    #[wasm_bindgen(method, setter = "composite")]
    fn set_composite(this: &BaseComputedKeyframe, val: Option<CompositeOperation>);
    #[doc = "Get the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
    #[wasm_bindgen(method, getter = "easing")]
    pub fn get_easing(this: &BaseComputedKeyframe) -> Option<String>;
    #[wasm_bindgen(method, setter = "easing")]
    fn set_easing(this: &BaseComputedKeyframe, val: &str);
    #[doc = "Get the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
    #[wasm_bindgen(method, getter = "offset")]
    pub fn get_offset(this: &BaseComputedKeyframe) -> Option<f64>;
    #[wasm_bindgen(method, setter = "offset")]
    fn set_offset(this: &BaseComputedKeyframe, val: Option<f64>);
    #[doc = "Get the `simulateComputeValuesFailure` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
    #[wasm_bindgen(method, getter = "simulateComputeValuesFailure")]
    pub fn get_simulate_compute_values_failure(this: &BaseComputedKeyframe) -> Option<bool>;
    #[wasm_bindgen(method, setter = "simulateComputeValuesFailure")]
    fn set_simulate_compute_values_failure(this: &BaseComputedKeyframe, val: bool);
    #[doc = "Get the `computedOffset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
    #[wasm_bindgen(method, getter = "computedOffset")]
    pub fn get_computed_offset(this: &BaseComputedKeyframe) -> Option<f64>;
    #[wasm_bindgen(method, setter = "computedOffset")]
    fn set_computed_offset(this: &BaseComputedKeyframe, val: f64);
}
impl BaseComputedKeyframe {
    #[doc = "Construct a new `BaseComputedKeyframe`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "CompositeOperation")]
    #[doc = "Change the `composite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`, `CompositeOperation`*"]
    pub fn composite(&mut self, val: Option<CompositeOperation>) -> &mut Self {
        self.set_composite(val);
        self
    }
    #[doc = "Change the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
    pub fn easing(&mut self, val: &str) -> &mut Self {
        self.set_easing(val);
        self
    }
    #[doc = "Change the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
    pub fn offset(&mut self, val: Option<f64>) -> &mut Self {
        self.set_offset(val);
        self
    }
    #[doc = "Change the `simulateComputeValuesFailure` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
    pub fn simulate_compute_values_failure(&mut self, val: bool) -> &mut Self {
        self.set_simulate_compute_values_failure(val);
        self
    }
    #[doc = "Change the `computedOffset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
    pub fn computed_offset(&mut self, val: f64) -> &mut Self {
        self.set_computed_offset(val);
        self
    }
}
impl Default for BaseComputedKeyframe {
    fn default() -> Self {
        Self::new()
    }
}
