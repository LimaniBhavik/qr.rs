use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct QrDisplayProps {
    pub data_url: AttrValue,
}

#[function_component(QrDisplay)]
pub fn qr_display(props: &QrDisplayProps) -> Html {
    html! {
        <div class="qr-display">
            <img src={props.data_url.clone()} alt="QR Code" style="max-width: 300px; border: 1px solid #ccc;" />
            <br/>
            <a href={props.data_url.clone()} download="qr.png" class="download-btn">{"Download PNG"}</a>
        </div>
    }
}
