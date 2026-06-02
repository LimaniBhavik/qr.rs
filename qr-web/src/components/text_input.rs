use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TextInputProps {
    pub value: AttrValue,
    pub oninput: Callback<InputEvent>,
}

#[function_component(TextInput)]
pub fn text_input(props: &TextInputProps) -> Html {
    html! {
        <div class="input-group">
            <label>{"Text"}</label>
            <textarea placeholder="Enter Text"
                value={props.value.clone()}
                oninput={props.oninput.clone()}
            />
        </div>
    }
}
