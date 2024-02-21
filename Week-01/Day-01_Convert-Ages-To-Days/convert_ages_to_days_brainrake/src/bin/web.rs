use yew::prelude::*;
use yew::events::InputEvent;
use web_sys::HtmlInputElement;
use convert_ages_to_days_brainrake::years_to_days;


#[function_component]
fn App() -> Html {
    let state = use_state(|| String::new());
    let result = state.to_string().parse().map(years_to_days);
    let oninput = {
        let state_ = state.clone();
        Callback::from(move |event: InputEvent| state_.set({
            let input: HtmlInputElement = event.target_unchecked_into();
            String::from(input.value())
        }))
    };
    html! {
        <div>
            <p>{ "Please enter your age in years:" }</p>
            <p><input value={state.to_string()} oninput={oninput} /></p>
            <p>
                if let Ok(days) = result {
                    { "You are around "}{days}{" days old." }
                } else {
                    { "Please enter a positive integer." }
                }
            </p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
