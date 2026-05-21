use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, PartialEq, Default)]
pub struct ContactState {
    pub first_name: AttrValue,
    pub last_name: AttrValue,
    pub phone: AttrValue,
    pub email: AttrValue,
    pub organization: AttrValue,
    pub website: AttrValue,
}

#[derive(Properties, PartialEq)]
pub struct ContactInputProps {
    pub value: ContactState,
    pub on_change: Callback<ContactState>,
}

#[function_component(ContactInput)]
pub fn contact_input(props: &ContactInputProps) -> Html {
    let oninput_factory = |field: fn(&mut ContactState, AttrValue)| {
        let value = props.value.clone();
        let on_change = props.on_change.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut new_state = value.clone();
            field(&mut new_state, AttrValue::from(input.value()));
            on_change.emit(new_state);
        })
    };

    let oninput_first_name = oninput_factory(|s, v| s.first_name = v);
    let oninput_last_name = oninput_factory(|s, v| s.last_name = v);
    let oninput_email = oninput_factory(|s, v| s.email = v);
    let oninput_phone = oninput_factory(|s, v| s.phone = v);
    let oninput_org = oninput_factory(|s, v| s.organization = v);
    let oninput_website = oninput_factory(|s, v| s.website = v);

    html! {
        <div class="contact-form">
             <div class="input-group">
                <label>{"First Name"}</label>
                <input type="text" value={props.value.first_name.clone()} oninput={oninput_first_name} />
             </div>
             <div class="input-group">
                <label>{"Last Name"}</label>
                <input type="text" value={props.value.last_name.clone()} oninput={oninput_last_name} />
             </div>
             <div class="input-group">
                <label>{"Email"}</label>
                <input type="email" value={props.value.email.clone()} oninput={oninput_email} />
             </div>
             <div class="input-group">
                <label>{"Phone"}</label>
                <input type="tel" value={props.value.phone.clone()} oninput={oninput_phone} />
             </div>
             <div class="input-group">
                <label>{"Organization"}</label>
                <input type="text" value={props.value.organization.clone()} oninput={oninput_org} />
             </div>
             <div class="input-group">
                <label>{"Website"}</label>
                <input type="url" value={props.value.website.clone()} oninput={oninput_website} />
             </div>
        </div>
    }
}
