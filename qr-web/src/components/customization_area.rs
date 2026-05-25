use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CustomizationAreaProps {
    pub ec_level: AttrValue,
    pub fg_color: AttrValue,
    pub bg_color: AttrValue,
    pub on_ec_change: Callback<Event>,
    pub on_fg_input: Callback<InputEvent>,
    pub on_bg_input: Callback<InputEvent>,
}

#[function_component(CustomizationArea)]
pub fn customization_area(props: &CustomizationAreaProps) -> Html {
    html! {
        <div class="customization-area" style="margin-top: 20px; padding: 15px; background: #eee; border-radius: 8px;">
            <h3>{"Customization"}</h3>
            <div class="input-group">
                <label>{"Error Correction Level"}</label>
                <select onchange={props.on_ec_change.clone()}>
                    <option value="L" selected={props.ec_level == "L"}>{"Low (7%)"}</option>
                    <option value="M" selected={props.ec_level == "M"}>{"Medium (15%)"}</option>
                    <option value="Q" selected={props.ec_level == "Q"}>{"Quartile (25%)"}</option>
                    <option value="H" selected={props.ec_level == "H"}>{"High (30%)"}</option>
                </select>
            </div>
            <div class="input-group">
                <label>{"Foreground Color"}</label>
                <input type="color" value={props.fg_color.clone()}
                    oninput={props.on_fg_input.clone()}
                />
            </div>
             <div class="input-group">
                <label>{"Background Color"}</label>
                <input type="color" value={props.bg_color.clone()}
                    oninput={props.on_bg_input.clone()}
                />
            </div>
        </div>
    }
}
