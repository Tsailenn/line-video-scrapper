use yew::prelude::*;
use yew::{html, Component, Context, Html};
pub enum Msg {
    SetInputEnabled(bool),
    Submit(String),
}

pub struct App {
    url: String,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            url: String::from("https://linevoom.line.me/post/_dZQCYP7Znvb76iGh_uKJJ6ddW6EsmbG_QE79IMs/1164747613005013697"),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        // match msg {
        //     Msg::SetInputEnabled(enabled) => {
        //         if self.input_enabled != enabled {
        //             self.input_enabled = enabled;
        //             true // Re-render
        //         } else {
        //             false
        //         }
        //     }
        // }
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h1>{ "LINE Video Scrapper" }</h1>
                <div>
                    <label for="sharing-link">{"Name: "}</label>
                    <input type="text" id="sharing-link" name="sharing link" />
                    <button type="button">{"Scrape"}</button>
                </div>
                <div>
                    <img src="https://cdn.discordapp.com/attachments/520055081031303168/955078755121000468/undertal.jpg" alt="video thumbnail" />
                    <br />
                    <button type="button">{"Download"}</button>
                </div>
            </>
        }
    }
}
