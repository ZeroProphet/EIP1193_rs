use js_sys::Function;
use wasm_bindgen::JsValue;
use serde::{Deserialize, Serialize};
use js_sys::Promise;
use std::vec::Vec;
use web_sys::Event;
use web_sys::EventTarget;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

pub type Callback = Box<dyn Fn(Result<JsValue, JsValue>) -> ()>;


#[derive(Debug, Clone)]
pub struct Provider {
    pub this: JsValue,
    pub request: Function
}

#[derive(Serialize, Deserialize)]
pub struct RequestMethod {
    method: String
}


impl Provider {

    pub fn new() -> Self {
        let provider = web_sys::window().unwrap().get("ethereum").unwrap();
        let request = js_sys::Reflect::get(
            &provider,
            &JsValue::from("request")
        ).unwrap();
        return Provider {this: JsValue::from(provider), request: Function::from(request)};
    }

    pub fn on(self, event: String, callback: Box<dyn FnMut(Event)>) -> Result<(), JsValue>{
        // doc: https://rustwasm.github.io/wasm-bindgen/examples/paint.html
        let closure = Closure::wrap(callback);
        return EventTarget::from(
            self.this
        ).add_event_listener_with_callback(&event, closure.as_ref().unchecked_ref())
    }

    pub async fn request(self, method: String, params: Option<Vec<String>> ) -> Result<JsValue, JsValue> {
        let ret = self.request.call2(
            &self.this,
            &JsValue::from_serde(&RequestMethod{method: method}).unwrap(),
            &JsValue::from_serde(&params).unwrap()
        );
        match ret {
            Ok(s)=> {
                let promise = Promise::resolve(&s.into());
                Ok(wasm_bindgen_futures::JsFuture::from(promise).await?)
            },
           Err(e) => Err(e)
        }
    }
}
