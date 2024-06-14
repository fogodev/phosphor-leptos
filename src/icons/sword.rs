//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "games", feature = "objects"))]
#[component]
pub fn Sword(
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
                <path d="M216,32H152a8,8,0,0,0-6.34,3.12l-64,83.21L72,108.69a16,16,0,0,0-22.64,0l-8.69,8.7a16,16,0,0,0,0,22.63l22,22-32,32a16,16,0,0,0,0,22.63l8.69,8.68a16,16,0,0,0,22.62,0l32-32,22,22a16,16,0,0,0,22.64,0l8.69-8.7a16,16,0,0,0,0-22.63l-9.64-9.64,83.21-64A8,8,0,0,0,224,104V40A8,8,0,0,0,216,32Zm-8,68.06-81.74,62.88L115.32,152l50.34-50.34a8,8,0,0,0-11.32-11.31L104,140.68,93.07,129.74,155.94,48H208Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M141.66,201,129,213.66a8,8,0,0,1-11.32,0L92,188,58.35,221.66a8,8,0,0,1-11.32,0L34.34,209a8,8,0,0,1,0-11.31L68,164,42.34,138.36a8,8,0,0,1,0-11.32L55,114.34a8,8,0,0,1,11.32,0l75.3,75.3A8,8,0,0,1,141.66,201Z"
        opacity="0.2"
    ></path>
    <path d="M216,32H152a8,8,0,0,0-6.34,3.12l-64,83.21L72,108.69a16,16,0,0,0-22.64,0l-12.69,12.7a16,16,0,0,0,0,22.63l20,20-28,28a16,16,0,0,0,0,22.63l12.69,12.68a16,16,0,0,0,22.62,0l28-28,20,20a16,16,0,0,0,22.64,0l12.69-12.7a16,16,0,0,0,0-22.63l-9.64-9.64,83.21-64A8,8,0,0,0,224,104V40A8,8,0,0,0,216,32ZM52.69,216,40,203.32l28-28L80.68,188Zm70.61-8L48,132.71,60.7,120,136,195.31ZM208,100.06l-81.74,62.88L115.32,152l50.34-50.34a8,8,0,0,0-11.32-11.31L104,140.68,93.07,129.74,155.94,48H208Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M216,36H152a4,4,0,0,0-3.17,1.56L82.05,124.38,69.19,111.52a12,12,0,0,0-17,0L39.52,124.21a12,12,0,0,0,0,17L62.34,164,31.52,194.83a12,12,0,0,0,0,17L44.2,224.49a12,12,0,0,0,17,0L92,193.67l22.81,22.82a12,12,0,0,0,17,0l12.69-12.7a12,12,0,0,0,0-17L131.62,174l86.82-66.79A4,4,0,0,0,220,104V40A4,4,0,0,0,216,36ZM55.52,218.83a4,4,0,0,1-5.66,0L37.17,206.15a4,4,0,0,1,0-5.66L68,169.67,86.34,188Zm83.31-26.36a4,4,0,0,1,0,5.67l-12.7,12.69a4,4,0,0,1-5.66,0l-75.3-75.3a4,4,0,0,1,0-5.66l12.7-12.69a4,4,0,0,1,5.66,0ZM212,102l-86.08,66.22L109.66,152l53.17-53.17a4,4,0,1,0-5.66-5.66L104,146.34,87.75,130.08,154,44h58Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M216,28H152a12,12,0,0,0-9.33,4.45L79.5,110.51l-4.66-4.65a20,20,0,0,0-28.29,0L29.86,122.55a20,20,0,0,0,0,28.29h0L45,166,23.86,187.17a20,20,0,0,0,0,28.28l16.69,16.69a20,20,0,0,0,28.28,0L90,211l15.17,15.16a20,20,0,0,0,28.29,0l16.69-16.69a20,20,0,0,0,0-28.3l-4.65-4.65,78.06-63.17A12,12,0,0,0,228,104V40A12,12,0,0,0,216,28ZM54.69,212.34l-11-11L62,183l11,11Zm64.61-6L49.65,136.7l11.05-11,69.65,69.65ZM204,98.27l-75.58,61.17L121,152l47.51-47.5a12,12,0,0,0-17-17L104,135l-7.45-7.44L157.73,52H204Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M216,34H152a6,6,0,0,0-4.76,2.34l-65.39,85L70.6,110.1a14,14,0,0,0-19.8,0L38.1,122.8a14,14,0,0,0,0,19.81h0L59.51,164,30.1,193.42a14,14,0,0,0,0,19.8l12.69,12.69a14,14,0,0,0,19.8,0L92,196.5l21.4,21.4a14,14,0,0,0,19.8,0l12.7-12.69a14,14,0,0,0,0-19.81l-11.25-11.25,85-65.39A6,6,0,0,0,222,104V40A6,6,0,0,0,216,34ZM54.1,217.42a2,2,0,0,1-2.83,0L38.59,204.73a2,2,0,0,1,0-2.82L68,172.5,83.51,188Zm83.31-20.7-12.69,12.7a2,2,0,0,1-2.84,0l-75.29-75.3h0a2,2,0,0,1,0-2.83l12.69-12.7a2,2,0,0,1,2.84,0l75.29,75.3A2,2,0,0,1,137.41,196.72ZM210,101.05,126.09,165.6,112.49,152l51.75-51.76a6,6,0,0,0-8.48-8.48L104,143.51l-13.6-13.6L155,46H210Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M216,32H152a8,8,0,0,0-6.34,3.12l-64,83.21L72,108.69a16,16,0,0,0-22.64,0l-12.69,12.7a16,16,0,0,0,0,22.63l20,20-28,28a16,16,0,0,0,0,22.63l12.69,12.68a16,16,0,0,0,22.62,0l28-28,20,20a16,16,0,0,0,22.64,0l12.69-12.7a16,16,0,0,0,0-22.63l-9.64-9.64,83.21-64A8,8,0,0,0,224,104V40A8,8,0,0,0,216,32ZM52.69,216,40,203.32l28-28L80.68,188Zm70.61-8L48,132.71,60.7,120,136,195.31ZM208,100.06l-81.74,62.88L115.32,152l50.34-50.34a8,8,0,0,0-11.32-11.31L104,140.68,93.07,129.74,155.94,48H208Z"></path>
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
