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

    let reset_state = {
        let value_state = value_state.clone();
        Callback::from(move |value: String| {
            value_state.set(value);
        })
    };

    let compute_result = {
        let value_state = value_state.clone();
        Callback::from(move |_| {
            let str_value = (*value_state).clone();
            log!(str_value);
            value_state.set("".to_string());
        })
    };

    let buttons_grid = [
        "1", "2", "3", "/",
        "4", "5", "6", "*",
        "7", "8", "9", "-",
        "0", ".", "=", "+",
    ].map(|btn| if btn == "=" {
        html! { <Button value={btn} text={btn} on_click={compute_result.clone()} /> }
    } else {
        html! { <Button value={btn} text={btn} on_click={add_to_state.clone()} /> }
    })
     .chunks(4)
     .map(|slice| slice.to_vec())
     .map(|row| html! { <tr> { for row } </tr> })
     .collect::<Vec<_>>();

    html! {
    <table id="calculator"> 
        <tr> 
            <td colspan="3"> 
                <input type="text" id="result" value={ (*value_state).clone() }/> 
            </td> 
            <Button value="" text="c" on_click={reset_state} class={ "reset" } />
        </tr> 
  
        { buttons_grid }
    </table> 
    }
}
