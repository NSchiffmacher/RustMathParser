use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub value: String,
    pub text: String,
    pub on_click: Callback<String>,

    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Button)]
pub fn button(
    ButtonProps {
        value,
        text,
        on_click,
        class,
    }: &ButtonProps,
) -> Html {
    let callback = {
        let on_click = on_click.clone();
        let value = value.clone();
        Callback::from(move |_| {
            on_click.emit(value.clone());
        })
    };

    html! {
        <td>
            <input class={class.clone()} type="button" value={text.clone()} onclick={ callback }/>
        </td>
    }
}
