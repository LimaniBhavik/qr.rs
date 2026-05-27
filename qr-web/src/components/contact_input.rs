use qr_rs::ContactData;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct ContactState {
    pub first_name: AttrValue,
    pub last_name: AttrValue,
    pub phone: AttrValue,
    pub email: AttrValue,
    pub organization: AttrValue,
    pub website: AttrValue,
}

impl ContactState {
    pub fn to_contact_data(&self) -> ContactData {
        ContactData {
            first_name: self.first_name.to_string(),
            last_name: self.last_name.to_string(),
            phone: self.phone.to_string(),
            email: self.email.to_string(),
            organization: self.organization.to_string(),
            website: self.website.to_string(),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ContactInputProps {
    pub contact: ContactState,
    pub on_update: Callback<ContactState>,
}

#[function_component(ContactInput)]
pub fn contact_input(props: &ContactInputProps) -> Html {
    let on_first_name = {
        let contact = props.contact.clone();
        let on_update = props.on_update.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            on_update.emit(ContactState {
                first_name: AttrValue::from(input.value()),
                last_name: contact.last_name.clone(),
                phone: contact.phone.clone(),
                email: contact.email.clone(),
                organization: contact.organization.clone(),
                website: contact.website.clone(),
            });
        })
    };

    let on_last_name = {
        let contact = props.contact.clone();
        let on_update = props.on_update.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            on_update.emit(ContactState {
                last_name: AttrValue::from(input.value()),
                first_name: contact.first_name.clone(),
                phone: contact.phone.clone(),
                email: contact.email.clone(),
                organization: contact.organization.clone(),
                website: contact.website.clone(),
            });
        })
    };

    let on_email = {
        let contact = props.contact.clone();
        let on_update = props.on_update.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            on_update.emit(ContactState {
                email: AttrValue::from(input.value()),
                first_name: contact.first_name.clone(),
                last_name: contact.last_name.clone(),
                phone: contact.phone.clone(),
                organization: contact.organization.clone(),
                website: contact.website.clone(),
            });
        })
    };

    let on_phone = {
        let contact = props.contact.clone();
        let on_update = props.on_update.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            on_update.emit(ContactState {
                phone: AttrValue::from(input.value()),
                first_name: contact.first_name.clone(),
                last_name: contact.last_name.clone(),
                email: contact.email.clone(),
                organization: contact.organization.clone(),
                website: contact.website.clone(),
            });
        })
    };

    let on_org = {
        let contact = props.contact.clone();
        let on_update = props.on_update.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            on_update.emit(ContactState {
                organization: AttrValue::from(input.value()),
                first_name: contact.first_name.clone(),
                last_name: contact.last_name.clone(),
                phone: contact.phone.clone(),
                email: contact.email.clone(),
                website: contact.website.clone(),
            });
        })
    };

    let on_website = {
        let contact = props.contact.clone();
        let on_update = props.on_update.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            on_update.emit(ContactState {
                website: AttrValue::from(input.value()),
                first_name: contact.first_name.clone(),
                last_name: contact.last_name.clone(),
                phone: contact.phone.clone(),
                email: contact.email.clone(),
                organization: contact.organization.clone(),
            });
        })
    };

    html! {
        <div class="contact-form">
            <div class="input-group">
                <label>{"First Name"}</label>
                <input type="text" value={props.contact.first_name.clone()} oninput={on_first_name} />
            </div>
            <div class="input-group">
                <label>{"Last Name"}</label>
                <input type="text" value={props.contact.last_name.clone()} oninput={on_last_name} />
            </div>
            <div class="input-group">
                <label>{"Email"}</label>
                <input type="email" value={props.contact.email.clone()} oninput={on_email} />
            </div>
            <div class="input-group">
                <label>{"Phone"}</label>
                <input type="tel" value={props.contact.phone.clone()} oninput={on_phone} />
            </div>
            <div class="input-group">
                <label>{"Organization"}</label>
                <input type="text" value={props.contact.organization.clone()} oninput={on_org} />
            </div>
            <div class="input-group">
                <label>{"Website"}</label>
                <input type="url" value={props.contact.website.clone()} oninput={on_website} />
            </div>
        </div>
    }
}
