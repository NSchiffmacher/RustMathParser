use yew::prelude::*;


#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub value: String,
    pub on_click: Callback<String>,
}

#[function_component(Button)]
pub fn button(ButtonProps { value, on_click }: &ButtonProps) -> Html {
    let callback = {
        let on_click = on_click.clone();
        let value = value.clone();
        Callback::from(move |_| {
            on_click.emit(value.clone());
        })
    };
    

    html! {
        <td>
            <input type="button" value={value.clone()} onclick={ callback }/>
        </td>
    }
}