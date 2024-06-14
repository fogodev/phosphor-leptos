//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "commerce", feature = "map", feature = "objects"))]
#[component]
pub fn TeaBag(
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
                <path d="M112,136V72h19.47a16.09,16.09,0,0,1,13.72,7.77L165.72,114a16.06,16.06,0,0,1,2.28,8.24V216a16,16,0,0,1-16,16H56a16,16,0,0,1-16-16V122.22A16.06,16.06,0,0,1,42.28,114L62.81,79.77A16.09,16.09,0,0,1,76.53,72H96v64a8,8,0,0,0,16,0Zm112,24a16,16,0,0,1-16-16V64A56,56,0,0,0,96,64v8h16V64a40,40,0,0,1,80,0v80a32,32,0,0,0,32,32,8,8,0,0,0,0-16Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M160,122.22V216a8,8,0,0,1-8,8H56a8,8,0,0,1-8-8V122.22a8,8,0,0,1,1.14-4.12L69.67,83.88A8,8,0,0,1,76.53,80h54.94a8,8,0,0,1,6.86,3.88l20.53,34.22A8,8,0,0,1,160,122.22Z"
        opacity="0.2"
    ></path>
    <path d="M224,160a16,16,0,0,1-16-16V64A56,56,0,0,0,96,64v8H76.53a16.09,16.09,0,0,0-13.72,7.77L42.28,114A16.06,16.06,0,0,0,40,122.22V216a16,16,0,0,0,16,16h96a16,16,0,0,0,16-16V122.22a16.06,16.06,0,0,0-2.28-8.24L145.19,79.77A16.09,16.09,0,0,0,131.47,72H112V64a40,40,0,0,1,80,0v80a32,32,0,0,0,32,32,8,8,0,0,0,0-16ZM131.47,88,152,122.22V216H56V122.22L76.53,88H96v48a8,8,0,0,0,16,0V88Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M224,164a20,20,0,0,1-20-20V64a52,52,0,0,0-104,0V76H76.53a12.06,12.06,0,0,0-10.29,5.83L45.71,116A12,12,0,0,0,44,122.22V216a12,12,0,0,0,12,12h96a12,12,0,0,0,12-12V122.22a12,12,0,0,0-1.71-6.18L141.76,81.83A12.06,12.06,0,0,0,131.47,76H108V64a44,44,0,0,1,88,0v80a28,28,0,0,0,28,28,4,4,0,0,0,0-8ZM131.47,84a4,4,0,0,1,3.43,1.94l20.53,34.22a4,4,0,0,1,.57,2.06V216a4,4,0,0,1-4,4H56a4,4,0,0,1-4-4V122.22a4,4,0,0,1,.57-2.06L73.1,85.94A4,4,0,0,1,76.53,84H100v52a4,4,0,0,0,8,0V84Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M224,156a12,12,0,0,1-12-12V64A60,60,0,0,0,92,64v4H76.53a20.1,20.1,0,0,0-17.15,9.71L38.85,111.92A20.07,20.07,0,0,0,36,122.22V216a20,20,0,0,0,20,20h96a20,20,0,0,0,20-20V122.22a20,20,0,0,0-2.85-10.29L148.62,77.71A20.1,20.1,0,0,0,131.47,68H116V64a36,36,0,0,1,72,0v80a36,36,0,0,0,36,36,12,12,0,0,0,0-24ZM129.21,92,148,123.32V212H60V123.32L78.79,92H92v44a12,12,0,0,0,24,0V92Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M224,162a18,18,0,0,1-18-18V64A54,54,0,0,0,98,64V74H76.53a14.07,14.07,0,0,0-12,6.8L44,115a14,14,0,0,0-2,7.21V216a14,14,0,0,0,14,14h96a14,14,0,0,0,14-14V122.22a14,14,0,0,0-2-7.21L143.48,80.8a14.07,14.07,0,0,0-12-6.8H110V64a42,42,0,0,1,84,0v80a30,30,0,0,0,30,30,6,6,0,0,0,0-12ZM131.47,86a2,2,0,0,1,1.72,1l20.53,34.22a2,2,0,0,1,.28,1V216a2,2,0,0,1-2,2H56a2,2,0,0,1-2-2V122.22a2,2,0,0,1,.29-1L74.81,87a2,2,0,0,1,1.72-1H98v50a6,6,0,0,0,12,0V86Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M224,160a16,16,0,0,1-16-16V64A56,56,0,0,0,96,64v8H76.53a16.09,16.09,0,0,0-13.72,7.77L42.28,114A16.06,16.06,0,0,0,40,122.22V216a16,16,0,0,0,16,16h96a16,16,0,0,0,16-16V122.22a16.06,16.06,0,0,0-2.28-8.24L145.19,79.77A16.09,16.09,0,0,0,131.47,72H112V64a40,40,0,0,1,80,0v80a32,32,0,0,0,32,32,8,8,0,0,0,0-16ZM131.47,88,152,122.22V216H56V122.22L76.53,88H96v48a8,8,0,0,0,16,0V88Z"></path>
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
