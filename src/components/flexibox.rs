use yew::{classes, function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct FlexiboxEntry {
    pub label: String,
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub proposition: String,
}

impl FlexiboxEntry {
    pub fn label_only(label: &str) -> Self {
        FlexiboxEntry {
            label: label.to_string(),
            value: "".to_string(),
            proposition: "".to_string(),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct FlexiboxProps {
    pub entries: Vec<FlexiboxEntry>,
    #[prop_or("1.5cm".to_string())]
    pub min_height: String,
}

#[function_component]
pub fn Flexibox(props: &FlexiboxProps) -> Html {
    let value_line_style = format!("min-height: {}", props.min_height);

    html! {
        <div class={classes!("row", "mt-1")}>
            {
                props.entries.iter().map(|entry| {
                    html! {
                        <div class={classes!("col", "bordered", "p-0")} style={value_line_style.clone()}>
                            {&entry.label }
                        </div>
                    }
                }).collect::<Html>()
            }
        </div>
    }
}
