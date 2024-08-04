use yew::{classes, function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct SectionProps {
    pub label: String,
    pub children: Html,
    #[prop_or("4cm".to_string())]
    pub height: String,
}

#[function_component]
pub fn Section(props: &SectionProps) -> Html {
    let style = format!("height: {}", props.height);

    html! {
        <div class={classes!("container", "mt-3")}>
            <div class={classes!("row", "p-0")}>
                <div class={classes!("col-1", "p-0")} style={"width: 1.2cm;"}>
                    <div class={classes!("sectionLabelContainer")} style={style}>
                        <div class={classes!("sectionLabel")}>
                            {&props.label}
                        </div>
                    </div>
                </div>
                <div class={classes!("col-11", "p-0")}>
                    {props.children.clone()}
                </div>
            </div>
        </div>
    }
}
