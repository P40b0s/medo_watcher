use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct Video 
{
    id: usize,
    title: String,
    speaker: String,
    url: String,
}
impl PartialEq for Video
{
    fn eq(&self, other: &Self) -> bool 
    {
        self.id == other.id
    }
}

#[derive(Properties, PartialEq)]
pub struct VideosListProps 
{
    pub videos: Vec<Video>,
    pub on_click: Callback<Video>
}

#[function_component(VideosList)]
pub fn videos_list(VideosListProps { videos, on_click }: &VideosListProps) -> Html 
{
    let on_click = on_click.clone();
    videos.iter().map(|video| 
    {
        let on_video_select = 
        {
            let on_click = on_click.clone();
            let video = video.clone();
            Callback::from(move |_| 
            {
                on_click.emit(video.clone())
            })
        };

    html! 
    {
        <p key={video.id} onclick={on_video_select}>{format!("{}: {}", video.speaker, video.title)}</p>
    }
    }).collect()
}

pub fn get_test_videos() -> Vec<Video>
{
    vec![
        Video {
            id: 1,
            title: "Building and breaking things".to_string(),
            speaker: "John Doe".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 2,
            title: "The development process".to_string(),
            speaker: "Jane Smith".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 3,
            title: "The Web 7.0".to_string(),
            speaker: "Matt Miller".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 4,
            title: "Mouseless development".to_string(),
            speaker: "Tom Jerry".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
    ]
}






#[derive(Properties, PartialEq)]
pub struct VideosDetailsProps 
{
    pub video: Video,
}

#[function_component(VideoDetails)]
pub fn video_details(VideosDetailsProps { video }: &VideosDetailsProps) -> Html 
{
    html! 
    {
        <div>
            <h3>{ video.title.clone() }</h3>
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
    }
}