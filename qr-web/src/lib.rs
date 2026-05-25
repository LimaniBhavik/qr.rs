use base64::{engine::general_purpose, Engine as _};
use qr_rs::utils::parse_hex_color;
use qr_rs::{ContactData, QRBuilder, QRData};
use yew::prelude::*;

pub mod components;

use components::{
    contact_input::ContactState, mode_selector::Mode, ContactInput, CustomizationArea,
    ModeSelector, QrDisplay, TextInput, UrlInput,
};

#[function_component(QRWeb)]
pub fn qr_web() -> Html {
    let mode = use_state(|| Mode::Url);
    let url_input = use_state(|| AttrValue::from(""));
    let text_input = use_state(|| AttrValue::from(""));
    let contact = use_state(ContactState::default);

    // Customization state
    let ec_level = use_state(|| AttrValue::from("H"));
    let fg_color = use_state(|| AttrValue::from("#000000"));
    let bg_color = use_state(|| AttrValue::from("#FFFFFF"));

    let qr_data_url = {
        let mode_val = *mode;
        let url_val = url_input.clone();
        let text_val = text_input.clone();
        let contact_val = contact.clone();
        let ec_val = ec_level.clone();
        let fg_val = fg_color.clone();
        let bg_val = bg_color.clone();

        use_memo(
            (
                mode_val,
                url_val,
                text_val,
                contact_val,
                ec_val,
                fg_val,
                bg_val,
            ),
            |(mode, url, text, contact, ec, fg, bg)| {
                let mut builder = QRBuilder::new();

                // Apply EC level
                let level = match (*ec).as_str() {
                    "L" => qr_rs::qrcode::EcLevel::L,
                    "M" => qr_rs::qrcode::EcLevel::M,
                    "Q" => qr_rs::qrcode::EcLevel::Q,
                    _ => qr_rs::qrcode::EcLevel::H,
                };
                builder = builder.error_correction(level);

                // Apply colors
                if let (Some(fg_rgba), Some(bg_rgba)) = (parse_hex_color(fg), parse_hex_color(bg)) {
                    builder = builder.colors(fg_rgba, bg_rgba);
                }

                let data = match mode {
                    Mode::Url => QRData::URL((*url).to_string()),
                    Mode::Text => QRData::Text((*text).to_string()),
                    Mode::Contact => QRData::Contact(ContactData {
                        first_name: contact.first_name.to_string(),
                        last_name: contact.last_name.to_string(),
                        phone: contact.phone.to_string(),
                        email: contact.email.to_string(),
                        organization: contact.organization.to_string(),
                        website: contact.website.to_string(),
                    }),
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
        )
    };

    let on_mode_change = {
        let mode = mode.clone();
        Callback::from(move |m| mode.set(m))
    };

    html! {
        <div class="app-container">
            <header>
                <h1>{"QR.RS"}</h1>
                <p>{"Quantum Response Generator"}</p>
            </header>

            <ModeSelector mode={*mode} on_change={on_mode_change} />

            <div class="input-area">
                if *mode == Mode::Url {
                    <UrlInput
                        value={(*url_input).clone()}
                        on_change={let url_input = url_input.clone(); Callback::from(move |v| url_input.set(v))}
                    />
                } else if *mode == Mode::Text {
                    <TextInput
                        value={(*text_input).clone()}
                        on_change={let text_input = text_input.clone(); Callback::from(move |v| text_input.set(v))}
                    />
                } else {
                    <ContactInput
                        value={(*contact).clone()}
                        on_change={let contact = contact.clone(); Callback::from(move |v| contact.set(v))}
                    />
                }
            </div>

            <CustomizationArea
                ec_level={(*ec_level).clone()}
                on_ec_level_change={let ec_level = ec_level.clone(); Callback::from(move |v| ec_level.set(v))}
                fg_color={(*fg_color).clone()}
                on_fg_color_change={let fg_color = fg_color.clone(); Callback::from(move |v| fg_color.set(v))}
                bg_color={(*bg_color).clone()}
                on_bg_color_change={let bg_color = bg_color.clone(); Callback::from(move |v| bg_color.set(v))}
            />

            <QrDisplay data_url={(*qr_data_url).clone()} />
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<QRWeb>::new().render();
}
use wasm_bindgen::prelude::*;
