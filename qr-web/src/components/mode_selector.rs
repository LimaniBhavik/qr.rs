use yew::prelude::*;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Mode {
    Url,
    Text,
    Contact,
}

#[derive(Properties, PartialEq)]
pub struct ModeSelectorProps {
    pub mode: Mode,
    pub on_change: Callback<Mode>,
}

#[function_component(ModeSelector)]
pub fn mode_selector(props: &ModeSelectorProps) -> Html {
    let on_mode_url = {
        let on_change = props.on_change.clone();
        Callback::from(move |_| on_change.emit(Mode::Url))
    };
    let on_mode_text = {
        let on_change = props.on_change.clone();
        Callback::from(move |_| on_change.emit(Mode::Text))
    };
    let on_mode_contact = {
        let on_change = props.on_change.clone();
        Callback::from(move |_| on_change.emit(Mode::Contact))
    };

    html! {
        <div class="mode-selector">
            <button onclick={on_mode_url} class={if props.mode == Mode::Url { "active" } else { "" }}>{"URL"}</button>
            <button onclick={on_mode_text} class={if props.mode == Mode::Text { "active" } else { "" }}>{"Text"}</button>
            <button onclick={on_mode_contact} class={if props.mode == Mode::Contact { "active" } else { "" }}>{"Contact"}</button>
        </div>
    }
}
