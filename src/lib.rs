use js_sys::Function;
use wasm_bindgen::JsValue;
use serde::{Deserialize, Serialize};
use js_sys::Promise;
use wasm_bindgen_futures::spawn_local;


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
    pub fn get_request() -> Option<Function> {
        let request = js_sys::Reflect::get(
            &*web_sys::window()?.get("ethereum")?,
            &JsValue::from("request")
        ).ok()?;
        Some(Function::from(request))
    }

    pub fn new() -> Result<Self, String> {
        match Self::get_request() {
            Some(req) => Ok(Provider{ this: JsValue::null(), request: req}),
            None => Err("Failed on get `window.ethereum.request`".to_string())
        }
    }

    async fn async_request(self, method: String) -> Result<JsValue, JsValue> {
        let ret = self.request.call1(
            &self.this,
            &JsValue::from_serde(&RequestMethod{method: method}).unwrap()
        );
        match ret {
            Ok(s)=> {
                let promise = Promise::resolve(&s.into());
                Ok(wasm_bindgen_futures::JsFuture::from(promise).await?)
            },
           Err(e) => Err(e)
        }
    }

    pub fn request(self, method: String, cb: Callback) -> () {
        let wrap = async move {
            cb(self.async_request(method).await);
        };
        spawn_local(wrap);
    }
}
