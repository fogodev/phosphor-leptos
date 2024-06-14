//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "uncategorized"))]
#[component]
pub fn FolderSimpleDashed(
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
                <path d="M24,80V64A16,16,0,0,1,40,48H93.33a16.12,16.12,0,0,1,9.6,3.2L132.8,73.6A8,8,0,0,1,128,88H32A8,8,0,0,1,24,80ZM88,200H40v-8a8,8,0,0,0-16,0v8.62A15.4,15.4,0,0,0,39.38,216H88a8,8,0,0,0,0-16Zm72,0H128a8,8,0,0,0,0,16h32a8,8,0,0,0,0-16Zm64-56a8,8,0,0,0-8,8v48H200a8,8,0,0,0,0,16h16.89A15.13,15.13,0,0,0,232,200.89V152A8,8,0,0,0,224,144Zm-8-72H168a8,8,0,0,0,0,16h48v24a8,8,0,0,0,16,0V88A16,16,0,0,0,216,72ZM32,160a8,8,0,0,0,8-8V120a8,8,0,0,0-16,0v32A8,8,0,0,0,32,160Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M224,88V200.89a7.11,7.11,0,0,1-7.11,7.11H40a8,8,0,0,1-8-8V64a8,8,0,0,1,8-8H93.33a8,8,0,0,1,4.8,1.6l27.74,20.8a8,8,0,0,0,4.8,1.6H216A8,8,0,0,1,224,88Z"
        opacity="0.2"
    ></path>
    <path d="M24,80V64A16,16,0,0,1,40,48H93.33a16.12,16.12,0,0,1,9.6,3.2L132.8,73.6a8,8,0,1,1-9.6,12.8L93.33,64H40V80a8,8,0,0,1-16,0ZM88,200H40v-8a8,8,0,0,0-16,0v8.62A15.4,15.4,0,0,0,39.38,216H88a8,8,0,0,0,0-16Zm72,0H128a8,8,0,0,0,0,16h32a8,8,0,0,0,0-16Zm64-56a8,8,0,0,0-8,8v48H200a8,8,0,0,0,0,16h16.89A15.13,15.13,0,0,0,232,200.89V152A8,8,0,0,0,224,144Zm-8-72H168a8,8,0,0,0,0,16h48v24a8,8,0,0,0,16,0V88A16,16,0,0,0,216,72ZM32,160a8,8,0,0,0,8-8V120a8,8,0,0,0-16,0v32A8,8,0,0,0,32,160Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M125.6,83.2,95.73,60.8a4,4,0,0,0-2.4-.8H40a4,4,0,0,0-4,4V80a4,4,0,0,1-8,0V64A12,12,0,0,1,40,52H93.33a12.05,12.05,0,0,1,7.2,2.4L130.4,76.8a4,4,0,1,1-4.8,6.4ZM88,204H39.38A3.39,3.39,0,0,1,36,200.62V192a4,4,0,0,0-8,0v8.62A11.4,11.4,0,0,0,39.38,212H88a4,4,0,0,0,0-8Zm72,0H128a4,4,0,0,0,0,8h32a4,4,0,0,0,0-8Zm64-56a4,4,0,0,0-4,4v48.89a3.12,3.12,0,0,1-3.11,3.11H200a4,4,0,0,0,0,8h16.89A11.12,11.12,0,0,0,228,200.89V152A4,4,0,0,0,224,148Zm-8-72H168a4,4,0,0,0,0,8h48a4,4,0,0,1,4,4v24a4,4,0,0,0,8,0V88A12,12,0,0,0,216,76ZM32,156a4,4,0,0,0,4-4V120a4,4,0,0,0-8,0v32A4,4,0,0,0,32,156Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M20,80V64A20,20,0,0,1,40,44H93.33a20.12,20.12,0,0,1,12,4L135.2,70.4a12,12,0,1,1-14.4,19.2L92,68H44V80a12,12,0,0,1-24,0ZM88,196H44v-4a12,12,0,0,0-24,0v8.62A19.41,19.41,0,0,0,39.38,220H88a12,12,0,0,0,0-24Zm72,0H128a12,12,0,0,0,0,24h32a12,12,0,0,0,0-24Zm64-56a12,12,0,0,0-12,12v44H200a12,12,0,0,0,0,24h16.89A19.13,19.13,0,0,0,236,200.89V152A12,12,0,0,0,224,140Zm-8-72H168a12,12,0,0,0,0,24h44v20a12,12,0,0,0,24,0V88A20,20,0,0,0,216,68ZM32,164a12,12,0,0,0,12-12V120a12,12,0,0,0-24,0v32A12,12,0,0,0,32,164Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M124.4,84.8,94.53,62.4a2,2,0,0,0-1.2-.4H40a2,2,0,0,0-2,2V80a6,6,0,0,1-12,0V64A14,14,0,0,1,40,50H93.33a14,14,0,0,1,8.4,2.8L131.6,75.2a6,6,0,0,1-7.2,9.6ZM88,202H39.38A1.4,1.4,0,0,1,38,200.62V192a6,6,0,0,0-12,0v8.62A13.39,13.39,0,0,0,39.38,214H88a6,6,0,0,0,0-12Zm72,0H128a6,6,0,0,0,0,12h32a6,6,0,0,0,0-12Zm64-56a6,6,0,0,0-6,6v48.89a1.11,1.11,0,0,1-1.11,1.11H200a6,6,0,0,0,0,12h16.89A13.12,13.12,0,0,0,230,200.89V152A6,6,0,0,0,224,146Zm-8-72H168a6,6,0,0,0,0,12h48a2,2,0,0,1,2,2v24a6,6,0,0,0,12,0V88A14,14,0,0,0,216,74ZM32,158a6,6,0,0,0,6-6V120a6,6,0,0,0-12,0v32A6,6,0,0,0,32,158Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M24,80V64A16,16,0,0,1,40,48H93.33a16.12,16.12,0,0,1,9.6,3.2L132.8,73.6a8,8,0,1,1-9.6,12.8L93.33,64H40V80a8,8,0,0,1-16,0ZM88,200H40v-8a8,8,0,0,0-16,0v8.62A15.4,15.4,0,0,0,39.38,216H88a8,8,0,0,0,0-16Zm72,0H128a8,8,0,0,0,0,16h32a8,8,0,0,0,0-16Zm64-56a8,8,0,0,0-8,8v48H200a8,8,0,0,0,0,16h16.89A15.13,15.13,0,0,0,232,200.89V152A8,8,0,0,0,224,144Zm-8-72H168a8,8,0,0,0,0,16h48v24a8,8,0,0,0,16,0V88A16,16,0,0,0,216,72ZM32,160a8,8,0,0,0,8-8V120a8,8,0,0,0-16,0v32A8,8,0,0,0,32,160Z"></path>
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
