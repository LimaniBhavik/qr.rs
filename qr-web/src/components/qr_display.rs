use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct QrDisplayProps {
    pub data_url: Option<String>,
}

#[function_component(QrDisplay)]
pub fn qr_display(props: &QrDisplayProps) -> Html {
    if let Some(url) = &props.data_url {
        html! {
            <div class="qr-display">
                <img src={url.clone()} alt="QR Code" style="max-width: 300px; border: 1px solid #ccc;" />
                <br/>
                <a href={url.clone()} download="qr.png" class="download-btn">{"Download PNG"}</a>
            </div>
        }
    } else {
        html! {}
    }
}
