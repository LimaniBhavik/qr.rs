use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CustomizationAreaProps {
    pub ec_level: AttrValue,
    pub on_ec_level_change: Callback<AttrValue>,
    pub fg_color: AttrValue,
    pub on_fg_color_change: Callback<AttrValue>,
    pub bg_color: AttrValue,
    pub on_bg_color_change: Callback<AttrValue>,
}

#[function_component(CustomizationArea)]
pub fn customization_area(props: &CustomizationAreaProps) -> Html {
    let on_ec_change = {
        let cb = props.on_ec_level_change.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            cb.emit(AttrValue::from(input.value()));
        })
    };

    let on_fg_change = {
        let cb = props.on_fg_color_change.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            cb.emit(AttrValue::from(input.value()));
        })
    };

    let on_bg_change = {
        let cb = props.on_bg_color_change.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            cb.emit(AttrValue::from(input.value()));
        })
    };

    html! {
        <div class="customization-area" style="margin-top: 20px; padding: 15px; background: #eee; border-radius: 8px;">
            <h3>{"Customization"}</h3>
            <div class="input-group">
                <label>{"Error Correction Level"}</label>
                <select onchange={on_ec_change}>
                    <option value="L" selected={props.ec_level == "L"}>{"Low (7%)"}</option>
                    <option value="M" selected={props.ec_level == "M"}>{"Medium (15%)"}</option>
                    <option value="Q" selected={props.ec_level == "Q"}>{"Quartile (25%)"}</option>
                    <option value="H" selected={props.ec_level == "H"}>{"High (30%)"}</option>
                </select>
            </div>
            <div class="input-group">
                <label>{"Foreground Color"}</label>
                <input type="color" value={props.fg_color.clone()} oninput={on_fg_change} />
            </div>
             <div class="input-group">
                <label>{"Background Color"}</label>
                <input type="color" value={props.bg_color.clone()} oninput={on_bg_change} />
            </div>
        </div>
    }
}
