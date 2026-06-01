use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct UrlInputProps {
    pub value: AttrValue,
    pub oninput: Callback<InputEvent>,
}

#[function_component(UrlInput)]
pub fn url_input(props: &UrlInputProps) -> Html {
    html! {
        <div class="input-group">
            <label>{"URL"}</label>
            <input type="text" placeholder="Enter URL"
                value={props.value.clone()}
                oninput={props.oninput.clone()}
            />
        </div>
    }
}
