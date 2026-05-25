use base64::{engine::general_purpose, Engine as _};
use qr_rs::utils::parse_hex_color;
use qr_rs::{QRBuilder, QRData};
use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;
use yew::prelude::*;

mod components;
use components::*;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Mode {
    Url,
    Text,
    Contact,
}

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
        let url_val = (*url_input).clone();
        let text_val = (*text_input).clone();
        let contact_val = (*contact).clone();
        let ec_val = (*ec_level).clone();
        let fg_val = (*fg_color).clone();
        let bg_val = (*bg_color).clone();

        use_memo(
            (mode_val, url_val, text_val, contact_val, ec_val, fg_val, bg_val),
            |(m, u, t, c, ec, fg, bg)| {
                let mut builder = QRBuilder::new();

                // Apply EC level
                let level = match ec.as_str() {
                    "L" => qr_rs::qrcode::EcLevel::L,
                    "M" => qr_rs::qrcode::EcLevel::M,
                    "Q" => qr_rs::qrcode::EcLevel::Q,
                    _ => qr_rs::qrcode::EcLevel::H,
                };
                builder = builder.error_correction(level);

                // Apply colors
                if let (Some(fg_rgba), Some(bg_rgba)) = (parse_hex_color(fg.as_str()), parse_hex_color(bg.as_str())) {
                    builder = builder.colors(fg_rgba, bg_rgba);
                }

                let data = match m {
                    Mode::Url => QRData::URL(u.to_string()),
                    Mode::Text => QRData::Text(t.to_string()),
                    Mode::Contact => QRData::Contact(c.to_contact_data()),
                };

                builder = builder.data(data);

                if let Ok(generator) = builder.build() {
                    if let Ok(bytes) = generator.to_png(300, None) {
                        let b64 = general_purpose::STANDARD.encode(&bytes);
                        Some(AttrValue::from(format!("data:image/png;base64,{}", b64)))
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
        )
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

    let on_url_input = {
        let url_input = url_input.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            url_input.set(AttrValue::from(input.value()));
        })
    };

    let on_text_input = {
        let text_input = text_input.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            text_input.set(AttrValue::from(input.value()));
        })
    };

    let on_contact_update = {
        let contact = contact.clone();
        Callback::from(move |c: ContactState| {
            contact.set(c);
        })
    };

    let on_ec_change = {
        let ec_level = ec_level.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            ec_level.set(AttrValue::from(input.value()));
        })
    };

    let on_fg_input = {
        let fg_color = fg_color.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            fg_color.set(AttrValue::from(input.value()));
        })
    };

    let on_bg_input = {
        let bg_color = bg_color.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            bg_color.set(AttrValue::from(input.value()));
        })
    };

    html! {
        <div class="app-container">
            <header>
                <h1>{"QR.RS"}</h1>
                <p>{"Quantum Response Generator"}</p>
            </header>

            <ModeSelector
                current_mode={*mode}
                on_mode_url={on_mode_url}
                on_mode_text={on_mode_text}
                on_mode_contact={on_mode_contact}
            />

            <div class="input-area">
                if *mode == Mode::Url {
                    <UrlInput value={(*url_input).clone()} oninput={on_url_input} />
                } else if *mode == Mode::Text {
                    <TextInput value={(*text_input).clone()} oninput={on_text_input} />
                } else {
                    <ContactInput contact={(*contact).clone()} on_update={on_contact_update} />
                }
            </div>

            <CustomizationArea
                ec_level={(*ec_level).clone()}
                fg_color={(*fg_color).clone()}
                bg_color={(*bg_color).clone()}
                on_ec_change={on_ec_change}
                on_fg_input={on_fg_input}
                on_bg_input={on_bg_input}
            />

            if let Some(data_url) = (*qr_data_url).clone() {
                <QrDisplay data_url={data_url} />
            }
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<QRWeb>::new().render();
}
