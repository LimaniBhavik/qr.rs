use yew::prelude::*;
use crate::Mode;

#[derive(Properties, PartialEq)]
pub struct ModeSelectorProps {
    pub current_mode: Mode,
    pub on_mode_url: Callback<MouseEvent>,
    pub on_mode_text: Callback<MouseEvent>,
    pub on_mode_contact: Callback<MouseEvent>,
}

#[function_component(ModeSelector)]
pub fn mode_selector(props: &ModeSelectorProps) -> Html {
    html! {
        <div class="mode-selector">
            <button onclick={props.on_mode_url.clone()} class={if props.current_mode == Mode::Url { "active" } else { "" }}>{"URL"}</button>
            <button onclick={props.on_mode_text.clone()} class={if props.current_mode == Mode::Text { "active" } else { "" }}>{"Text"}</button>
            <button onclick={props.on_mode_contact.clone()} class={if props.current_mode == Mode::Contact { "active" } else { "" }}>{"Contact"}</button>
        </div>
    }
}
