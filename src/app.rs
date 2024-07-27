use yew::prelude::*;

use crate::button::Button;
use crate::parser::parse;

#[function_component(App)]
pub fn app() -> Html {
    let value_state = use_state(|| "".to_string());
    let allow_input = use_state(|| true);

    let add_to_state = {
        let value_state = value_state.clone();
        let allow_input = allow_input.clone();
        Callback::from(move |value: String| {
            if !*allow_input {
                return;
            }

            let v = (*value_state).clone();
            value_state.set(v + &value);
        })
    };

    let clear = {
        let value_state = value_state.clone();
        let allow_input = allow_input.clone();
        Callback::from(move |value: String| {
            if !*allow_input {
                allow_input.set(true);
            }

            value_state.set(value);
        })
    };

    let clear_entry = {
        let value_state = value_state.clone();
        let allow_input = allow_input.clone();
        Callback::from(move |_: String| match *allow_input {
            true => {
                let mut v = (*value_state).clone();
                v.pop();
                value_state.set(v);
            }
            false => {
                value_state.set("".to_string());
                allow_input.set(true);
            }
        })
    };

    let compute_result = {
        let value_state = value_state.clone();
        let allow_input = allow_input.clone();
        Callback::from(move |_| {
            if !*allow_input {
                return;
            }

            let str_value = (*value_state).clone();
            let result = parse(&str_value);

            let str_result = match result {
                Ok(expr) => format!("{:?}", expr),
                Err(err) => {
                    allow_input.set(false);
                    format!("Error: {}", err)
                }
            };

            value_state.set(str_result);
        })
    };

    let buttons_grid = vec![
        vec!["(", ")", "%", "CE"], 
        vec!["7", "8", "9", "/"], 
        vec!["4", "5", "6", "*"], 
        vec!["1", "2", "3", "-"], 
        vec!["0", "^", "=", "+"],
    ]
    .into_iter()
    .map(|row| {
        let btns_html: Vec<_> = row.iter().map(|btn| {
            if *btn == "=" {
                html! { <Button value="" text={*btn} on_click={compute_result.clone()} /> }
            } else if *btn == "CE" {
                html! { <Button value="" text={*btn} on_click={clear_entry.clone()} class={ "reset" } />}
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
            <Button value="" text="C" on_click={clear} class={ "reset" } />
        </tr>

        { buttons_grid }
    </table>
    }
}
