use yew::{classes, function_component, html, Classes, Html, Properties};

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

    pub fn label_and_proposition(label: &str, proposition: &str) -> Self {
        FlexiboxEntry {
            label: label.to_string(),
            value: "".to_string(),
            proposition: proposition.to_string(),
        }
    }

    pub fn label_and_value(label: &str, value: &str) -> Self {
        FlexiboxEntry {
            label: label.to_string(),
            value: value.to_string(),
            proposition: "".to_string(),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct FlexiboxProps {
    pub entries: Vec<FlexiboxEntry>,
    #[prop_or("1.5cm".to_string())]
    pub min_height: String,

    #[prop_or_default]
    pub class: Classes,
}

#[function_component]
pub fn Flexibox(props: &FlexiboxProps) -> Html {
    let value_line_style = format!("min-height: {}", props.min_height);

    let mut class = props.class.clone();
    class.push("row");
    class.push("mt-1");
    class.push("mx-0");

    html! {
        <div {class}>
            {
                props.entries.iter().map(|entry| {
                    html! {
                        <div class={classes!("col", "bordered", "p-0", "box")} style={value_line_style.clone()}>
                            {&entry.label }
                            <span class={classes!("proposition")}>{&entry.proposition}</span>
                        </div>
                    }
                }).collect::<Html>()
            }
        </div>
    }
}
