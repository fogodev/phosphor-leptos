//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "people"))]
#[component]
pub fn GenderNonbinary(
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
                <path d="M200,24H56A16,16,0,0,0,40,40V216a16,16,0,0,0,16,16H200a16,16,0,0,0,16-16V40A16,16,0,0,0,200,24ZM128,208a52,52,0,0,1-8-103.38V83.82L91,95.43A8,8,0,0,1,85,80.57L106.46,72,85,63.43A8,8,0,0,1,91,48.57l37,14.81,37-14.81A8,8,0,1,1,171,63.43L149.54,72,171,80.57A8,8,0,0,1,165,95.43L136,83.82v20.8A52,52,0,0,1,128,208Zm36-52a36,36,0,1,1-36-36A36,36,0,0,1,164,156Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M192,168a64,64,0,1,1-64-64A64,64,0,0,1,192,168Z" opacity="0.2"></path>
    <path d="M136,96.46V62.13l27.88,16.73a8,8,0,1,0,8.24-13.72L143.55,48l28.57-17.14a8,8,0,0,0-8.24-13.72L128,38.67,92.12,17.14a8,8,0,0,0-8.24,13.72L112.45,48,83.88,65.14a8,8,0,0,0,8.24,13.72L120,62.13V96.46a72,72,0,1,0,16,0ZM128,224a56,56,0,1,1,56-56A56.06,56.06,0,0,1,128,224Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M132,100.13V55.07l33.94,20.36a4,4,0,1,0,4.12-6.86L135.77,48l34.29-20.57a4,4,0,1,0-4.12-6.86L128,43.34,90.06,20.57a4,4,0,1,0-4.12,6.86L120.23,48,85.94,68.57a4,4,0,0,0,4.12,6.86L124,55.07v45.06a68,68,0,1,0,8,0ZM128,228a60,60,0,1,1,60-60A60.07,60.07,0,0,1,128,228Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M140,93V69.2l21.83,13.09a12,12,0,1,0,12.34-20.58L151.32,48l22.85-13.71a12,12,0,0,0-12.34-20.58L128,34,94.17,13.71A12,12,0,0,0,81.83,34.29L104.68,48,81.83,61.71A12,12,0,1,0,94.17,82.29L116,69.2V93a76,76,0,1,0,24,0ZM128,220a52,52,0,1,1,52-52A52.06,52.06,0,0,1,128,220Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M134,98.27V58.6l30.91,18.54a6,6,0,1,0,6.18-10.28L139.66,48l31.43-18.85a6,6,0,1,0-6.18-10.29L128,41,91.09,18.86a6,6,0,1,0-6.18,10.29L116.34,48,84.91,66.86a6,6,0,1,0,6.18,10.28L122,58.6V98.27a70,70,0,1,0,12,0ZM128,226a58,58,0,1,1,58-58A58.07,58.07,0,0,1,128,226Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M136,96.46V62.13l27.88,16.73a8,8,0,1,0,8.24-13.72L143.55,48l28.57-17.14a8,8,0,0,0-8.24-13.72L128,38.67,92.12,17.14a8,8,0,0,0-8.24,13.72L112.45,48,83.88,65.14a8,8,0,0,0,8.24,13.72L120,62.13V96.46a72,72,0,1,0,16,0ZM128,224a56,56,0,1,1,56-56A56.06,56.06,0,0,1,128,224Z"></path>
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
