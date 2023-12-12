//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn Broadcast(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view! {
                <path d="M128,84a44,44,0,1,0,44,44A44.05,44.05,0,0,0,128,84Zm0,64a20,20,0,1,1,20-20A20,20,0,0,1,128,148Zm77.39,12.7A83.94,83.94,0,0,1,190.61,184a12,12,0,0,1-17.89-16,59.92,59.92,0,0,0,0-80,12,12,0,0,1,17.89-16,84.07,84.07,0,0,1,14.78,88.7ZM83.28,168a12,12,0,0,1-17.89,16,83.94,83.94,0,0,1,0-112A12,12,0,0,1,83.28,88a59.92,59.92,0,0,0,0,80ZM252,128a123.63,123.63,0,0,1-35.43,86.78A12,12,0,1,1,199.43,198a99.88,99.88,0,0,0,0-140,12,12,0,0,1,17.14-16.8A123.63,123.63,0,0,1,252,128ZM56.57,198a12,12,0,0,1-17.14,16.8,123.89,123.89,0,0,1,0-173.56A12,12,0,0,1,56.57,58a99.88,99.88,0,0,0,0,140Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M160,128a32,32,0,1,1-32-32A32,32,0,0,1,160,128Z" opacity="0.2"></path>
    <path d="M128,88a40,40,0,1,0,40,40A40,40,0,0,0,128,88Zm0,64a24,24,0,1,1,24-24A24,24,0,0,1,128,152Zm73.71,7.14a80,80,0,0,1-14.08,22.2,8,8,0,0,1-11.92-10.67,63.95,63.95,0,0,0,0-85.33,8,8,0,1,1,11.92-10.67,80.08,80.08,0,0,1,14.08,84.47ZM69,103.09a64,64,0,0,0,11.26,67.58,8,8,0,0,1-11.92,10.67,79.93,79.93,0,0,1,0-106.67A8,8,0,1,1,80.29,85.34,63.77,63.77,0,0,0,69,103.09ZM248,128a119.58,119.58,0,0,1-34.29,84,8,8,0,1,1-11.42-11.2,103.9,103.9,0,0,0,0-145.56A8,8,0,1,1,213.71,44,119.58,119.58,0,0,1,248,128ZM53.71,200.78A8,8,0,1,1,42.29,212a119.87,119.87,0,0,1,0-168,8,8,0,1,1,11.42,11.2,103.9,103.9,0,0,0,0,145.56Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M168,128a40,40,0,1,1-40-40A40,40,0,0,1,168,128Zm40,0a79.74,79.74,0,0,0-20.37-53.33,8,8,0,1,0-11.92,10.67,64,64,0,0,1,0,85.33,8,8,0,0,0,11.92,10.67A79.79,79.79,0,0,0,208,128ZM80.29,85.34A8,8,0,1,0,68.37,74.67a79.94,79.94,0,0,0,0,106.67,8,8,0,0,0,11.92-10.67,63.95,63.95,0,0,1,0-85.33Zm158.28-4A119.48,119.48,0,0,0,213.71,44a8,8,0,1,0-11.42,11.2,103.9,103.9,0,0,1,0,145.56A8,8,0,1,0,213.71,212,120.12,120.12,0,0,0,238.57,81.29ZM32.17,168.48A103.9,103.9,0,0,1,53.71,55.22,8,8,0,1,0,42.29,44a119.87,119.87,0,0,0,0,168,8,8,0,1,0,11.42-11.2A103.61,103.61,0,0,1,32.17,168.48Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M128,90a38,38,0,1,0,38,38A38,38,0,0,0,128,90Zm0,64a26,26,0,1,1,26-26A26,26,0,0,1,128,154Zm78-26a77.74,77.74,0,0,1-19.86,52,6,6,0,0,1-8.94-8,65.93,65.93,0,0,0,0-88,6,6,0,1,1,8.94-8A77.74,77.74,0,0,1,206,128ZM67.18,102.31A65.93,65.93,0,0,0,78.8,172a6,6,0,0,1-.47,8.47,6,6,0,0,1-8.47-.47,77.93,77.93,0,0,1,0-104,6,6,0,1,1,8.94,8A66.21,66.21,0,0,0,67.18,102.31ZM246,128a117.71,117.71,0,0,1-33.71,82.58,6,6,0,0,1-8.58-8.4,105.88,105.88,0,0,0,0-148.36,6,6,0,0,1,8.58-8.4A117.71,117.71,0,0,1,246,128ZM52.29,202.18a6,6,0,0,1-8.58,8.4,117.92,117.92,0,0,1,0-165.16,6,6,0,1,1,8.58,8.4,105.88,105.88,0,0,0,0,148.36Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M128,88a40,40,0,1,0,40,40A40,40,0,0,0,128,88Zm0,64a24,24,0,1,1,24-24A24,24,0,0,1,128,152Zm73.71,7.14a80,80,0,0,1-14.08,22.2,8,8,0,0,1-11.92-10.67,63.95,63.95,0,0,0,0-85.33,8,8,0,1,1,11.92-10.67,80.08,80.08,0,0,1,14.08,84.47ZM69,103.09a64,64,0,0,0,11.26,67.58,8,8,0,0,1-11.92,10.67,79.93,79.93,0,0,1,0-106.67A8,8,0,1,1,80.29,85.34,63.77,63.77,0,0,0,69,103.09ZM248,128a119.58,119.58,0,0,1-34.29,84,8,8,0,1,1-11.42-11.2,103.9,103.9,0,0,0,0-145.56A8,8,0,1,1,213.71,44,119.58,119.58,0,0,1,248,128ZM53.71,200.78A8,8,0,1,1,42.29,212a119.87,119.87,0,0,1,0-168,8,8,0,1,1,11.42,11.2,103.9,103.9,0,0,0,0,145.56Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M128,92a36,36,0,1,0,36,36A36,36,0,0,0,128,92Zm0,64a28,28,0,1,1,28-28A28,28,0,0,1,128,156Zm76-28a75.74,75.74,0,0,1-19.35,50.67,4,4,0,0,1-6-5.34,67.92,67.92,0,0,0,0-90.66,4,4,0,0,1,6-5.34A75.74,75.74,0,0,1,204,128ZM65.34,101.53a67.92,67.92,0,0,0,12,71.8,4,4,0,0,1-6,5.34,75.93,75.93,0,0,1,0-101.34,4,4,0,1,1,6,5.34A68,68,0,0,0,65.34,101.53ZM244,128a115.68,115.68,0,0,1-33.14,81.18,4,4,0,0,1-5.72-5.6,107.89,107.89,0,0,0,0-151.16,4,4,0,0,1,5.72-5.6A115.68,115.68,0,0,1,244,128ZM50.86,203.58a4,4,0,0,1-5.72,5.6,115.91,115.91,0,0,1,0-162.36,4,4,0,1,1,5.72,5.6,107.89,107.89,0,0,0,0,151.16Z"></path>
}.into_view()
        }
    };

    let transform = move || if mirrored.get() { "scale(-1, 1)" } else { "" };

    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width=size.get()
            height=size.get()
            fill=color
            transform=transform
            viewBox="0 0 256 256"
        >
            {body}
        </svg>
    }
}
