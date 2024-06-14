//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "objects"))]
#[component]
pub fn Shower(
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
                <path d="M64,236a12,12,0,1,1-12-12A12,12,0,0,1,64,236Zm20-44a12,12,0,1,0,12,12A12,12,0,0,0,84,192Zm-64,0a12,12,0,1,0,12,12A12,12,0,0,0,20,192Zm32-32a12,12,0,1,0,12,12A12,12,0,0,0,52,160ZM248,32H216a8,8,0,0,0-5.66,2.34l-30.2,30.2L53.38,86.19a16,16,0,0,0-8.69,27.1l98,98a16,16,0,0,0,27.09-8.66L191.46,75.86,219.31,48H248a8,8,0,0,0,0-16Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M184,72,161.91,201.31a8,8,0,0,1-13.54,4.34l-98-98a8,8,0,0,1,4.34-13.54Z"
        opacity="0.2"
    ></path>
    <path d="M64,236a12,12,0,1,1-12-12A12,12,0,0,1,64,236Zm20-44a12,12,0,1,0,12,12A12,12,0,0,0,84,192Zm-64,0a12,12,0,1,0,12,12A12,12,0,0,0,20,192Zm32-32a12,12,0,1,0,12,12A12,12,0,0,0,52,160ZM256,40a8,8,0,0,1-8,8H219.31L191.46,75.86,169.8,202.65a16,16,0,0,1-27.09,8.66l-98-98a16,16,0,0,1,8.69-27.1L180.14,64.54l30.2-30.2A8,8,0,0,1,216,32h32A8,8,0,0,1,256,40ZM174.21,81.79,56,102l98,98Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M60,236a8,8,0,1,1-8-8A8,8,0,0,1,60,236Zm24-40a8,8,0,1,0,8,8A8,8,0,0,0,84,196Zm-64,0a8,8,0,1,0,8,8A8,8,0,0,0,20,196Zm32-32a8,8,0,1,0,8,8A8,8,0,0,0,52,164ZM252,40a4,4,0,0,1-4,4H217.66L187.73,73.93,165.86,202a12,12,0,0,1-8.17,9.44A12.09,12.09,0,0,1,154,212a12,12,0,0,1-8.46-3.52l-98-98A12,12,0,0,1,54,90.14l128-21.87,31.1-31.1A4,4,0,0,1,216,36h32A4,4,0,0,1,252,40ZM179.11,76.89,55.37,98a4,4,0,0,0-2.19,6.78l98,98a4,4,0,0,0,6.78-2.17Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M68,236a16,16,0,1,1-16-16A16,16,0,0,1,68,236Zm16-48a16,16,0,1,0,16,16A16,16,0,0,0,84,188Zm-64,0a16,16,0,1,0,16,16A16,16,0,0,0,20,188Zm32,0a16,16,0,1,0-16-16A16,16,0,0,0,52,188ZM256,40a12,12,0,0,1-12,12H221L195.19,77.79,173.74,203.33a20,20,0,0,1-33.86,10.8l-98-98A20,20,0,0,1,52.72,82.25l125.5-21.44,29.29-29.3A12,12,0,0,1,216,28h28A12,12,0,0,1,256,40ZM169.32,86.68l-105,17.94,87.07,87.07Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M62,236a10,10,0,1,1-10-10A10,10,0,0,1,62,236Zm22-42a10,10,0,1,0,10,10A10,10,0,0,0,84,194Zm-64,0a10,10,0,1,0,10,10A10,10,0,0,0,20,194Zm32-32a10,10,0,1,0,10,10A10,10,0,0,0,52,162ZM254,40a6,6,0,0,1-6,6H218.49l-28.9,28.89L167.83,202.32a14,14,0,0,1-23.71,7.57l-98-98a14,14,0,0,1,7.6-23.71l127.4-21.76,30.65-30.65A6,6,0,0,1,216,34h32A6,6,0,0,1,254,40ZM176.66,79.34,55.7,100a2,2,0,0,0-1.11,3.39l98,98a2,2,0,0,0,3.39-1.09Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M64,236a12,12,0,1,1-12-12A12,12,0,0,1,64,236Zm20-44a12,12,0,1,0,12,12A12,12,0,0,0,84,192Zm-64,0a12,12,0,1,0,12,12A12,12,0,0,0,20,192Zm32-32a12,12,0,1,0,12,12A12,12,0,0,0,52,160ZM256,40a8,8,0,0,1-8,8H219.31L191.46,75.86,169.8,202.65a16,16,0,0,1-27.09,8.66l-98-98a16,16,0,0,1,8.69-27.1L180.14,64.54l30.2-30.2A8,8,0,0,1,216,32h32A8,8,0,0,1,256,40ZM174.21,81.79,56,102l98,98Z"></path>
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
