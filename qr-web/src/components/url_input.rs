use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct UrlInputProps {
    pub value: AttrValue,
    pub on_change: Callback<AttrValue>,
}

#[function_component(UrlInput)]
pub fn url_input(props: &UrlInputProps) -> Html {
    let oninput = {
        let on_change = props.on_change.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            on_change.emit(AttrValue::from(input.value()));
        })
    };

    html! {
        <div class="input-group">
            <label>{"URL"}</label>
            <input type="text" placeholder="Enter URL"
                value={props.value.clone()}
                {oninput}
            />
        </div>
    }
}
