//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn DropHalf(
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
                <path d="M134.88,6.17a12,12,0,0,0-13.76,0,259,259,0,0,0-42.18,39C50.85,77.43,36,111.63,36,144a92,92,0,0,0,184,0C220,66.64,138.36,8.6,134.88,6.17ZM194.08,160H140V144h56A68,68,0,0,1,194.08,160ZM140,120V104h47a115,115,0,0,1,5.68,16Zm19.3-58.71A197.29,197.29,0,0,1,173.68,80H140V41.46A243.5,243.5,0,0,1,159.3,61.29ZM60,144c0-33.31,20-63.37,36.7-82.71A243.5,243.5,0,0,1,116,41.46V210.92A68.1,68.1,0,0,1,60,144Zm80,66.92V184h42.94A68,68,0,0,1,140,210.92Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M208,144a80,80,0,0,1-80,80V16S208,72,208,144Z" opacity="0.2"></path>
    <path d="M174,47.75a254.19,254.19,0,0,0-41.45-38.3,8,8,0,0,0-9.18,0A254.19,254.19,0,0,0,82,47.75C54.51,79.32,40,112.6,40,144a88,88,0,0,0,176,0C216,112.6,201.49,79.32,174,47.75ZM56,144c0-50,42.26-92.71,64-111.4V215.54A72.08,72.08,0,0,1,56,144Zm80,71.54V32.6C157.74,51.29,200,94,200,144A72.08,72.08,0,0,1,136,215.54Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M174,47.75a254.19,254.19,0,0,0-41.45-38.3,8,8,0,0,0-9.18,0A254.19,254.19,0,0,0,82,47.75C54.51,79.32,40,112.6,40,144a88,88,0,0,0,176,0C216,112.6,201.49,79.32,174,47.75ZM56,144c0-57.23,55.47-105,72-118V216A72.08,72.08,0,0,1,56,144Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M172.53,49.06a251.42,251.42,0,0,0-41.09-38,6,6,0,0,0-6.88,0,251.42,251.42,0,0,0-41.09,38C56.34,80.26,42,113.09,42,144a86,86,0,0,0,172,0C214,113.09,199.66,80.26,172.53,49.06ZM202,144a75,75,0,0,1-.69,10H134V134h67.44A92.09,92.09,0,0,1,202,144ZM186.8,90H134V70h39.89A176,176,0,0,1,186.8,90ZM134,198h44.52A73.76,73.76,0,0,1,134,217.74Zm0-12V166h64.66a74.05,74.05,0,0,1-9.78,20Zm0-64V102h58.7a117.43,117.43,0,0,1,6.69,20Zm30.29-64H134V28.3A257.09,257.09,0,0,1,164.29,58ZM54,144c0-53.42,47.35-98.56,68-115.7V217.74A74.09,74.09,0,0,1,54,144Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M174,47.75a254.19,254.19,0,0,0-41.45-38.3,8,8,0,0,0-9.18,0A254.19,254.19,0,0,0,82,47.75C54.51,79.32,40,112.6,40,144a88,88,0,0,0,176,0C216,112.6,201.49,79.32,174,47.75ZM200,144a70.57,70.57,0,0,1-.46,8H136V136h63.64Q200,140,200,144ZM183.39,88H136V72h36.89A175.85,175.85,0,0,1,183.39,88ZM136,200h37.19A71.67,71.67,0,0,1,136,215.54Zm0-16V168h59.87a72,72,0,0,1-8,16Zm0-64V104h55.39a116.84,116.84,0,0,1,5.45,16Zm23.89-64H136V32.6A257.22,257.22,0,0,1,159.89,56ZM56,144c0-50,42.26-92.71,64-111.4V215.54A72.08,72.08,0,0,1,56,144Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M171,50.37a250.18,250.18,0,0,0-40.73-37.65,4,4,0,0,0-4.58,0A250.18,250.18,0,0,0,85,50.37C58.17,81.21,44,113.58,44,144a84,84,0,0,0,168,0C212,113.58,197.83,81.21,171,50.37ZM204,144a75.41,75.41,0,0,1-1,12H132V132h71.21A93.38,93.38,0,0,1,204,144ZM190.14,92H132V68h43A176.56,176.56,0,0,1,190.14,92ZM132,164h69.31a75.63,75.63,0,0,1-11.4,24H132Zm0-40V100h62a120.07,120.07,0,0,1,7.88,24Zm33.1-68.23q1.77,2,3.59,4.23H132V24.07A256.44,256.44,0,0,1,165.1,55.77ZM52,144c0-35.9,21.15-67.8,38.9-88.23A256.44,256.44,0,0,1,124,24.07V219.89A76.09,76.09,0,0,1,52,144Zm80,75.89V196h51.35A75.79,75.79,0,0,1,132,219.89Z"></path>
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
