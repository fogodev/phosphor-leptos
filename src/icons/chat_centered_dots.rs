//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "communication"))]
#[component]
pub fn ChatCenteredDots(
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
                <path d="M216,40H40A16,16,0,0,0,24,56V184a16,16,0,0,0,16,16h60.43l13.68,23.94a16,16,0,0,0,27.78,0L155.57,200H216a16,16,0,0,0,16-16V56A16,16,0,0,0,216,40ZM84,132a12,12,0,1,1,12-12A12,12,0,0,1,84,132Zm44,0a12,12,0,1,1,12-12A12,12,0,0,1,128,132Zm44,0a12,12,0,1,1,12-12A12,12,0,0,1,172,132Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M224,56V184a8,8,0,0,1-8,8H150.93l-16,28a8,8,0,0,1-13.9,0l-16-28H40a8,8,0,0,1-8-8V56a8,8,0,0,1,8-8H216A8,8,0,0,1,224,56Z"
        opacity="0.2"
    ></path>
    <path d="M116,120a12,12,0,1,1,12,12A12,12,0,0,1,116,120ZM84,132a12,12,0,1,0-12-12A12,12,0,0,0,84,132Zm88,0a12,12,0,1,0-12-12A12,12,0,0,0,172,132Zm60-76V184a16,16,0,0,1-16,16H155.57l-13.68,23.94a16,16,0,0,1-27.78,0L100.43,200H40a16,16,0,0,1-16-16V56A16,16,0,0,1,40,40H216A16,16,0,0,1,232,56Zm-16,0H40V184h65.07a8,8,0,0,1,7,4l16,28,16-28a8,8,0,0,1,7-4H216Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M136,120a8,8,0,1,1-8-8A8,8,0,0,1,136,120Zm-52-8a8,8,0,1,0,8,8A8,8,0,0,0,84,112Zm88,0a8,8,0,1,0,8,8A8,8,0,0,0,172,112Zm56-56V184a12,12,0,0,1-12,12H153.25L138.42,222a12,12,0,0,1-20.84,0L102.75,196H40a12,12,0,0,1-12-12V56A12,12,0,0,1,40,44H216A12,12,0,0,1,228,56Zm-8,0a4,4,0,0,0-4-4H40a4,4,0,0,0-4,4V184a4,4,0,0,0,4,4h65.07a4,4,0,0,1,3.47,2l16,28a4,4,0,0,0,6.94,0l16-28a4,4,0,0,1,3.47-2H216a4,4,0,0,0,4-4Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M88,120a16,16,0,1,1,16,16A16,16,0,0,1,88,120Zm64,16a16,16,0,1,0-16-16A16,16,0,0,0,152,136Zm84-80V184a20,20,0,0,1-20,20H157.89l-12.52,21.92a20,20,0,0,1-34.74,0L98.11,204H40a20,20,0,0,1-20-20V56A20,20,0,0,1,40,36H216A20,20,0,0,1,236,56Zm-24,4H44V180h61.07a12,12,0,0,1,10.42,6.05L128,207.94l12.51-21.89A12,12,0,0,1,150.93,180H212Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M138,120a10,10,0,1,1-10-10A10,10,0,0,1,138,120ZM84,110a10,10,0,1,0,10,10A10,10,0,0,0,84,110Zm88,0a10,10,0,1,0,10,10A10,10,0,0,0,172,110Zm58-54V184a14,14,0,0,1-14,14H154.41L140.16,223a14,14,0,0,1-24.32,0L101.59,198H40a14,14,0,0,1-14-14V56A14,14,0,0,1,40,42H216A14,14,0,0,1,230,56Zm-12,0a2,2,0,0,0-2-2H40a2,2,0,0,0-2,2V184a2,2,0,0,0,2,2h65.07a6,6,0,0,1,5.21,3l16,28a2,2,0,0,0,3.48,0l16-28a6,6,0,0,1,5.21-3H216a2,2,0,0,0,2-2Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M116,120a12,12,0,1,1,12,12A12,12,0,0,1,116,120ZM84,132a12,12,0,1,0-12-12A12,12,0,0,0,84,132Zm88,0a12,12,0,1,0-12-12A12,12,0,0,0,172,132Zm60-76V184a16,16,0,0,1-16,16H155.57l-13.68,23.94a16,16,0,0,1-27.78,0L100.43,200H40a16,16,0,0,1-16-16V56A16,16,0,0,1,40,40H216A16,16,0,0,1,232,56Zm-16,0H40V184h65.07a8,8,0,0,1,7,4l16,28,16-28a8,8,0,0,1,7-4H216Z"></path>
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
