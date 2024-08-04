use yew::{classes, function_component, html, Classes, Html, Properties};

#[derive(PartialEq)]
pub enum Alignment {
    Horizontal,
    Vertical,
}

#[derive(Properties, PartialEq)]
pub struct InfoboxProps {
    pub label: String,

    #[prop_or_default]
    pub children: Html,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub proposition: String,

    #[prop_or_default]
    pub dense: bool,

    #[prop_or(Alignment::Horizontal)]
    pub align: Alignment,

    #[prop_or("0.75cm".to_string())]
    pub min_height: String,

    #[prop_or("col-6".to_string())]
    pub size_left: String,

    #[prop_or("col-6".to_string())]
    pub size_right: String,
}

#[function_component]
pub fn Infobox(props: &InfoboxProps) -> Html {
    let label = props.label.clone();
    let proposition = props.proposition.clone();

    let value_line_style = format!("min-height: {}", props.min_height);

    match &props.align {
        Alignment::Horizontal => {
            let class = classes!("row", "infobox", props.class.clone());

            let mut base_classes = classes!("box");
            if props.dense {
                base_classes.push("p-0");
            }

            let mut class_left = base_classes.clone();
            class_left.push(&props.size_left);
            let mut class_right = base_classes.clone();
            class_right.push(&props.size_right);
            class_right.push("boxContent");

            html! {
                <div {class} style={value_line_style}>
                    <div class={class_left}>
                        {label.clone()}
                    </div>
                    <div class={class_right}>
                        {props.children.clone()}
                        <span class={classes!("proposition")}>{proposition}</span>
                    </div>
                </div>
            }
        }
        Alignment::Vertical => {
            let class = classes!("infobox", props.class.clone());
            html! {
                <div {class}>
                    <div class={classes!("box")}>
                        {label.clone()}
                    </div>
                    <div class={classes!("box", "boxContent")} style={value_line_style}>
                        {props.children.clone()}
                        <span class={classes!("proposition")}>{proposition}</span>
                    </div>
                </div>
            }
        }
    }
}
