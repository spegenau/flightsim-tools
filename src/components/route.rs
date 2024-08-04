use yew::{classes, function_component, html, use_context, Html};

use crate::{simbrief::fix_info_point::FixInfoPoint, Context};

#[function_component]
pub fn Route() -> Html {
    let ctx = use_context::<Context>().expect("no ctx found");

    let navlog = ctx.simbrief.navlog.fix.clone();

    let sid = ctx.simbrief.general.get_sid();
    let star = ctx.simbrief.general.get_star();

    let mut points_sid: Vec<FixInfoPoint> = Vec::new();
    let mut points_crz: Vec<FixInfoPoint> = Vec::new();
    let mut points_star: Vec<FixInfoPoint> = Vec::new();

    for point in navlog {
        if point.ident != "TOC" && point.ident != "TOD" {
            if point.via_airway == sid {
                points_sid.push(point.clone());
            } else if point.via_airway == star {
                points_star.push(point.clone());
            } else {
                points_crz.push(point.clone());
            }
        }
    }

    let route_sid = points_to_route(points_sid);
    let route_crz = points_to_route(points_crz);
    let route_star = points_to_route(points_star);

    html! {
        <table class={classes!("table", "table-striped", "table-bordered", "table-sm")}>
            <thead>
                <tr>
                    <th>{"SID"}</th>
                    <th>{"Cruise"}</th>
                    <th>{"STAR"}</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td>{&route_sid}</td>
                    <td>{&route_crz}</td>
                    <td>{&route_star}</td>
                </tr>
            </tbody>
        </table>
    }
}

fn points_to_route(points: Vec<FixInfoPoint>) -> String {
    points
        .iter()
        .map(|p| {
            if p.ident == p.name {
                p.ident.clone()
            } else {
                format!("{} ({})", p.ident, p.name)
            }
        })
        .collect::<Vec<String>>()
        .join(" → ")
}
