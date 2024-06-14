//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "media", feature = "system"))]
#[component]
pub fn SpeakerSimpleSlash(
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
                <path d="M221.92,210.62a8,8,0,1,1-11.84,10.76L168,175.09v48.6a8.29,8.29,0,0,1-3.91,7.18,8,8,0,0,1-9-.56L85.25,176H40a16,16,0,0,1-16-16V96A16,16,0,0,1,40,80H81.55L50.08,45.38A8,8,0,0,1,61.92,34.62ZM200.53,160a8.17,8.17,0,0,0,7.47-8.25V104.27A8.17,8.17,0,0,0,200.53,96a8,8,0,0,0-8.53,8v48A8,8,0,0,0,200.53,160ZM161,119.87a4,4,0,0,0,7-2.7V32.24a8.25,8.25,0,0,0-2.88-6.39,8,8,0,0,0-10-.16L111.83,59.33a4,4,0,0,0-.5,5.85ZM231.47,80A8.17,8.17,0,0,0,224,88.27v79.46a8.17,8.17,0,0,0,7.47,8.25,8,8,0,0,0,8.53-8V88A8,8,0,0,0,231.47,80Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M160,32V224L88,168H40a8,8,0,0,1-8-8V96a8,8,0,0,1,8-8H88Z" opacity="0.2"></path>
    <path d="M192,152V104a8,8,0,0,1,16,0v48a8,8,0,0,1-16,0Zm40-72a8,8,0,0,0-8,8v80a8,8,0,0,0,16,0V88A8,8,0,0,0,232,80ZM221.92,210.62a8,8,0,1,1-11.84,10.76L168,175.09V224a8,8,0,0,1-12.91,6.31L85.25,176H40a16,16,0,0,1-16-16V96A16,16,0,0,1,40,80H81.55L50.08,45.38A8,8,0,0,1,61.92,34.62ZM152,157.49,96.1,96H40v64H88a7.94,7.94,0,0,1,4.91,1.69L152,207.64ZM125.06,69.31l26.94-21v58.47a8,8,0,0,0,16,0V32a8,8,0,0,0-12.91-6.31l-39.85,31a8,8,0,0,0,9.82,12.63Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M196,152V104a4,4,0,0,1,8,0v48a4,4,0,0,1-8,0Zm36-68a4,4,0,0,0-4,4v80a4,4,0,0,0,8,0V88A4,4,0,0,0,232,84ZM219,213.31a4,4,0,1,1-5.92,5.38l-49-53.94V224a4,4,0,0,1-2.24,3.59A3.92,3.92,0,0,1,160,228a4.07,4.07,0,0,1-2.46-.84L86.63,172H40a12,12,0,0,1-12-12V96A12,12,0,0,1,40,84H90.59L53,42.69A4,4,0,0,1,59,37.31ZM156,156,97.87,92H40a4,4,0,0,0-4,4v64a4,4,0,0,0,4,4H88a4.06,4.06,0,0,1,2.46.84l65.54,51Zm-33.39-89.8,33.39-26v66.65a4,4,0,0,0,8,0V32a4,4,0,0,0-6.46-3.16l-39.85,31a4,4,0,1,0,4.92,6.31Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M248,88v80a12,12,0,0,1-24,0V88a12,12,0,0,1,24,0ZM224.88,207.93a12,12,0,1,1-17.76,16.14L172,185.44V224a12,12,0,0,1-19.37,9.47L83.88,180H40a20,20,0,0,1-20-20V96A20,20,0,0,1,40,76H72.51L47.12,48.07A12,12,0,0,1,64.88,31.93ZM148,159,94.33,100H44v56H88a12,12,0,0,1,7.37,2.53L148,199.46Zm-14.15-91.5,14.15-11v38.4a12,12,0,0,0,24,0V32a12,12,0,0,0-19.37-9.47L119.12,48.6a12,12,0,1,0,14.73,18.94ZM200,150.94a12,12,0,0,0,12-12V104a12,12,0,0,0-24,0v34.94A12,12,0,0,0,200,150.94Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M194,152V104a6,6,0,0,1,12,0v48a6,6,0,0,1-12,0Zm38-70a6,6,0,0,0-6,6v80a6,6,0,0,0,12,0V88A6,6,0,0,0,232,82ZM220.44,212a6,6,0,0,1-8.88,8.08L166,169.92V224a6,6,0,0,1-9.68,4.74L85.94,174H40a14,14,0,0,1-14-14V96A14,14,0,0,1,40,82H86.07L51.56,44A6,6,0,0,1,60.44,36ZM154,156.72,97,94H40a2,2,0,0,0-2,2v64a2,2,0,0,0,2,2H88a6,6,0,0,1,3.68,1.26L154,211.73Zm-30.17-89L154,44.27v62.56a6,6,0,0,0,12,0V32a6,6,0,0,0-9.68-4.74l-39.85,31a6,6,0,1,0,7.36,9.47Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M192,152V104a8,8,0,0,1,16,0v48a8,8,0,0,1-16,0Zm40-72a8,8,0,0,0-8,8v80a8,8,0,0,0,16,0V88A8,8,0,0,0,232,80ZM221.92,210.62a8,8,0,1,1-11.84,10.76L168,175.09V224a8,8,0,0,1-12.91,6.31L85.25,176H40a16,16,0,0,1-16-16V96A16,16,0,0,1,40,80H81.55L50.08,45.38A8,8,0,0,1,61.92,34.62ZM152,157.49,96.1,96H40v64H88a7.94,7.94,0,0,1,4.91,1.69L152,207.64ZM125.06,69.31l26.94-21v58.47a8,8,0,0,0,16,0V32a8,8,0,0,0-12.91-6.31l-39.85,31a8,8,0,0,0,9.82,12.63Z"></path>
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
