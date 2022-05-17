use log::{info, warn};
use wasm_bindgen::JsCast;
use web3::types::Address;
use web_sys::{EventTarget, HtmlInputElement, InputEvent};
use yew::prelude::*;

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{ "My App" }</h1>
                <SearchBar/>
            </div>
        }
    }

    fn create(_: &Context<Self>) -> Self {
        App {}
    }
}

pub enum Msg {
    InputValue(String),
}

pub struct SearchBar;

fn is_address(value: &str) -> bool {
    /*match let result_addr: Address = value.parse() {
        Ok(_) => true,
        Err(_) => false,
    }*/
    if let Address(_) = value.parse() {
        return true;
    }
    false
}

fn is_tx_hash() -> bool {}

impl Component for SearchBar {
    type Message = Msg;
    type Properties = ();

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_change = ctx.link().callback(|e: InputEvent| {
            // DESIGN:
            // - user enters "one full data point" - like a tx_hash or an address etc.(I'll find more as I go through more API)
            // - once a full data point is found - put some sort of loading indicator and send off the API calls through the backend library
            // - (maybe) the api calls come back basically as a set of different rust structs. Maybe I have an enum of all possible structs that my SearchResult component can display and it will display each one slightly differently. This could be good cause it would allow me to have just one component for all results even if they vary a bit.

            let target: EventTarget = e.target().expect("Event should have a target");

            // this form of on_change can only be used when the target is a HtmlInputElement
            // the full_value is all of the text currently in the target HtmlInputElement
            let full_value = target.unchecked_into::<HtmlInputElement>().value();

            // TODO: comment out below logging when not testing
            info!("Current search query: {}", full_value);

            // TODO: for every change to the text parse out the content for the API parameters we're looking for
            // once a "full parameter" is found, attempt to make API calls using it
            // e.g. tx_hash, address

            Msg::InputValue(full_value)
        });
        html! {
            <label for="search-bar">
            { "My Search Bar: " }
                <input oninput={on_change} id="search-bar" type="text"/>
            </label>
        }
    }

    fn create(_: &Context<Self>) -> Self {
        SearchBar {}
    }
}

pub struct SearchResult;

impl Component for SearchResult {
    type Message = ();
    type Properties = ();

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <h1>{ "Sample Search Result" }</h1>
        }
    }

    fn create(_: &Context<Self>) -> Self {
        SearchResult {}
    }
}
