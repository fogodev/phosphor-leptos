//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "system"))]
#[component]
pub fn CloudSlash(
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
                <path d="M248,128.72A87.74,87.74,0,0,1,222.41,190a4,4,0,0,1-5.77-.16L103.78,65.67a4,4,0,0,1,.39-5.76A87.82,87.82,0,0,1,160.87,40C209.15,40.47,248.38,80.43,248,128.72ZM53.92,34.62A8,8,0,1,0,42.08,45.38L81.33,88.56l-.06.11A64,64,0,0,0,8,153c.53,35.12,29.84,63,65,63h87a87.65,87.65,0,0,0,31.78-5.95l10.3,11.33a8,8,0,0,0,11.33.52,8.32,8.32,0,0,0,.29-11.52Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M240,128a80,80,0,0,1-80,80H72A56,56,0,1,1,85.92,97.74l0,.1A80,80,0,0,1,240,128Z"
        opacity="0.2"
    ></path>
    <path d="M53.92,34.62A8,8,0,1,0,42.08,45.38L81.32,88.55l-.06.12A65,65,0,0,0,72,88a64,64,0,0,0,0,128h88a87.34,87.34,0,0,0,31.8-5.93l10.28,11.31a8,8,0,1,0,11.84-10.76ZM160,200H72a48,48,0,0,1,0-96c1.1,0,2.2,0,3.3.12A88.4,88.4,0,0,0,72,128a8,8,0,0,0,16,0,72.25,72.25,0,0,1,5.06-26.54l87,95.7A71.66,71.66,0,0,1,160,200Zm88-72a87.89,87.89,0,0,1-22.35,58.61A8,8,0,0,1,213.71,176,72,72,0,0,0,117.37,70a8,8,0,0,1-9.48-12.89A88,88,0,0,1,248,128Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M51,37.31A4,4,0,0,0,45,42.69L86.16,87.93q-1.38,2.55-2.59,5.19A60,60,0,1,0,72,212h88a83.19,83.19,0,0,0,32.88-6.69L205,218.69a4,4,0,1,0,5.92-5.38ZM160,204H72a52,52,0,0,1,0-104,52.92,52.92,0,0,1,8.54.72A84.21,84.21,0,0,0,76,128a4,4,0,0,0,8,0,76,76,0,0,1,7.9-33.76L187.13,199A75.37,75.37,0,0,1,160,204Zm84-76a83.86,83.86,0,0,1-21.34,55.94,4,4,0,1,1-6-5.33A76,76,0,0,0,115,66.75a4,4,0,0,1-4.74-6.45A84,84,0,0,1,244,128Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M56.88,31.93A12,12,0,1,0,39.12,48.07L71.79,84A68,68,0,0,0,72,220h88a91.26,91.26,0,0,0,30.66-5.24l8.46,9.31a12,12,0,0,0,17.76-16.14ZM160,196H72a44,44,0,0,1-1.8-87.95A91.91,91.91,0,0,0,68,128a12,12,0,0,0,24,0,68.22,68.22,0,0,1,2.66-18.84l77.88,85.67A68.67,68.67,0,0,1,160,196Zm92-68a91.32,91.32,0,0,1-17.53,54,12,12,0,1,1-19.41-14.11,68,68,0,0,0-89.57-98.53,12,12,0,0,1-12.2-20.66A92,92,0,0,1,252,128Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M52.44,36A6,6,0,0,0,43.56,44l40.18,44.2c-.45.87-.9,1.75-1.32,2.64A62,62,0,1,0,72,214h88a85.23,85.23,0,0,0,32.35-6.3L203.56,220a6,6,0,0,0,8.88-8.08ZM160,202H72a50,50,0,1,1,5.9-99.64A86.25,86.25,0,0,0,74,128a6,6,0,0,0,12,0,73.92,73.92,0,0,1,6.44-30.2l91.22,100.34A73.65,73.65,0,0,1,160,202Zm86-74a85.85,85.85,0,0,1-21.85,57.27,6,6,0,0,1-4.47,2,6,6,0,0,1-4.47-10,74,74,0,0,0-99-108.92,6,6,0,1,1-7.11-9.67A86,86,0,0,1,246,128Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M53.92,34.62A8,8,0,1,0,42.08,45.38L81.32,88.55l-.06.12A65,65,0,0,0,72,88a64,64,0,0,0,0,128h88a87.34,87.34,0,0,0,31.8-5.93l10.28,11.31a8,8,0,1,0,11.84-10.76ZM160,200H72a48,48,0,0,1,0-96c1.1,0,2.2,0,3.3.12A88.4,88.4,0,0,0,72,128a8,8,0,0,0,16,0,72.25,72.25,0,0,1,5.06-26.54l87,95.7A71.66,71.66,0,0,1,160,200Zm88-72a87.89,87.89,0,0,1-22.35,58.61A8,8,0,0,1,213.71,176,72,72,0,0,0,117.37,70a8,8,0,0,1-9.48-12.89A88,88,0,0,1,248,128Z"></path>
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
