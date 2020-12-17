use yew::prelude::*;
use web_sys;
use js_sys;
use js_sys::JSON::stringify;
use wasm_bindgen_futures::spawn_local;
use serde::{Deserialize, Serialize};
use std::panic;
use wasm_bindgen::JsValue;
use std::fmt::Display;
use EIP1193::Provider;
use EIP1193::Callback;

#[derive(Serialize, Deserialize)]
pub struct RequestMethod {
    method: String
}

pub struct DAOApp {
    pub link: ComponentLink<Self>,
    pub chain_id: Option<String>,
    pub provider: Provider
}

pub enum Msg {
    BlockId,
    GotBlockId(String),
}


impl Component for DAOApp {
    type Properties = ();
    type Message = Msg;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        panic::set_hook(Box::new(console_error_panic_hook::hook));
        Self {
            link: link,
            chain_id: None,
            provider: Provider::new().unwrap()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::BlockId => {

                // self.provider.clone().request("eth_chainId".to_string(), box move |x| {
                //     let chain_id = stringify(&x.unwrap()).unwrap().as_string().unwrap();
                // });
                let link = self.link.clone();
                let provider = self.provider.clone();
                spawn_local(async move {
                    match provider.request("eth_chainId".to_string()).await {
                        Ok(cid) => {
                            link.send_message(Msg::GotBlockId(stringify(&cid).unwrap().as_string().unwrap()));
                        },
                        Err(e) => {
                            link.send_message(Msg::GotBlockId(stringify(&e).unwrap().as_string().unwrap()));
                        }
                    }
                });
                return true;
            },
            Msg::GotBlockId(x) => {
                self.chain_id = Some(x);
                return true;
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::BlockId)>
            {"Click to Get block id: "}{self.chain_id.as_ref().unwrap_or(&" ".to_string())}
                </button>
            </div>
        }
    }
}
