//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "arrows"))]
#[component]
pub fn ArrowFatUp(
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
                <path d="M231.39,123.06A8,8,0,0,1,224,128H184v80a16,16,0,0,1-16,16H88a16,16,0,0,1-16-16V128H32a8,8,0,0,1-5.66-13.66l96-96a8,8,0,0,1,11.32,0l96,96A8,8,0,0,1,231.39,123.06Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M224,120H176v88a8,8,0,0,1-8,8H88a8,8,0,0,1-8-8V120H32l96-96Z" opacity="0.2"></path>
    <path d="M229.66,114.34l-96-96a8,8,0,0,0-11.32,0l-96,96A8,8,0,0,0,32,128H72v80a16,16,0,0,0,16,16h80a16,16,0,0,0,16-16V128h40a8,8,0,0,0,5.66-13.66ZM176,112a8,8,0,0,0-8,8v88H88V120a8,8,0,0,0-8-8H51.31L128,35.31,204.69,112Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M226.83,117.17l-96-96a4,4,0,0,0-5.66,0l-96,96A4,4,0,0,0,32,124H76v84a12,12,0,0,0,12,12h80a12,12,0,0,0,12-12V124h44a4,4,0,0,0,2.83-6.83ZM176,116a4,4,0,0,0-4,4v88a4,4,0,0,1-4,4H88a4,4,0,0,1-4-4V120a4,4,0,0,0-4-4H41.66L128,29.66,214.34,116Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M232.48,111.51l-96-96a12,12,0,0,0-17,0l-96,96A12,12,0,0,0,32,132H68v76a20,20,0,0,0,20,20h80a20,20,0,0,0,20-20V132h36a12,12,0,0,0,8.48-20.49ZM176,108a12,12,0,0,0-12,12v84H92V120a12,12,0,0,0-12-12H61l67-67,67,67Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M228.24,115.76l-96-96a6,6,0,0,0-8.48,0l-96,96A6,6,0,0,0,32,126H74v82a14,14,0,0,0,14,14h80a14,14,0,0,0,14-14V126h42a6,6,0,0,0,4.24-10.24ZM176,114a6,6,0,0,0-6,6v88a2,2,0,0,1-2,2H88a2,2,0,0,1-2-2V120a6,6,0,0,0-6-6H46.49L128,32.49,209.51,114Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M229.66,114.34l-96-96a8,8,0,0,0-11.32,0l-96,96A8,8,0,0,0,32,128H72v80a16,16,0,0,0,16,16h80a16,16,0,0,0,16-16V128h40a8,8,0,0,0,5.66-13.66ZM176,112a8,8,0,0,0-8,8v88H88V120a8,8,0,0,0-8-8H51.31L128,35.31,204.69,112Z"></path>
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
