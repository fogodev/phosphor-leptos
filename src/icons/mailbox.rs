//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "communication", feature = "objects", feature = "map"))]
#[component]
pub fn Mailbox(
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
                <path d="M104,152a8,8,0,0,1-8,8H56a8,8,0,0,1,0-16H96A8,8,0,0,1,104,152ZM168,32h24a8,8,0,0,0,0-16H160a8,8,0,0,0-8,8V56h16Zm72,84v60a16,16,0,0,1-16,16H136v32a8,8,0,0,1-16,0V192H32a16,16,0,0,1-16-16V116A60.07,60.07,0,0,1,76,56h76v88a8,8,0,0,0,16,0V56h12A60.07,60.07,0,0,1,240,116Zm-120,0a44,44,0,0,0-88,0v60h88Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M232,116v60a8,8,0,0,1-8,8H128V116A52,52,0,0,0,76,64H180A52,52,0,0,1,232,116Z"
        opacity="0.2"
    ></path>
    <path d="M104,152a8,8,0,0,1-8,8H56a8,8,0,0,1,0-16H96A8,8,0,0,1,104,152Zm136-36v60a16,16,0,0,1-16,16H136v32a8,8,0,0,1-16,0V192H32a16,16,0,0,1-16-16V116A60.07,60.07,0,0,1,76,56h76V24a8,8,0,0,1,8-8h32a8,8,0,0,1,0,16H168V56h12A60.07,60.07,0,0,1,240,116ZM120,176V116a44,44,0,0,0-88,0v60Zm104-60a44.05,44.05,0,0,0-44-44H168v72a8,8,0,0,1-16,0V72H116.75A59.86,59.86,0,0,1,136,116v60h88Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M100,152a4,4,0,0,1-4,4H56a4,4,0,0,1,0-8H96A4,4,0,0,1,100,152Zm136-36v60a12,12,0,0,1-12,12H132v36a4,4,0,0,1-8,0V188H32a12,12,0,0,1-12-12V116A56.06,56.06,0,0,1,76,60h80V24a4,4,0,0,1,4-4h32a4,4,0,0,1,0,8H164V60h16A56.06,56.06,0,0,1,236,116ZM124,180V116a48,48,0,0,0-96,0v60a4,4,0,0,0,4,4Zm104-64a48.05,48.05,0,0,0-48-48H164v76a4,4,0,0,1-8,0V68H104.82A56,56,0,0,1,132,116v64h92a4,4,0,0,0,4-4Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M100,148a12,12,0,0,1-12,12H64a12,12,0,0,1,0-24H88A12,12,0,0,1,100,148Zm144-32v60a20,20,0,0,1-20,20H140v28a12,12,0,0,1-24,0V196H32a20,20,0,0,1-20-20V116A64.07,64.07,0,0,1,76,52h80V24a12,12,0,0,1,12-12h32a12,12,0,0,1,0,24H180V52A64.07,64.07,0,0,1,244,116ZM116,172V116a40,40,0,0,0-80,0v56Zm104-56a40,40,0,0,0-40-40v68a12,12,0,0,1-24,0V76H125.93A63.7,63.7,0,0,1,140,116v56h80Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M102,152a6,6,0,0,1-6,6H56a6,6,0,0,1,0-12H96A6,6,0,0,1,102,152Zm136-36v60a14,14,0,0,1-14,14H134v34a6,6,0,0,1-12,0V190H32a14,14,0,0,1-14-14V116A58.07,58.07,0,0,1,76,58h78V24a6,6,0,0,1,6-6h32a6,6,0,0,1,0,12H166V58h14A58.07,58.07,0,0,1,238,116ZM122,178V116a46,46,0,0,0-92,0v60a2,2,0,0,0,2,2Zm104-62a46.06,46.06,0,0,0-46-46H166v74a6,6,0,0,1-12,0V70H111.29A57.93,57.93,0,0,1,134,116v62h90a2,2,0,0,0,2-2Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M104,152a8,8,0,0,1-8,8H56a8,8,0,0,1,0-16H96A8,8,0,0,1,104,152Zm136-36v60a16,16,0,0,1-16,16H136v32a8,8,0,0,1-16,0V192H32a16,16,0,0,1-16-16V116A60.07,60.07,0,0,1,76,56h76V24a8,8,0,0,1,8-8h32a8,8,0,0,1,0,16H168V56h12A60.07,60.07,0,0,1,240,116ZM120,176V116a44,44,0,0,0-88,0v60Zm104-60a44.05,44.05,0,0,0-44-44H168v72a8,8,0,0,1-16,0V72H116.75A59.86,59.86,0,0,1,136,116v60h88Z"></path>
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
