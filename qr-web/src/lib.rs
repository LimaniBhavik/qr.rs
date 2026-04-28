use base64::{engine::general_purpose, Engine as _};
use qr_rs::utils::parse_hex_color;
use qr_rs::{ContactData, QRBuilder, QRData};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Mode {
    Url,
    Text,
    Contact,
}

#[derive(PartialEq, Clone, Default, Debug)]
struct ContactFields {
    pub first_name: AttrValue,
    pub last_name: AttrValue,
    pub phone: AttrValue,
    pub email: AttrValue,
    pub organization: AttrValue,
    pub website: AttrValue,
}

impl From<&ContactFields> for ContactData {
    fn from(fields: &ContactFields) -> Self {
        Self {
            first_name: fields.first_name.to_string(),
            last_name: fields.last_name.to_string(),
            phone: fields.phone.to_string(),
            email: fields.email.to_string(),
            organization: fields.organization.to_string(),
            website: fields.website.to_string(),
        }
    }
}

#[function_component(QRWeb)]
pub fn qr_web() -> Html {
    let mode = use_state(|| Mode::Url);
    let url_input = use_state(AttrValue::default);
    let text_input = use_state(AttrValue::default);
    let contact = use_state(ContactFields::default);

    // Customization state
    let ec_level = use_state(|| AttrValue::from("H"));
    let fg_color = use_state(|| AttrValue::from("#000000"));
    let bg_color = use_state(|| AttrValue::from("#FFFFFF"));

    let qr_data_url = use_memo(
        (
            *mode,
            url_input.clone(),
            text_input.clone(),
            contact.clone(),
            ec_level.clone(),
            fg_color.clone(),
            bg_color.clone(),
        ),
        |(mode, url_input, text_input, contact, ec_level, fg_color, bg_color)| {
            let mut builder = QRBuilder::new();

            // Apply EC level
            let level = match ec_level.as_str() {
                "L" => qr_rs::qrcode::EcLevel::L,
                "M" => qr_rs::qrcode::EcLevel::M,
                "Q" => qr_rs::qrcode::EcLevel::Q,
                _ => qr_rs::qrcode::EcLevel::H,
            };
            builder = builder.error_correction(level);

            // Apply colors
            if let (Some(fg_rgba), Some(bg_rgba)) =
                (parse_hex_color(fg_color), parse_hex_color(bg_color))
            {
                builder = builder.colors(fg_rgba, bg_rgba);
            }

            let data = match mode {
                Mode::Url => QRData::URL(url_input.to_string()),
                Mode::Text => QRData::Text(text_input.to_string()),
                Mode::Contact => QRData::Contact(ContactData::from(&**contact)),
            };

            builder = builder.data(data);

            if let Ok(generator) = builder.build() {
                if let Ok(bytes) = generator.to_png(300, None) {
                    let b64 = general_purpose::STANDARD.encode(&bytes);
                    Some(format!("data:image/png;base64,{}", b64))
                } else {
                    None
                }
            } else {
                None
            }
        },
    );

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
                                    url_input.set(AttrValue::from(input.value()));
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
                                    text_input.set(AttrValue::from(input.value()));
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
                                    let mut c = (*contact).clone(); c.first_name = AttrValue::from(val); contact.set(c);
                                })} />
                         </div>
                         <div class="input-group">
                            <label>{"Last Name"}</label>
                            <input type="text" value={contact.last_name.clone()}
                                oninput={let contact = contact.clone(); Callback::from(move |e: InputEvent| {
                                    let val = e.target_unchecked_into::<HtmlInputElement>().value();
                                    let mut c = (*contact).clone(); c.last_name = AttrValue::from(val); contact.set(c);
                                })} />
                         </div>
                         <div class="input-group">
                            <label>{"Email"}</label>
                            <input type="email" value={contact.email.clone()}
                                oninput={let contact = contact.clone(); Callback::from(move |e: InputEvent| {
                                    let val = e.target_unchecked_into::<HtmlInputElement>().value();
                                    let mut c = (*contact).clone(); c.email = AttrValue::from(val); contact.set(c);
                                })} />
                         </div>
                         <div class="input-group">
                            <label>{"Phone"}</label>
                            <input type="tel" value={contact.phone.clone()}
                                oninput={let contact = contact.clone(); Callback::from(move |e: InputEvent| {
                                    let val = e.target_unchecked_into::<HtmlInputElement>().value();
                                    let mut c = (*contact).clone(); c.phone = AttrValue::from(val); contact.set(c);
                                })} />
                         </div>
                         <div class="input-group">
                            <label>{"Organization"}</label>
                            <input type="text" value={contact.organization.clone()}
                                oninput={let contact = contact.clone(); Callback::from(move |e: InputEvent| {
                                    let val = e.target_unchecked_into::<HtmlInputElement>().value();
                                    let mut c = (*contact).clone(); c.organization = AttrValue::from(val); contact.set(c);
                                })} />
                         </div>
                         <div class="input-group">
                            <label>{"Website"}</label>
                            <input type="url" value={contact.website.clone()}
                                oninput={let contact = contact.clone(); Callback::from(move |e: InputEvent| {
                                    let val = e.target_unchecked_into::<HtmlInputElement>().value();
                                    let mut c = (*contact).clone(); c.website = AttrValue::from(val); contact.set(c);
                                })} />
                         </div>
                    </div>
                }
            </div>

            <div class="customization-area" style="margin-top: 20px; padding: 15px; background: #eee; border-radius: 8px;">
                <h3>{"Customization"}</h3>
                <div class="input-group">
                    <label>{"Error Correction Level"}</label>
                    <select onchange={let ec_level = ec_level.clone(); Callback::from(move |e: Event| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        ec_level.set(AttrValue::from(input.value()));
                    })}>
                        <option value="L" selected={*ec_level == "L"}>{"Low (7%)"}</option>
                        <option value="M" selected={*ec_level == "M"}>{"Medium (15%)"}</option>
                        <option value="Q" selected={*ec_level == "Q"}>{"Quartile (25%)"}</option>
                        <option value="H" selected={*ec_level == "H"}>{"High (30%)"}</option>
                    </select>
                </div>
                <div class="input-group">
                    <label>{"Foreground Color"}</label>
                    <input type="color" value={(*fg_color).clone()}
                        oninput={let fg_color = fg_color.clone(); Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            fg_color.set(AttrValue::from(input.value()));
                        })}
                    />
                </div>
                 <div class="input-group">
                    <label>{"Background Color"}</label>
                    <input type="color" value={(*bg_color).clone()}
                        oninput={let bg_color = bg_color.clone(); Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            bg_color.set(AttrValue::from(input.value()));
                        })}
                    />
                </div>
            </div>

            if let Some(data_url) = &*qr_data_url {
                <div class="qr-display">
                    <img src={data_url.clone()} alt="QR Code" style="max-width: 300px; border: 1px solid #ccc;" />
                    <br/>
                    <a href={data_url.clone()} download="qr.png" class="download-btn">{"Download PNG"}</a>
                </div>
            }
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<QRWeb>::new().render();
}
