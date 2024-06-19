#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCRtpContributingSource)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpContributingSource` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpContributingSource`*"]
    pub type RtcRtpContributingSource;
    #[doc = "Get the `audioLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpContributingSource`*"]
    #[wasm_bindgen(method, getter = "audioLevel")]
    pub fn get_audio_level(this: &RtcRtpContributingSource) -> Option<f64>;
    #[wasm_bindgen(method, setter = "audioLevel")]
    fn set_audio_level(this: &RtcRtpContributingSource, val: f64);
    #[doc = "Get the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpContributingSource`*"]
    #[wasm_bindgen(method, getter = "source")]
    pub fn get_source(this: &RtcRtpContributingSource) -> u32;
    #[wasm_bindgen(method, setter = "source")]
    fn set_source(this: &RtcRtpContributingSource, val: u32);
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpContributingSource`*"]
    #[wasm_bindgen(method, getter = "timestamp")]
    pub fn get_timestamp(this: &RtcRtpContributingSource) -> f64;
    #[wasm_bindgen(method, setter = "timestamp")]
    fn set_timestamp(this: &RtcRtpContributingSource, val: f64);
}
impl RtcRtpContributingSource {
    #[doc = "Construct a new `RtcRtpContributingSource`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpContributingSource`*"]
    pub fn new(source: u32, timestamp: f64) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.source(source);
        ret.timestamp(timestamp);
        ret
    }
    #[doc = "Change the `audioLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpContributingSource`*"]
    pub fn audio_level(&mut self, val: f64) -> &mut Self {
        self.set_audio_level(val);
        self
    }
    #[doc = "Change the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpContributingSource`*"]
    pub fn source(&mut self, val: u32) -> &mut Self {
        self.set_source(val);
        self
    }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpContributingSource`*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        self.set_timestamp(val);
        self
    }
}
