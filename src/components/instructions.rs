use yew::{classes, function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct InstructionsProps {
    pub lines: i32,
    #[prop_or("0.7cm".to_string())]
    pub min_line_height: String,
}

#[function_component]
pub fn Instructions(props: &InstructionsProps) -> Html {
    let lines = (1..props.lines)
        .map(|l| l.to_string())
        .collect::<Vec<String>>();

    let line_height = format!("height: {};", props.min_line_height);

    html! {
        <table class={classes!("table", "table-striped", "table-bordered", "table-sm")}>
            <thead>
                <tr>
                    <th class={classes!("bordered")} style="width: 7%;">{"L/R"}</th>
                    <th class={classes!("bordered", "boldBorderRight")} style="width: 15%;">{"HDG"}</th>
                    <th class={classes!("bordered")} style="width: 7%;">{"><"}</th>
                    <th class={classes!("bordered", "boldBorderRight")} style="width: 15%;">{"SPD"}</th>
                    <th class={classes!("bordered")} style="width: 7%;">{"∓"}</th>
                    <th class={classes!("bordered")} style="width: 15%;">{"FL"}</th>
                    <th class={classes!("bordered")} style="width: 15%;">{"Rate"}</th>
                </tr>
            </thead>
            <tbody>
                {
                    lines.into_iter().map(|line| {
                        html!{
                            <tr key={line} style={line_height.clone()}>
                                <td class={classes!("bordered")}></td>
                                <td class={classes!("bordered", "boldBorderRight")}></td>
                                <td class={classes!("bordered")}></td>
                                <td class={classes!("bordered", "boldBorderRight")}></td>
                                <td class={classes!("bordered")}></td>
                                <td class={classes!("bordered")}></td>
                                <td class={classes!("bordered")}></td>
                            </tr>
                        }
                    }).collect::<Html>()
                }
            </tbody>
        </table>
    }
}
