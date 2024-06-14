//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "games", feature = "health", feature = "objects"))]
#[component]
pub fn TennisBall(
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
                <path d="M24.81,115.41a103.9,103.9,0,0,1,90.6-90.65,4,4,0,0,1,4.47,3.79,87.82,87.82,0,0,1-91.27,91.33A4,4,0,0,1,24.81,115.41Zm202.54,20.7c-1.12,0-2.23-.07-3.35-.07a87.84,87.84,0,0,0-87.88,91.41,4,4,0,0,0,4.47,3.79,103.9,103.9,0,0,0,90.6-90.66A4,4,0,0,0,227.35,136.11Zm-76.89,14.35A103.33,103.33,0,0,1,224,120c1,0,2.06,0,3.09,0a4,4,0,0,0,4.12-4.43,103.91,103.91,0,0,0-90.88-90.89,4,4,0,0,0-4.43,4.12,103.72,103.72,0,0,1-30.36,76.7A103.33,103.33,0,0,1,32,136c-1,0-2.06,0-3.09,0a4,4,0,0,0-4.12,4.43,103.91,103.91,0,0,0,90.88,90.89,4,4,0,0,0,4.43-4.12A103.72,103.72,0,0,1,150.46,150.46Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M60.12,60.09A95.74,95.74,0,0,1,127.83,32h0A95.94,95.94,0,0,1,32,127.75,95.64,95.64,0,0,1,60.12,60.09ZM32,127.92v-.17h0Zm124.28,28.26a95.54,95.54,0,0,0-28.11,67.65A96,96,0,0,0,224,128.08h0A95.79,95.79,0,0,0,156.28,156.18ZM128.17,224v-.17h0Z"
        opacity="0.2"
    ></path>
    <path d="M201.57,54.46a104,104,0,1,0,0,147.08A103.4,103.4,0,0,0,201.57,54.46ZM65.75,65.77a87.63,87.63,0,0,1,53.66-25.31A87.31,87.31,0,0,1,94,94.06a87.42,87.42,0,0,1-53.62,25.35A87.58,87.58,0,0,1,65.75,65.77ZM40.33,135.48a103.29,103.29,0,0,0,65-30.11,103.24,103.24,0,0,0,30.13-65,87.78,87.78,0,0,1,80.18,80.14,104,104,0,0,0-95.16,95.1,87.78,87.78,0,0,1-80.18-80.14Zm149.92,54.75a87.69,87.69,0,0,1-53.66,25.31,88,88,0,0,1,79-78.95A87.58,87.58,0,0,1,190.25,190.23Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M198.74,57.29A99.39,99.39,0,0,0,128,28h-.19a100,100,0,1,0,70.93,29.29ZM62.92,63a91.38,91.38,0,0,1,60.8-26.85A91.31,91.31,0,0,1,96.87,96.89,91.44,91.44,0,0,1,36.1,123.73,91.61,91.61,0,0,1,62.92,63ZM36.09,131.74a99.36,99.36,0,0,0,66.44-29.2,99.25,99.25,0,0,0,29.21-66.45,91.9,91.9,0,0,1,88.17,88.17,100,100,0,0,0-95.65,95.65,91.9,91.9,0,0,1-88.17-88.17Zm157,61.31a91.38,91.38,0,0,1-60.8,26.85,92,92,0,0,1,87.62-87.63A91.61,91.61,0,0,1,193.08,193.05Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M204.4,51.6a108,108,0,1,0,0,152.8A108.16,108.16,0,0,0,204.4,51.6Zm-135.82,17a83.47,83.47,0,0,1,46.2-23.52,84.09,84.09,0,0,1-69.72,69.72A83.47,83.47,0,0,1,68.58,68.58ZM44.74,139.07a107.27,107.27,0,0,0,63.45-30.88,107.27,107.27,0,0,0,30.88-63.45,83.84,83.84,0,0,1,72.19,72.19,108.11,108.11,0,0,0-94.33,94.32,83.82,83.82,0,0,1-72.19-72.18Zm142.68,48.35a83.53,83.53,0,0,1-46.2,23.52,84.09,84.09,0,0,1,69.72-69.72A83.47,83.47,0,0,1,187.42,187.42Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M200.16,55.88a102,102,0,1,0,0,144.24A101.4,101.4,0,0,0,200.16,55.88ZM64.33,64.36a89.62,89.62,0,0,1,57.25-26.07A89.32,89.32,0,0,1,95.46,95.47a89.38,89.38,0,0,1-57.21,26.11A89.61,89.61,0,0,1,64.33,64.36ZM38.2,133.63A101.36,101.36,0,0,0,104,104a101.24,101.24,0,0,0,29.68-65.72,89.76,89.76,0,0,1,84.17,84.13,102,102,0,0,0-95.43,95.39A89.76,89.76,0,0,1,38.2,133.63Zm153.47,58a89.63,89.63,0,0,1-57.25,26.06,89.94,89.94,0,0,1,83.33-83.28A89.61,89.61,0,0,1,191.67,191.64Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M201.57,54.46a104,104,0,1,0,0,147.08A103.4,103.4,0,0,0,201.57,54.46ZM65.75,65.77a87.63,87.63,0,0,1,53.66-25.31A87.31,87.31,0,0,1,94,94.06a87.42,87.42,0,0,1-53.62,25.35A87.58,87.58,0,0,1,65.75,65.77ZM40.33,135.48a103.29,103.29,0,0,0,65-30.11,103.24,103.24,0,0,0,30.13-65,87.78,87.78,0,0,1,80.18,80.14,104,104,0,0,0-95.16,95.1,87.78,87.78,0,0,1-80.18-80.14Zm149.92,54.75a87.69,87.69,0,0,1-53.66,25.31,88,88,0,0,1,79-78.95A87.58,87.58,0,0,1,190.25,190.23Z"></path>
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
