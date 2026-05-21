use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TextInputProps {
    pub value: AttrValue,
    pub on_change: Callback<AttrValue>,
}

#[function_component(TextInput)]
pub fn text_input(props: &TextInputProps) -> Html {
    let oninput = {
        let on_change = props.on_change.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            on_change.emit(AttrValue::from(input.value()));
        })
    };

    html! {
        <div class="input-group">
            <label>{"Text"}</label>
            <textarea placeholder="Enter Text"
                value={props.value.clone()}
                {oninput}
            />
        </div>
    }
}
