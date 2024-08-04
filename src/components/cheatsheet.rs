use yew::{classes, function_component, html, Html};

use crate::components::{
    approach_taxi::ApproachTaxi, clearance::Clearance, cruise::Cruise,
    general_information::GeneralInformation, route::Route, section::Section, taxi::Taxi,
};

#[function_component]
pub fn Cheatsheet() -> Html {
    html! {
        <div class={classes!("paper")} id={"section-to-print"}>
                <h1 class={classes!("mb-3")}>{"Vatsim Preparation"}</h1>
                <Section label="Gen" height={"1.5cm".to_string()}>
                        <GeneralInformation />
                </Section>
                <Section label="Clearance" height={"3.2cm".to_string()}>
                        <Clearance/>
                </Section>
                <Section label="Taxi" height={"3.9cm".to_string()}>
                        <Taxi />
                </Section>
                <Section label="Cruise" height={"7cm".to_string()}>
                        <Cruise  />
                </Section>
                <Section label="Routing" height={"2cm".to_string()}>
                        <Route  />
                </Section>
                <Section label="Approach & Taxi" height={"4.8cm".to_string()}>
                        <ApproachTaxi/>
                </Section>
        </div>
    }
}
