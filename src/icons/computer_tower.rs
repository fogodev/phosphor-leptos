//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "development", feature = "objects"))]
#[component]
pub fn ComputerTower(
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
                <path d="M192,24H64A16,16,0,0,0,48,40V216a16,16,0,0,0,16,16H192a16,16,0,0,0,16-16V40A16,16,0,0,0,192,24ZM128,192a12,12,0,1,1,12-12A12,12,0,0,1,128,192Zm32-80H96a8,8,0,0,1,0-16h64a8,8,0,0,1,0,16Zm0-32H96a8,8,0,0,1,0-16h64a8,8,0,0,1,0,16Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M200,40V216a8,8,0,0,1-8,8H64a8,8,0,0,1-8-8V40a8,8,0,0,1,8-8H192A8,8,0,0,1,200,40Z"
        opacity="0.2"
    ></path>
    <path d="M88,72a8,8,0,0,1,8-8h64a8,8,0,0,1,0,16H96A8,8,0,0,1,88,72Zm8,40h64a8,8,0,0,0,0-16H96a8,8,0,0,0,0,16ZM208,40V216a16,16,0,0,1-16,16H64a16,16,0,0,1-16-16V40A16,16,0,0,1,64,24H192A16,16,0,0,1,208,40Zm-16,0H64V216H192ZM128,168a12,12,0,1,0,12,12A12,12,0,0,0,128,168Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M164,72a4,4,0,0,1-4,4H96a4,4,0,0,1,0-8h64A4,4,0,0,1,164,72Zm-4,28H96a4,4,0,0,0,0,8h64a4,4,0,0,0,0-8Zm44-60V216a12,12,0,0,1-12,12H64a12,12,0,0,1-12-12V40A12,12,0,0,1,64,28H192A12,12,0,0,1,204,40Zm-8,0a4,4,0,0,0-4-4H64a4,4,0,0,0-4,4V216a4,4,0,0,0,4,4H192a4,4,0,0,0,4-4ZM128,172a8,8,0,1,0,8,8A8,8,0,0,0,128,172Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M84,76A12,12,0,0,1,96,64h64a12,12,0,0,1,0,24H96A12,12,0,0,1,84,76Zm12,52h64a12,12,0,0,0,0-24H96a12,12,0,0,0,0,24ZM212,40V216a20,20,0,0,1-20,20H64a20,20,0,0,1-20-20V40A20,20,0,0,1,64,20H192A20,20,0,0,1,212,40Zm-24,4H68V212H188ZM128,168a16,16,0,1,0,16,16A16,16,0,0,0,128,168Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M166,72a6,6,0,0,1-6,6H96a6,6,0,0,1,0-12h64A6,6,0,0,1,166,72Zm-6,26H96a6,6,0,0,0,0,12h64a6,6,0,0,0,0-12Zm46-58V216a14,14,0,0,1-14,14H64a14,14,0,0,1-14-14V40A14,14,0,0,1,64,26H192A14,14,0,0,1,206,40Zm-12,0a2,2,0,0,0-2-2H64a2,2,0,0,0-2,2V216a2,2,0,0,0,2,2H192a2,2,0,0,0,2-2ZM128,170a10,10,0,1,0,10,10A10,10,0,0,0,128,170Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M88,72a8,8,0,0,1,8-8h64a8,8,0,0,1,0,16H96A8,8,0,0,1,88,72Zm8,40h64a8,8,0,0,0,0-16H96a8,8,0,0,0,0,16ZM208,40V216a16,16,0,0,1-16,16H64a16,16,0,0,1-16-16V40A16,16,0,0,1,64,24H192A16,16,0,0,1,208,40Zm-16,0H64V216H192ZM128,168a12,12,0,1,0,12,12A12,12,0,0,0,128,168Z"></path>
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
