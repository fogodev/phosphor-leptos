//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "design", feature = "editor", feature = "objects"))]
#[component]
pub fn Palette(
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
                <path d="M200.77,53.89A103.27,103.27,0,0,0,128,24h-1.07A104,104,0,0,0,24,128c0,43,26.58,79.06,69.36,94.17A32,32,0,0,0,136,192a16,16,0,0,1,16-16h46.21a31.81,31.81,0,0,0,31.2-24.88,104.43,104.43,0,0,0,2.59-24A103.28,103.28,0,0,0,200.77,53.89ZM84,168a12,12,0,1,1,12-12A12,12,0,0,1,84,168Zm0-56a12,12,0,1,1,12-12A12,12,0,0,1,84,112Zm44-24a12,12,0,1,1,12-12A12,12,0,0,1,128,88Zm44,24a12,12,0,1,1,12-12A12,12,0,0,1,172,112Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M224,127.17a96.48,96.48,0,0,1-2.39,22.18A24,24,0,0,1,198.21,168H152a24,24,0,0,0-24,24,24,24,0,0,1-32,22.61C58.73,201.44,32,169.81,32,128a96,96,0,0,1,95-96C179.84,31.47,223.55,74.35,224,127.17Z"
        opacity="0.2"
    ></path>
    <path d="M200.77,53.89A103.27,103.27,0,0,0,128,24h-1.07A104,104,0,0,0,24,128c0,43,26.58,79.06,69.36,94.17A32,32,0,0,0,136,192a16,16,0,0,1,16-16h46.21a31.81,31.81,0,0,0,31.2-24.88,104.43,104.43,0,0,0,2.59-24A103.28,103.28,0,0,0,200.77,53.89Zm13,93.71A15.89,15.89,0,0,1,198.21,160H152a32,32,0,0,0-32,32,16,16,0,0,1-21.31,15.07C62.49,194.3,40,164,40,128a88,88,0,0,1,87.09-88h.9a88.35,88.35,0,0,1,88,87.25A88.86,88.86,0,0,1,213.81,147.6ZM140,76a12,12,0,1,1-12-12A12,12,0,0,1,140,76ZM96,100A12,12,0,1,1,84,88,12,12,0,0,1,96,100Zm0,56a12,12,0,1,1-12-12A12,12,0,0,1,96,156Zm88-56a12,12,0,1,1-12-12A12,12,0,0,1,184,100Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M198,56.74A99.31,99.31,0,0,0,128,28h-1A100,100,0,0,0,28,128c0,41.22,25.55,75.85,66.69,90.38a28.34,28.34,0,0,0,9.42,1.63A28,28,0,0,0,132,192a20,20,0,0,1,20-20h46.21a27.84,27.84,0,0,0,27.3-21.76,100.37,100.37,0,0,0,2.49-23.1A99.26,99.26,0,0,0,198,56.74Zm19.74,91.72A19.89,19.89,0,0,1,198.21,164H152a28,28,0,0,0-28,28,20,20,0,0,1-26.64,18.83C59.51,197.46,36,165.72,36,128a92,92,0,0,1,91.05-92H128a92,92,0,0,1,89.72,112.46ZM136,76a8,8,0,1,1-8-8A8,8,0,0,1,136,76ZM92,100a8,8,0,1,1-8-8A8,8,0,0,1,92,100Zm0,56a8,8,0,1,1-8-8A8,8,0,0,1,92,156Zm88-56a8,8,0,1,1-8-8A8,8,0,0,1,180,100Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M203.57,51A107.9,107.9,0,0,0,20,128c0,44.72,27.6,82.25,72,97.94A36,36,0,0,0,140,192a12,12,0,0,1,12-12h46.21a35.79,35.79,0,0,0,35.1-28A108.6,108.6,0,0,0,236,127.09,107.23,107.23,0,0,0,203.57,51Zm6.34,95.67a11.91,11.91,0,0,1-11.7,9.3H152a36,36,0,0,0-36,36,12,12,0,0,1-16,11.3c-16.65-5.88-30.65-15.76-40.48-28.56A76,76,0,0,1,44,128a84,84,0,0,1,83.13-84H128a84.35,84.35,0,0,1,84,83.29A84.72,84.72,0,0,1,209.91,146.71ZM144,76a16,16,0,1,1-16-16A16,16,0,0,1,144,76Zm-44,24A16,16,0,1,1,84,84,16,16,0,0,1,100,100Zm0,56a16,16,0,1,1-16-16A16,16,0,0,1,100,156Zm88-56a16,16,0,1,1-16-16A16,16,0,0,1,188,100Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M199.37,55.31A101.32,101.32,0,0,0,128,26h-1A102,102,0,0,0,26,128c0,42.09,26.07,77.44,68,92.26A30.21,30.21,0,0,0,104.11,222,30.06,30.06,0,0,0,134,192a18,18,0,0,1,18-18h46.21a29.82,29.82,0,0,0,29.25-23.31A102.71,102.71,0,0,0,230,127.11,101.25,101.25,0,0,0,199.37,55.31ZM215.76,148a17.89,17.89,0,0,1-17.55,14H152a30,30,0,0,0-30,30,18,18,0,0,1-24,17C61,195.86,38,164.85,38,128a90,90,0,0,1,89.07-90H128a90.34,90.34,0,0,1,90,89.22A90.46,90.46,0,0,1,215.76,148ZM138,76a10,10,0,1,1-10-10A10,10,0,0,1,138,76ZM94,100A10,10,0,1,1,84,90,10,10,0,0,1,94,100Zm0,56a10,10,0,1,1-10-10A10,10,0,0,1,94,156Zm88-56a10,10,0,1,1-10-10A10,10,0,0,1,182,100Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M200.77,53.89A103.27,103.27,0,0,0,128,24h-1.07A104,104,0,0,0,24,128c0,43,26.58,79.06,69.36,94.17A32,32,0,0,0,136,192a16,16,0,0,1,16-16h46.21a31.81,31.81,0,0,0,31.2-24.88,104.43,104.43,0,0,0,2.59-24A103.28,103.28,0,0,0,200.77,53.89Zm13,93.71A15.89,15.89,0,0,1,198.21,160H152a32,32,0,0,0-32,32,16,16,0,0,1-21.31,15.07C62.49,194.3,40,164,40,128a88,88,0,0,1,87.09-88h.9a88.35,88.35,0,0,1,88,87.25A88.86,88.86,0,0,1,213.81,147.6ZM140,76a12,12,0,1,1-12-12A12,12,0,0,1,140,76ZM96,100A12,12,0,1,1,84,88,12,12,0,0,1,96,100Zm0,56a12,12,0,1,1-12-12A12,12,0,0,1,96,156Zm88-56a12,12,0,1,1-12-12A12,12,0,0,1,184,100Z"></path>
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
