//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "finance"))]
#[component]
pub fn NumberCircleThree(
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
                <path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm-4,164a35.71,35.71,0,0,1-25.71-10.81A8,8,0,1,1,109.71,166,20,20,0,1,0,124,132a8,8,0,0,1-6.55-12.59L136.63,92H104a8,8,0,0,1,0-16h48a8,8,0,0,1,6.55,12.59l-21,30A36,36,0,0,1,124,188Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M224,128a96,96,0,1,1-96-96A96,96,0,0,1,224,128Z" opacity="0.2"></path>
    <path d="M160,152a36,36,0,0,1-61.71,25.19A8,8,0,1,1,109.71,166,20,20,0,1,0,124,132a8,8,0,0,1-6.55-12.59L136.63,92H104a8,8,0,0,1,0-16h48a8,8,0,0,1,6.55,12.59l-21,30A36.07,36.07,0,0,1,160,152Zm72-24A104,104,0,1,1,128,24,104.11,104.11,0,0,1,232,128Zm-16,0a88,88,0,1,0-88,88A88.1,88.1,0,0,0,216,128Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M128,28A100,100,0,1,0,228,128,100.11,100.11,0,0,0,128,28Zm0,192a92,92,0,1,1,92-92A92.1,92.1,0,0,1,128,220Zm28-68a32,32,0,0,1-54.86,22.4,4,4,0,1,1,5.72-5.6A24,24,0,1,0,124,128a4,4,0,0,1-3.28-6.29L144.32,88H104a4,4,0,0,1,0-8h48a4,4,0,0,1,3.28,6.29L131.12,120.8A32.06,32.06,0,0,1,156,152Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M128,20A108,108,0,1,0,236,128,108.12,108.12,0,0,0,128,20Zm0,192a84,84,0,1,1,84-84A84.09,84.09,0,0,1,128,212Zm36-60a40,40,0,0,1-68.57,28,12,12,0,1,1,17.14-16.79A16,16,0,1,0,124,136a12,12,0,0,1-9.83-18.88L129,96H104a12,12,0,0,1,0-24h48a12,12,0,0,1,9.83,18.88l-18.34,26.2A40,40,0,0,1,164,152Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M128,26A102,102,0,1,0,230,128,102.12,102.12,0,0,0,128,26Zm0,192a90,90,0,1,1,90-90A90.1,90.1,0,0,1,128,218Zm30-66a34,34,0,0,1-58.29,23.79,6,6,0,0,1,8.58-8.39A22,22,0,1,0,124,130a6,6,0,0,1-4.92-9.44L140.48,90H104a6,6,0,0,1,0-12h48a6,6,0,0,1,4.92,9.44l-22.53,32.18A34.06,34.06,0,0,1,158,152Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm0,192a88,88,0,1,1,88-88A88.1,88.1,0,0,1,128,216Zm32-64a36,36,0,0,1-61.71,25.19A8,8,0,1,1,109.71,166,20,20,0,1,0,124,132a8,8,0,0,1-6.55-12.59L136.63,92H104a8,8,0,0,1,0-16h48a8,8,0,0,1,6.55,12.59l-21,30A36.07,36.07,0,0,1,160,152Z"></path>
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
