use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct DefendControlsProps {
    pub on_left: Callback<bool>,
    pub on_right: Callback<bool>,
    pub on_fire: Callback<bool>,
    pub is_charging: bool,
    pub charge_level: f64,
}

#[function_component(DefendControls)]
pub fn defend_controls(props: &DefendControlsProps) -> Html {
    let on_left = props.on_left.clone();
    let on_right = props.on_right.clone();
    let on_fire = props.on_fire.clone();

    // Style the fire button based on charge level
    let fire_style = if props.is_charging {
        let percent = (props.charge_level * 100.0) as u32;
        let color = if props.charge_level >= 1.0 {
            "rgba(255, 235, 59, 0.4)" // bright yellow glow
        } else {
            "rgba(224, 93, 68, 0.3)" // soft red glow
        };
        format!(
            "background: linear-gradient(90deg, {} {}%, rgba(0,0,0,0.2) {}%)",
            color, percent, percent
        )
    } else {
        "".to_string()
    };

    let fire_class = if props.charge_level >= 1.0 {
        "btn-touch control-fire active fully-charged"
    } else {
        "btn-touch control-fire active"
    };

    html! {
        <div class="mode-toggles">
            <button
                class="btn-touch control-left"
                onmousedown={
                    let on_left = on_left.clone();
                    Callback::from(move |e: MouseEvent| { e.prevent_default(); on_left.emit(true); })
                }
                onmouseup={
                    let on_left = on_left.clone();
                    Callback::from(move |_| on_left.emit(false))
                }
                onmouseleave={
                    let on_left = on_left.clone();
                    Callback::from(move |_| on_left.emit(false))
                }
                ontouchstart={
                    let on_left = on_left.clone();
                    Callback::from(move |e: TouchEvent| { e.prevent_default(); on_left.emit(true); })
                }
                ontouchend={
                    let on_left = on_left.clone();
                    Callback::from(move |_| on_left.emit(false))
                }
            >
                { "◀" }
            </button>
            <button
                class="btn-touch control-right"
                onmousedown={
                    let on_right = on_right.clone();
                    Callback::from(move |e: MouseEvent| { e.prevent_default(); on_right.emit(true); })
                }
                onmouseup={
                    let on_right = on_right.clone();
                    Callback::from(move |_| on_right.emit(false))
                }
                onmouseleave={
                    let on_right = on_right.clone();
                    Callback::from(move |_| on_right.emit(false))
                }
                ontouchstart={
                    let on_right = on_right.clone();
                    Callback::from(move |e: TouchEvent| { e.prevent_default(); on_right.emit(true); })
                }
                ontouchend={
                    let on_right = on_right.clone();
                    Callback::from(move |_| on_right.emit(false))
                }
            >
                { "▶" }
            </button>
            <button
                class={fire_class}
                style={fire_style}
                onmousedown={
                    let on_fire = on_fire.clone();
                    Callback::from(move |e: MouseEvent| { e.prevent_default(); on_fire.emit(true); })
                }
                onmouseup={
                    let on_fire = on_fire.clone();
                    Callback::from(move |_| on_fire.emit(false))
                }
                onmouseleave={
                    let on_fire = on_fire.clone();
                    Callback::from(move |_| on_fire.emit(false))
                }
                ontouchstart={
                    let on_fire = on_fire.clone();
                    Callback::from(move |e: TouchEvent| { e.prevent_default(); on_fire.emit(true); })
                }
                ontouchend={
                    let on_fire = on_fire.clone();
                    Callback::from(move |_| on_fire.emit(false))
                }
            >
                if props.charge_level >= 1.0 {
                    { "💥 RELEASE!" }
                } else if props.is_charging {
                    { "⚡ CHARGING" }
                } else {
                    { "🔥 FIRE" }
                }
            </button>
        </div>
    }
}
