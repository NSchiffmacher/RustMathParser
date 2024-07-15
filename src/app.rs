use yew::prelude::*;
use gloo_console::log;

use crate::button::Button;

#[function_component(App)]
pub fn app() -> Html {
    let value_state = use_state(|| "".to_string());

    let add_to_state = {
        let value_state = value_state.clone();
        Callback::from(move |value: String| {
            let v = (*value_state).clone();
            value_state.set(v + &value);
        })
    };

    let buttons_grid = [
        "1", "2", "3", "/",
        "4", "5", "6", "*",
        "7", "8", "9", "-",
        "0", ".", "=", "+",
    ].map(|btn| html! { <Button value={btn} on_click={add_to_state.clone()} /> })
     .chunks(4)
     .map(|slice| slice.to_vec())
     .map(|row| html! { <tr> { for row } </tr> })
     .collect::<Vec<_>>();

    html! {
    <table id="calcu"> 
        <tr> 
            <td colspan="3"> 
                <input type="text" id="result" value={ (*value_state).clone() }/> 
            </td> 
            <td><input class="reset" type="button" value="c"/></td> 
        </tr> 
  
        { buttons_grid }
    </table> 
    }
}
