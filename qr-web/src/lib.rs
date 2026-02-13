use base64::{engine::general_purpose, Engine as _};
use qr_rs::{ContactData, QRData, QRGenerator};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Mode {
    Url,
    Text,
    Contact,
}

#[function_component(QRWeb)]
pub fn qr_web() -> Html {
    let mode = use_state(|| Mode::Url);
    let url_input = use_state(String::new);
    let text_input = use_state(String::new);
    let contact = use_state(ContactData::default);

    let qr_data_url = {
        let mode = *mode;
        let url = (*url_input).clone();
        let text = (*text_input).clone();
        let contact_data = (*contact).clone();

        let generator = QRGenerator::new();
        let data = match mode {
            Mode::Url => QRData::URL(url),
            Mode::Text => QRData::Text(text),
            Mode::Contact => QRData::Contact(contact_data),
        };

        if let Ok(qr) = generator.generate(&data) {
            if let Ok(bytes) = generator.to_png(&qr, 300) {
                let b64 = general_purpose::STANDARD.encode(&bytes);
                Some(format!("data:image/png;base64,{}", b64))
            } else {
                None
            }
        } else {
            None
        }
    };

    let on_mode_url = {
        let mode = mode.clone();
        Callback::from(move |_| mode.set(Mode::Url))
    };
    let on_mode_text = {
        let mode = mode.clone();
        Callback::from(move |_| mode.set(Mode::Text))
    };
    let on_mode_contact = {
        let mode = mode.clone();
        Callback::from(move |_| mode.set(Mode::Contact))
    };

    html! {
        <div class="app-container">
            <header>
                <h1>{"QR.RS"}</h1>
                <p>{"Quantum Response Generator"}</p>
            </header>

            <div class="mode-selector">
                <button onclick={on_mode_url} class={if *mode == Mode::Url { "active" } else { "" }}>{"URL"}</button>
                <button onclick={on_mode_text} class={if *mode == Mode::Text { "active" } else { "" }}>{"Text"}</button>
                <button onclick={on_mode_contact} class={if *mode == Mode::Contact { "active" } else { "" }}>{"Contact"}</button>
            </div>

            <div class="input-area">
                if *mode == Mode::Url {
                    <div class="input-group">
                        <label>{"URL"}</label>
                        <input type="text" placeholder="Enter URL"
                            value={(*url_input).clone()}
                            oninput={
                                let url_input = url_input.clone();
                                Callback::from(move |e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    url_input.set(input.value());
                                })
                            }
                        />
                    </div>
                } else if *mode == Mode::Text {
                    <div class="input-group">
                        <label>{"Text"}</label>
                        <textarea placeholder="Enter Text"
                            value={(*text_input).clone()}
                            oninput={
                                let text_input = text_input.clone();
                                Callback::from(move |e: InputEvent| {
                                    let input: HtmlTextAreaElement = e.target_unchecked_into();
                                    text_input.set(input.value());
                                })
                            }
                        />
                    </div>
                } else {
                    <div class="contact-form">
                         <div class="input-group">
                            <label>{"First Name"}</label>
                            <input type="text" value={contact.first_name.clone()}
                                oninput={let contact = contact.clone(); Callback::from(move |e: InputEvent| {
                                    let val = e.target_unchecked_into::<HtmlInputElement>().value();
                                    let mut c = (*contact).clone(); c.first_name = val; contact.set(c);
                                })} />
                         </div>
                         <div class="input-group">
                            <label>{"Last Name"}</label>
                            <input type="text" value={contact.last_name.clone()}
                                oninput={let contact = contact.clone(); Callback::from(move |e: InputEvent| {
                                    let val = e.target_unchecked_into::<HtmlInputElement>().value();
                                    let mut c = (*contact).clone(); c.last_name = val; contact.set(c);
                                })} />
                         </div>
                         <div class="input-group">
                            <label>{"Email"}</label>
                            <input type="email" value={contact.email.clone()}
                                oninput={let contact = contact.clone(); Callback::from(move |e: InputEvent| {
                                    let val = e.target_unchecked_into::<HtmlInputElement>().value();
                                    let mut c = (*contact).clone(); c.email = val; contact.set(c);
                                })} />
                         </div>
                         <div class="input-group">
                            <label>{"Phone"}</label>
                            <input type="tel" value={contact.phone.clone()}
                                oninput={let contact = contact.clone(); Callback::from(move |e: InputEvent| {
                                    let val = e.target_unchecked_into::<HtmlInputElement>().value();
                                    let mut c = (*contact).clone(); c.phone = val; contact.set(c);
                                })} />
                         </div>
                         <div class="input-group">
                            <label>{"Organization"}</label>
                            <input type="text" value={contact.organization.clone()}
                                oninput={let contact = contact.clone(); Callback::from(move |e: InputEvent| {
                                    let val = e.target_unchecked_into::<HtmlInputElement>().value();
                                    let mut c = (*contact).clone(); c.organization = val; contact.set(c);
                                })} />
                         </div>
                         <div class="input-group">
                            <label>{"Website"}</label>
                            <input type="url" value={contact.website.clone()}
                                oninput={let contact = contact.clone(); Callback::from(move |e: InputEvent| {
                                    let val = e.target_unchecked_into::<HtmlInputElement>().value();
                                    let mut c = (*contact).clone(); c.website = val; contact.set(c);
                                })} />
                         </div>
                    </div>
                }
            </div>

            if let Some(data_url) = qr_data_url {
                <div class="qr-display">
                    <img src={data_url.clone()} alt="QR Code" style="max-width: 300px; border: 1px solid #ccc;" />
                    <br/>
                    <a href={data_url} download="qr.png" class="download-btn">{"Download PNG"}</a>
                </div>
            }
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<QRWeb>::new().render();
}
