//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "brand", feature = "media"))]
#[component]
pub fn ApplePodcastsLogo(
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
                <path d="M159.8,151.82a19.67,19.67,0,0,1,3.58,17.05l-12.18,48A20.17,20.17,0,0,1,131.56,232h-7.12a20.17,20.17,0,0,1-19.64-15.13l-12.18-48a19.67,19.67,0,0,1,3.58-17.05,20.17,20.17,0,0,1,16-7.82h31.5A20.17,20.17,0,0,1,159.8,151.82ZM156,116a28,28,0,1,0-28,28A28,28,0,0,0,156,116Zm26,27a8,8,0,1,0,15.41,4.29,72,72,0,1,0-138.74,0A8,8,0,0,0,74,143,56,56,0,1,1,182,143ZM128,24A104,104,0,0,0,70.18,214.46a8,8,0,1,0,8.9-13.3,88,88,0,1,1,97.84,0,8,8,0,0,0,8.9,13.3A104,104,0,0,0,128,24Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M104,120a24,24,0,1,1,24,24A24,24,0,0,1,104,120Zm39.75,24h-31.5a16.06,16.06,0,0,0-15.76,19.88l12.19,48A16.2,16.2,0,0,0,124.44,224h7.12a16.2,16.2,0,0,0,15.76-12.12l12.19-48A16.06,16.06,0,0,0,143.75,144Z"
        opacity="0.2"
    ></path>
    <path d="M154.2,138.33a32,32,0,1,0-52.4,0,24.18,24.18,0,0,0-8.76,7,23.68,23.68,0,0,0-4.3,20.49l12.18,48A24.18,24.18,0,0,0,124.44,232h7.12a24.18,24.18,0,0,0,23.52-18.15l12.18-48a23.68,23.68,0,0,0-4.3-20.49A24.18,24.18,0,0,0,154.2,138.33ZM128,104a16,16,0,1,1-16,16A16,16,0,0,1,128,104Zm23.75,57.91-12.18,48a8.18,8.18,0,0,1-8,6.09h-7.12a8.18,8.18,0,0,1-8-6.09l-12.18-48a7.71,7.71,0,0,1,1.42-6.73,8.26,8.26,0,0,1,6.58-3.18h31.5a8.26,8.26,0,0,1,6.58,3.18A7.71,7.71,0,0,1,151.75,161.91ZM72,128a56.31,56.31,0,0,0,2,15,8,8,0,0,1-15.41,4.29,72,72,0,1,1,138.74,0A8,8,0,0,1,182,143,56,56,0,1,0,72,128Zm160,0a103.92,103.92,0,0,1-46.18,86.46,8,8,0,0,1-8.9-13.3,88,88,0,1,0-97.84,0,8,8,0,0,1-8.9,13.3A104,104,0,1,1,232,128Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M147.26,140.3a28,28,0,1,0-38.52,0,20.12,20.12,0,0,0-12.54,7.52,19.67,19.67,0,0,0-3.58,17.05l12.18,48A20.17,20.17,0,0,0,124.44,228h7.12a20.17,20.17,0,0,0,19.64-15.13l12.18-48a19.67,19.67,0,0,0-3.58-17.05A20.12,20.12,0,0,0,147.26,140.3ZM108,120a20,20,0,1,1,20,20A20,20,0,0,1,108,120Zm47.63,42.9-12.18,48a12.19,12.19,0,0,1-11.89,9.1h-7.12a12.19,12.19,0,0,1-11.89-9.1l-12.18-48a11.76,11.76,0,0,1,2.14-10.17,12.24,12.24,0,0,1,9.74-4.73h31.5a12.24,12.24,0,0,1,9.74,4.73A11.76,11.76,0,0,1,155.63,162.9ZM188,128A60,60,0,1,0,70.19,144.11a4,4,0,1,1-7.71,2.14,68,68,0,1,1,131,0,4,4,0,0,1-3.85,2.93,3.88,3.88,0,0,1-1.07-.14,4,4,0,0,1-2.79-4.93A60.19,60.19,0,0,0,188,128Zm40,0a99.92,99.92,0,0,1-44.4,83.13,4,4,0,0,1-4.46-6.64,92,92,0,1,0-102.28,0,4,4,0,0,1-4.46,6.64A100,100,0,1,1,228,128Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M161.36,130a40,40,0,1,0-66.72,0,28.35,28.35,0,0,0-4.52,4.51,27.86,27.86,0,0,0-5.42,23.67l12.75,56A27.87,27.87,0,0,0,124.77,236h6.46a27.87,27.87,0,0,0,27.32-21.79l12.75-56a27.86,27.86,0,0,0-5.42-23.67A28.35,28.35,0,0,0,161.36,130ZM128,92a16,16,0,1,1-16,16A16,16,0,0,1,128,92Zm19.89,60.88-12.74,56a4,4,0,0,1-3.92,3.12h-6.46a4,4,0,0,1-3.92-3.12l-12.74-56a3.92,3.92,0,0,1,.77-3.37A4,4,0,0,1,112,148H144a4,4,0,0,1,3.15,1.51A3.92,3.92,0,0,1,147.89,152.88ZM236,128a107.88,107.88,0,0,1-38,82.21A12,12,0,0,1,182.47,192a84,84,0,1,0-108.94,0A12,12,0,0,1,58,210.21,108,108,0,1,1,236,128Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M151,139.2a30,30,0,1,0-46.06,0,22.14,22.14,0,0,0-10.35,7.39,21.68,21.68,0,0,0-3.94,18.77l12.18,48A22.17,22.17,0,0,0,124.44,230h7.12a22.17,22.17,0,0,0,21.58-16.64l12.18-48a21.68,21.68,0,0,0-3.94-18.77A22.14,22.14,0,0,0,151,139.2ZM128,102a18,18,0,1,1-18,18A18,18,0,0,1,128,102Zm25.69,60.4-12.18,48a10.19,10.19,0,0,1-9.95,7.6h-7.12a10.19,10.19,0,0,1-10-7.6l-12.18-48a9.75,9.75,0,0,1,1.78-8.44,10.25,10.25,0,0,1,8.16-4h31.5a10.25,10.25,0,0,1,8.16,4A9.75,9.75,0,0,1,153.69,162.4ZM186,128A58,58,0,1,0,72.11,143.58a6,6,0,0,1-11.56,3.21,70,70,0,1,1,134.9,0,6,6,0,0,1-11.56-3.21A58.31,58.31,0,0,0,186,128Zm44,0a101.91,101.91,0,0,1-45.29,84.79,6,6,0,1,1-6.68-10,90,90,0,1,0-100.06,0,6,6,0,0,1-6.68,10A102,102,0,1,1,230,128Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M154.2,138.33a32,32,0,1,0-52.4,0,24.27,24.27,0,0,0-8.76,7,23.68,23.68,0,0,0-4.3,20.49l12.18,48A24.18,24.18,0,0,0,124.44,232h7.12a24.18,24.18,0,0,0,23.52-18.15l12.18-48a23.68,23.68,0,0,0-4.3-20.49A24.27,24.27,0,0,0,154.2,138.33ZM128,104a16,16,0,1,1-16,16A16,16,0,0,1,128,104Zm23.75,57.91-12.18,48a8.18,8.18,0,0,1-8,6.09h-7.12a8.18,8.18,0,0,1-8-6.09l-12.18-48a7.71,7.71,0,0,1,1.42-6.73,8.26,8.26,0,0,1,6.58-3.18h31.5a8.26,8.26,0,0,1,6.58,3.18A7.71,7.71,0,0,1,151.75,161.91ZM72,128a56.31,56.31,0,0,0,2,15,8,8,0,0,1-15.41,4.29,72,72,0,1,1,138.74,0A8,8,0,0,1,182,143,56,56,0,1,0,72,128Zm160,0a103.92,103.92,0,0,1-46.18,86.46,8,8,0,0,1-8.9-13.3,88,88,0,1,0-97.84,0,8,8,0,0,1-8.9,13.3A104,104,0,1,1,232,128Z"></path>
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
