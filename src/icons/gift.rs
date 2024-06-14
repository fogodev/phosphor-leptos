//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "commerce", feature = "objects"))]
#[component]
pub fn Gift(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
    #[prop(into, optional)] id: MaybeProp<TextProp>,
    #[prop(into, optional)] class: MaybeProp<TextProp>,
) -> impl IntoView {
    let body = Signal::derive(move || {
        match weight.get() {
            IconWeight::Fill => view! {
                <path d="M216,72H180.92c.39-.33.79-.65,1.17-1A29.53,29.53,0,0,0,192,49.57,32.62,32.62,0,0,0,158.44,16,29.53,29.53,0,0,0,137,25.91a54.94,54.94,0,0,0-9,14.48,54.94,54.94,0,0,0-9-14.48A29.53,29.53,0,0,0,97.56,16,32.62,32.62,0,0,0,64,49.57,29.53,29.53,0,0,0,73.91,71c.38.33.78.65,1.17,1H40A16,16,0,0,0,24,88v32a16,16,0,0,0,16,16v64a16,16,0,0,0,16,16h60a4,4,0,0,0,4-4V120H40V88h80v32h16V88h80v32H136v92a4,4,0,0,0,4,4h60a16,16,0,0,0,16-16V136a16,16,0,0,0,16-16V88A16,16,0,0,0,216,72ZM84.51,59a13.69,13.69,0,0,1-4.5-10A16.62,16.62,0,0,1,96.59,32h.49a13.69,13.69,0,0,1,10,4.5c8.39,9.48,11.35,25.2,12.39,34.92C109.71,70.39,94,67.43,84.51,59Zm87,0c-9.49,8.4-25.24,11.36-35,12.4C137.7,60.89,141,45.5,149,36.51a13.69,13.69,0,0,1,10-4.5h.49A16.62,16.62,0,0,1,176,49.08,13.69,13.69,0,0,1,171.49,59Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M208,128v72a8,8,0,0,1-8,8H56a8,8,0,0,1-8-8V128Z" opacity="0.2"></path>
    <path d="M216,72H180.92c.39-.33.79-.65,1.17-1A29.53,29.53,0,0,0,192,49.57,32.62,32.62,0,0,0,158.44,16,29.53,29.53,0,0,0,137,25.91a54.94,54.94,0,0,0-9,14.48,54.94,54.94,0,0,0-9-14.48A29.53,29.53,0,0,0,97.56,16,32.62,32.62,0,0,0,64,49.57,29.53,29.53,0,0,0,73.91,71c.38.33.78.65,1.17,1H40A16,16,0,0,0,24,88v32a16,16,0,0,0,16,16v64a16,16,0,0,0,16,16H200a16,16,0,0,0,16-16V136a16,16,0,0,0,16-16V88A16,16,0,0,0,216,72ZM149,36.51a13.69,13.69,0,0,1,10-4.5h.49A16.62,16.62,0,0,1,176,49.08a13.69,13.69,0,0,1-4.5,10c-9.49,8.4-25.24,11.36-35,12.4C137.7,60.89,141,45.5,149,36.51Zm-64.09.36A16.63,16.63,0,0,1,96.59,32h.49a13.69,13.69,0,0,1,10,4.5c8.39,9.48,11.35,25.2,12.39,34.92-9.72-1-25.44-4-34.92-12.39a13.69,13.69,0,0,1-4.5-10A16.6,16.6,0,0,1,84.87,36.87ZM40,88h80v32H40Zm16,48h64v64H56Zm144,64H136V136h64Zm16-80H136V88h80v32Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M216,76H166.81a50.54,50.54,0,0,0,12.63-8A25.53,25.53,0,0,0,188,49.44,28.62,28.62,0,0,0,158.56,20,25.53,25.53,0,0,0,140,28.56c-5.91,6.67-9.63,15.36-12,23.69-2.35-8.33-6.07-17-12-23.69A25.53,25.53,0,0,0,97.44,20,28.62,28.62,0,0,0,68,49.44,25.53,25.53,0,0,0,76.56,68a50.54,50.54,0,0,0,12.63,8H40A12,12,0,0,0,28,88v32a12,12,0,0,0,12,12h4v68a12,12,0,0,0,12,12H200a12,12,0,0,0,12-12V132h4a12,12,0,0,0,12-12V88A12,12,0,0,0,216,76ZM146,33.86A17.59,17.59,0,0,1,158.8,28h.61A20.62,20.62,0,0,1,180,49.2,17.6,17.6,0,0,1,174.14,62c-12.09,10.7-33.07,13.21-42,13.79C132.76,66.93,135.26,46,146,33.86ZM76,49.2A20.62,20.62,0,0,1,96.59,28h.61A17.59,17.59,0,0,1,110,33.86c10.71,12.09,13.21,33.07,13.79,42-8.89-.58-29.87-3.09-42-13.79A17.6,17.6,0,0,1,76,49.2ZM36,120V88a4,4,0,0,1,4-4h84v40H40A4,4,0,0,1,36,120Zm16,80V132h72v72H56A4,4,0,0,1,52,200Zm152,0a4,4,0,0,1-4,4H132V132h72Zm16-80a4,4,0,0,1-4,4H132V84h84a4,4,0,0,1,4,4Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M216,68H190.06A33.82,33.82,0,0,0,196,49.69,36.62,36.62,0,0,0,158.31,12,33.44,33.44,0,0,0,134,23.25a54.65,54.65,0,0,0-6,8.3,54.65,54.65,0,0,0-6-8.3A33.44,33.44,0,0,0,97.69,12,36.62,36.62,0,0,0,60,49.69,33.82,33.82,0,0,0,65.94,68H40A20,20,0,0,0,20,88v32a20,20,0,0,0,16,19.6V200a20,20,0,0,0,20,20H200a20,20,0,0,0,20-20V139.6A20,20,0,0,0,236,120V88A20,20,0,0,0,216,68Zm-4,48H140V92h72ZM152,39.17A9.59,9.59,0,0,1,159,36h.35A12.62,12.62,0,0,1,172,49,9.59,9.59,0,0,1,168.83,56c-6.9,6.12-18.25,9.26-27.63,10.76C142.7,57.42,145.84,46.07,152,39.17ZM87.7,39.7A12.8,12.8,0,0,1,96.61,36H97A9.59,9.59,0,0,1,104,39.17c6.12,6.9,9.26,18.24,10.75,27.61C105.45,65.27,94,62.13,87.17,56A9.59,9.59,0,0,1,84,49,12.72,12.72,0,0,1,87.7,39.7ZM44,92h72v24H44Zm16,48h56v56H60Zm80,56V140h56v56Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M216,74H174.74a46.41,46.41,0,0,0,6-4.48,27.56,27.56,0,0,0,9.22-20A30.63,30.63,0,0,0,158.5,18a27.56,27.56,0,0,0-20,9.22A57.1,57.1,0,0,0,128,45.76a57.1,57.1,0,0,0-10.48-18.53A27.56,27.56,0,0,0,97.5,18,30.63,30.63,0,0,0,66,49.51a27.56,27.56,0,0,0,9.22,20,45.74,45.74,0,0,0,6,4.48H40A14,14,0,0,0,26,88v32a14,14,0,0,0,14,14h2v66a14,14,0,0,0,14,14H200a14,14,0,0,0,14-14V134h2a14,14,0,0,0,14-14V88A14,14,0,0,0,216,74ZM135.77,63c2.25-12.12,6.29-21.75,11.69-27.85A15.68,15.68,0,0,1,158.86,30h.55A18.6,18.6,0,0,1,178,49.14a15.68,15.68,0,0,1-5.18,11.4C162.1,70,143.92,72.83,134.34,73.65,134.59,70.76,135,67.08,135.77,63ZM83.45,35.45A18.69,18.69,0,0,1,96.59,30h.55a15.68,15.68,0,0,1,11.4,5.18C118,45.9,120.83,64.08,121.65,73.66c-2.89-.25-6.57-.68-10.61-1.43C98.92,70,89.29,65.94,83.19,60.53A15.64,15.64,0,0,1,78,49.14,18.65,18.65,0,0,1,83.45,35.45ZM38,120V88a2,2,0,0,1,2-2h82v36H40A2,2,0,0,1,38,120Zm16,80V134h68v68H56A2,2,0,0,1,54,200Zm148,0a2,2,0,0,1-2,2H134V134h68Zm16-80a2,2,0,0,1-2,2H134V86h82a2,2,0,0,1,2,2Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M216,72H180.92c.39-.33.79-.65,1.17-1A29.53,29.53,0,0,0,192,49.57,32.62,32.62,0,0,0,158.44,16,29.53,29.53,0,0,0,137,25.91a54.94,54.94,0,0,0-9,14.48,54.94,54.94,0,0,0-9-14.48A29.53,29.53,0,0,0,97.56,16,32.62,32.62,0,0,0,64,49.57,29.53,29.53,0,0,0,73.91,71c.38.33.78.65,1.17,1H40A16,16,0,0,0,24,88v32a16,16,0,0,0,16,16v64a16,16,0,0,0,16,16H200a16,16,0,0,0,16-16V136a16,16,0,0,0,16-16V88A16,16,0,0,0,216,72ZM149,36.51a13.69,13.69,0,0,1,10-4.5h.49A16.62,16.62,0,0,1,176,49.08a13.69,13.69,0,0,1-4.5,10c-9.49,8.4-25.24,11.36-35,12.4C137.7,60.89,141,45.5,149,36.51Zm-64.09.36A16.63,16.63,0,0,1,96.59,32h.49a13.69,13.69,0,0,1,10,4.5c8.39,9.48,11.35,25.2,12.39,34.92-9.72-1-25.44-4-34.92-12.39a13.69,13.69,0,0,1-4.5-10A16.6,16.6,0,0,1,84.87,36.87ZM40,88h80v32H40Zm16,48h64v64H56Zm144,64H136V136h64Zm16-80H136V88h80v32Z"></path>
}.into_view()
        }
    });

    let transform = move || if mirrored.get() { "scale(-1, 1)" } else { "" };
    let height = size.clone();

    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width=size
            height=height
            fill=color
            transform=transform
            viewBox="0 0 256 256"
            id=id
            class=class
        >
            {body}
        </svg>
    }
}
