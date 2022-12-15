mod video;

use yew::prelude::*;
use log::info;
use wasm_bindgen::JsValue;
use crate::video::VideosList;

#[function_component(App)]
fn app() -> Html 
{
    let selected_video = use_state(|| None);
        let on_video_select = 
        {
            let selected_video = selected_video.clone();
            Callback::from(move |video: video::Video| 
            {
                selected_video.set(Some(video))
            })
        };
    
        let details = selected_video.as_ref().map(|video| html! 
        {
            <video::VideoDetails video={video.clone()} />
        });
        //let object = JsValue::from("world");
        //info!("Hello {}", object.as_string().unwrap());
        info!("Hello {}", "world");
        html! 
        {
            <>
                <h1>{ "RustConf Explorer" }</h1>
                <div>
                    <h3>{"Videos to watch"}</h3>
                    <VideosList videos={video::get_test_videos()} on_click={on_video_select.clone()} />
                </div>
                { for details }
            </>
        }
}

fn main() 
{
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}