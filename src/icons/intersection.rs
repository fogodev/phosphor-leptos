//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "finance", feature = "development"))]
#[component]
pub fn Intersection(
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
                <path d="M208,32H48A16,16,0,0,0,32,48V208a16,16,0,0,0,16,16H208a16,16,0,0,0,16-16V48A16,16,0,0,0,208,32ZM184,176a8,8,0,0,1-16,0V120a40,40,0,0,0-80,0v56a8,8,0,0,1-16,0V120a56,56,0,0,1,112,0Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M200,120v80H56V120a72,72,0,0,1,144,0Z" opacity="0.2"></path>
    <path d="M208,120v80a8,8,0,0,1-16,0V120a64,64,0,0,0-128,0v80a8,8,0,0,1-16,0V120a80,80,0,0,1,160,0Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M204,120v80a4,4,0,0,1-8,0V120a68,68,0,0,0-136,0v80a4,4,0,0,1-8,0V120a76,76,0,0,1,152,0Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M212,120v80a12,12,0,0,1-24,0V120a60,60,0,0,0-120,0v80a12,12,0,0,1-24,0V120a84,84,0,0,1,168,0Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M206,120v80a6,6,0,0,1-12,0V120a66,66,0,0,0-132,0v80a6,6,0,0,1-12,0V120a78,78,0,0,1,156,0Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M208,120v80a8,8,0,0,1-16,0V120a64,64,0,0,0-128,0v80a8,8,0,0,1-16,0V120a80,80,0,0,1,160,0Z"></path>
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
