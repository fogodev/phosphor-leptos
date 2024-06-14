//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "map", feature = "objects"))]
#[component]
pub fn Motorcycle(
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
                <path d="M216,120a41,41,0,0,0-6.6.55l-5.82-15.14A55.64,55.64,0,0,1,216,104a8,8,0,0,0,0-16H196.88L183.47,53.13A8,8,0,0,0,176,48H144a8,8,0,0,0,0,16h26.51l9.23,24H152c-18.5,0-33.5,4.31-43.37,12.46a16,16,0,0,1-16.76,2.07c-10.58-4.81-73.29-30.12-73.8-30.26a8,8,0,0,0-5,15.19S68.57,109.4,79.6,120.4A55.67,55.67,0,0,1,95.43,152H79.2a40,40,0,1,0,0,16h52.12a31.91,31.91,0,0,0,30.74-23.1,56,56,0,0,1,26.59-33.72l5.82,15.13A40,40,0,1,0,216,120ZM40,168H62.62a24,24,0,1,1,0-16H40a8,8,0,0,0,0,16Zm176,16a24,24,0,0,1-15.58-42.23l8.11,21.1a8,8,0,1,0,14.94-5.74L215.35,136l.65,0a24,24,0,0,1,0,48Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M248,160a32,32,0,1,1-32-32A32,32,0,0,1,248,160ZM40,128a32,32,0,1,0,32,32A32,32,0,0,0,40,128Z"
        opacity="0.2"
    ></path>
    <path d="M216,120a41,41,0,0,0-6.6.55l-5.82-15.14A55.64,55.64,0,0,1,216,104a8,8,0,0,0,0-16H196.88L183.47,53.13A8,8,0,0,0,176,48H144a8,8,0,0,0,0,16h26.51l9.23,24H152c-18.5,0-33.5,4.31-43.37,12.46a16,16,0,0,1-16.76,2.07C81.29,97.72,31.13,77.33,26.71,75.6L21,73.36A17.74,17.74,0,0,0,16,72a8,8,0,0,0-2.87,15.46h0c.46.18,47.19,18.3,72.13,29.63a32.15,32.15,0,0,0,33.56-4.29c4.86-4,14.57-8.8,33.19-8.8h18.82a71.74,71.74,0,0,0-24.17,36.59A15.86,15.86,0,0,1,131.32,152H79.2a40,40,0,1,0,0,16h52.12a31.91,31.91,0,0,0,30.74-23.1,56,56,0,0,1,26.59-33.72l5.82,15.13A40,40,0,1,0,216,120ZM40,168H62.62a24,24,0,1,1,0-16H40a8,8,0,0,0,0,16Zm176,16a24,24,0,0,1-15.58-42.23l8.11,21.1a8,8,0,1,0,14.94-5.74L215.35,136l.65,0a24,24,0,0,1,0,48Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M216,124a36,36,0,0,0-9.11,1.17l-8.64-22.46A59.64,59.64,0,0,1,216,100a4,4,0,0,0,0-8H194.13l-14.4-37.44A4,4,0,0,0,176,52H144a4,4,0,0,0,0,8h29.25l12.31,32H152c-17.56,0-31.67,4-40.83,11.54a20.05,20.05,0,0,1-21,2.63c-13.11-6-60.55-25.12-65-26.85C22.83,78.37,17.24,76,16,76a4,4,0,0,0-1.43,7.72h0c.46.18,47.42,18.4,72.34,29.72a28.12,28.12,0,0,0,29.37-3.74c7.7-6.35,20-9.71,35.73-9.71h32.08a68.05,68.05,0,0,0-33.58,41.67A19.86,19.86,0,0,1,131.32,156H75.77a36,36,0,1,0,0,8h55.55a27.9,27.9,0,0,0,26.89-20.18,60.06,60.06,0,0,1,32.58-38.22l8.64,22.46A36,36,0,1,0,216,124ZM40,164H67.71a28,28,0,1,1,0-8H40a4,4,0,0,0,0,8Zm176,24a28,28,0,0,1-13.68-52.42l10,25.86A4,4,0,0,0,216,164a3.87,3.87,0,0,0,1.44-.27,4,4,0,0,0,2.29-5.17l-9.94-25.86A28,28,0,1,1,216,188Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M208,112c-.57,0-1.13,0-1.69,0l-3.5-11.8c1.71-.15,3.44-.24,5.19-.24a12,12,0,0,0,0-24H195.63l-8.12-27.41A12,12,0,0,0,176,40H144a12,12,0,0,0,0,24h23l3.56,12H152c-19.44,0-35.32,4.62-45.92,13.37a12,12,0,0,1-12.56,1.52c-13.13-6-57.19-25-61.65-26.89L26.6,61.76S22.4,60,20,60a12,12,0,0,0-4.61,23.08c1.72.73,43.67,18.5,68.2,29.66a36.25,36.25,0,0,0,15,3.23,35.78,35.78,0,0,0,22.78-8.09c5.74-4.73,15.58-7.48,27.89-7.84a84,84,0,0,0-24.4,48H94.48a48,48,0,1,0,0,24H136a12,12,0,0,0,12-12,60,60,0,0,1,31.8-52.94l3.5,11.8A48,48,0,1,0,208,112ZM48,172H68.77a24,24,0,1,1,0-24H48a12,12,0,0,0,0,24Zm160,12a24,24,0,0,1-17.4-40.5l5.89,19.91A12,12,0,0,0,208,172a12.28,12.28,0,0,0,3.41-.49,12,12,0,0,0,8.1-14.92l-5.91-19.92A24,24,0,0,1,208,184Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M216,122a38.48,38.48,0,0,0-7.87.82L200.9,104a57.83,57.83,0,0,1,15.1-2,6,6,0,0,0,0-12H195.51L181.6,53.85A6,6,0,0,0,176,50H144a6,6,0,0,0,0,12h27.88l10.77,28H152c-18,0-32.58,4.15-42.1,12A18.05,18.05,0,0,1,91,104.35C77.9,98.38,30.4,79.19,26,77.46l-5.72-2.24A14.66,14.66,0,0,0,16,74a6,6,0,0,0-2.15,11.6h0c.46.18,47.13,18.26,72.23,29.67a30.12,30.12,0,0,0,31.47-4c7.34-6,19.25-9.25,34.46-9.25h24.89a70,70,0,0,0-28.32,39.13A17.85,17.85,0,0,1,131.32,154H77.52a38,38,0,1,0,0,12h53.8a29.9,29.9,0,0,0,28.81-21.64,58,58,0,0,1,29.58-36l7.23,18.8A38,38,0,1,0,216,122ZM40,166H65.29a26,26,0,1,1,0-12H40a6,6,0,0,0,0,12Zm176,20a26,26,0,0,1-14.68-47.45l9.08,23.6a6,6,0,0,0,11.2-4.3l-9.08-23.61A26.64,26.64,0,0,1,216,134a26,26,0,0,1,0,52Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M216,120a41,41,0,0,0-6.6.55l-5.82-15.14A55.64,55.64,0,0,1,216,104a8,8,0,0,0,0-16H196.88L183.47,53.13A8,8,0,0,0,176,48H144a8,8,0,0,0,0,16h26.51l9.23,24H152c-18.5,0-33.5,4.31-43.37,12.46a16,16,0,0,1-16.76,2.07C81.29,97.72,31.13,77.33,26.71,75.6L21,73.36A17.74,17.74,0,0,0,16,72a8,8,0,0,0-2.87,15.46h0c.46.18,47.19,18.3,72.13,29.63a32.15,32.15,0,0,0,33.56-4.29c4.86-4,14.57-8.8,33.19-8.8h18.82a71.74,71.74,0,0,0-24.17,36.59A15.86,15.86,0,0,1,131.32,152H79.2a40,40,0,1,0,0,16h52.12a31.91,31.91,0,0,0,30.74-23.1,56,56,0,0,1,26.59-33.72l5.82,15.13A40,40,0,1,0,216,120ZM40,168H62.62a24,24,0,1,1,0-16H40a8,8,0,0,0,0,16Zm176,16a24,24,0,0,1-15.58-42.23l8.11,21.1a8,8,0,1,0,14.94-5.74L215.35,136l.65,0a24,24,0,0,1,0,48Z"></path>
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
