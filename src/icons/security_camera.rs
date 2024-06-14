//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "objects", feature = "system"))]
#[component]
pub fn SecurityCamera(
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
                <path d="M248,136a8,8,0,0,0-8,8v16H195.31L177,141.66l50.34-50.35a16,16,0,0,0,0-22.62L189.66,31h0L171.31,12.69a16,16,0,0,0-22.63,0L2.92,158.94A10,10,0,0,0,10,176H49.37l35.32,35.31a16,16,0,0,0,22.62,0L165.66,153,184,171.31A15.86,15.86,0,0,0,195.31,176H240v16a8,8,0,0,0,16,0V144A8,8,0,0,0,248,136ZM160,24l12.69,12.69L49.37,160H24.46Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M221.66,85.66l-120,120a8,8,0,0,1-11.32,0L52.69,168,184,36.69l37.66,37.65A8,8,0,0,1,221.66,85.66Z"
        opacity="0.2"
    ></path>
    <path d="M248,136a8,8,0,0,0-8,8v16H195.31L177,141.66l50.34-50.35a16,16,0,0,0,0-22.62l-56-56a16,16,0,0,0-22.63,0L2.92,158.94A10,10,0,0,0,10,176H49.37l35.32,35.31a16,16,0,0,0,22.62,0L165.66,153,184,171.31A15.86,15.86,0,0,0,195.31,176H240v16a8,8,0,0,0,16,0V144A8,8,0,0,0,248,136ZM160,24l12.69,12.69L49.37,160H24.46ZM96,200,64,168,184,48l32,32Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M248,140a4,4,0,0,0-4,4v20H195.31a4,4,0,0,1-2.82-1.17l-21.18-21.17,53.18-53.17a12,12,0,0,0,0-17l-56-56a12,12,0,0,0-17,0L5.76,161.76A6,6,0,0,0,10,172H51l36.48,36.49a12,12,0,0,0,17,0l61.18-61.18,21.17,21.17a11.9,11.9,0,0,0,8.48,3.52H244v20a4,4,0,0,0,8,0V144A4,4,0,0,0,248,140ZM157.17,21.17a4.1,4.1,0,0,1,5.66,0l15.51,15.52L51,164H14.82ZM98.83,202.83a4.1,4.1,0,0,1-5.66,0L58.34,168,184,42.34l34.83,34.83a4,4,0,0,1,0,5.66Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M244,132a12,12,0,0,0-12,12v12H197l-14.35-14.34,47.52-47.52a20,20,0,0,0,0-28.28l-56-56a20,20,0,0,0-28.32,0L6.42,150.47A22,22,0,0,0,22,188h33.7l26.14,26.14a20,20,0,0,0,28.29,0l55.51-55.51,15.52,15.51A19.86,19.86,0,0,0,195.31,180H232v12a12,12,0,0,0,24,0V144A12,12,0,0,0,244,132ZM160,29.67l15,15L55.71,164H26.81ZM96,194.34,77.65,176,192,61.66,210.34,80l-53.17,53.17h0Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M248,138a6,6,0,0,0-6,6v18H195.31a2,2,0,0,1-1.41-.59l-19.76-19.75L225.9,89.9a14,14,0,0,0,0-19.8l-56-56a14,14,0,0,0-19.81,0L4.34,160.35A8,8,0,0,0,10,174H50.2l35.9,35.9a14,14,0,0,0,19.8,0l59.76-59.76,19.75,19.76a13.94,13.94,0,0,0,9.9,4.1H242v18a6,6,0,0,0,12,0V144A6,6,0,0,0,248,138ZM158.59,22.59a2,2,0,0,1,2.82,0l14.1,14.1L50.2,162H19.64ZM97.41,201.41a2,2,0,0,1-2.82,0L61.17,168,184,45.17l33.41,33.42a2,2,0,0,1,0,2.82Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M248,136a8,8,0,0,0-8,8v16H195.31L177,141.66l50.34-50.35a16,16,0,0,0,0-22.62l-56-56a16,16,0,0,0-22.63,0L2.92,158.94A10,10,0,0,0,10,176H49.37l35.32,35.31a16,16,0,0,0,22.62,0L165.66,153,184,171.31A15.86,15.86,0,0,0,195.31,176H240v16a8,8,0,0,0,16,0V144A8,8,0,0,0,248,136ZM160,24l12.69,12.69L49.37,160H24.46ZM96,200,64,168,184,48l32,32Z"></path>
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
