use yew::prelude::*;
pub mod components;
pub mod scraper;

// #[function_component(App)]
// fn app() -> Html {
//     html! {
//         <>
//             <h1>{ "LINE Video Scrapper" }</h1>
//             <div>
//                 <label for="sharing-link">{"Name: "}</label>
//                 <input type="text" id="sharing-link" name="sharing link" />
//                 <button type="button">{"Scrape"}</button>
//             </div>
//             <div>
//                 <h3>{ "John Doe: Building and breaking things" }</h3>
//                 <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
//                 <br />
//                 <button type="button">{"Download"}</button>
//             </div>
//         </>
//     }
// }

#[tokio::main]
async fn main() {
    //yew::start_app::<App>();
}
