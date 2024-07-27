use gloo_console::log;
use yew::prelude::*;

use crate::button::Button;
use crate::parser::parse;

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
            let result = parse(&str_value);

            let str_result = match result {
                Ok(expr) => format!("{:?}", expr),
                Err(err) => format!("Error: {}", err),
            };

            value_state.set(str_result);
        })
    };

    let buttons_grid = vec![
        vec!["1", "2", "3", "/"], 
        vec!["4", "5", "6", "*"], 
        vec!["7", "8", "9", "-"], 
        vec!["0", ".", "=", "+"],
    ]
    .into_iter()
    .map(|row| {
        let btns_html: Vec<_> = row.iter().map(|btn| {
            if *btn == "=" {
                html! { <Button value={*btn} text={*btn} on_click={compute_result.clone()} /> }
            } else {
                html! { <Button value={*btn} text={*btn} on_click={add_to_state.clone()} /> }
            }
        }).collect();
        html! { <tr> { for btns_html } </tr> }
    })
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
