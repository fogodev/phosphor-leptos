//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "games"))]
#[component]
pub fn FinnTheHuman(
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
                <path d="M176,148a12,12,0,1,1-12-12A12,12,0,0,1,176,148ZM92,136a12,12,0,1,0,12,12A12,12,0,0,0,92,136ZM240,72v80a72.08,72.08,0,0,1-72,72H88a72.08,72.08,0,0,1-72-72V72a32,32,0,0,1,63-8h98a32,32,0,0,1,63,8Zm-40,72a32,32,0,0,0-32-32H88a32,32,0,0,0-32,32v8a32,32,0,0,0,32,32h80a32,32,0,0,0,32-32Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M208,48a24,24,0,0,0-24,24H72a24,24,0,0,0-48,0v80a64,64,0,0,0,64,64h80a64,64,0,0,0,64-64V72A24,24,0,0,0,208,48Zm-8,104a32,32,0,0,1-32,32H88a32,32,0,0,1-32-32v-8a32,32,0,0,1,32-32h80a32,32,0,0,1,32,32Z"
        opacity="0.2"
    ></path>
    <path d="M168,104H88a40,40,0,0,0-40,40v8a40,40,0,0,0,40,40h80a40,40,0,0,0,40-40v-8A40,40,0,0,0,168,104Zm24,48a24,24,0,0,1-24,24H88a24,24,0,0,1-24-24v-8a24,24,0,0,1,24-24h80a24,24,0,0,1,24,24ZM208,40a32.06,32.06,0,0,0-31,24H79a32,32,0,0,0-63,8v80a72.08,72.08,0,0,0,72,72h80a72.08,72.08,0,0,0,72-72V72A32,32,0,0,0,208,40Zm16,112a56.06,56.06,0,0,1-56,56H88a56.06,56.06,0,0,1-56-56V72a16,16,0,0,1,32,0,8,8,0,0,0,8,8H184a8,8,0,0,0,8-8,16,16,0,0,1,32,0Zm-120-4a12,12,0,1,1-12-12A12,12,0,0,1,104,148Zm72,0a12,12,0,1,1-12-12A12,12,0,0,1,176,148Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M168,108H88a36,36,0,0,0-36,36v8a36,36,0,0,0,36,36h80a36,36,0,0,0,36-36v-8A36,36,0,0,0,168,108Zm28,44a28,28,0,0,1-28,28H88a28,28,0,0,1-28-28v-8a28,28,0,0,1,28-28h80a28,28,0,0,1,28,28ZM208,44a28,28,0,0,0-27.71,24H75.71A28,28,0,0,0,20,72v80a68.07,68.07,0,0,0,68,68h80a68.07,68.07,0,0,0,68-68V72A28,28,0,0,0,208,44Zm20,108a60.07,60.07,0,0,1-60,60H88a60.07,60.07,0,0,1-60-60V72a20,20,0,0,1,40,0,4,4,0,0,0,4,4H184a4,4,0,0,0,4-4,20,20,0,0,1,40,0Zm-128-4a8,8,0,1,1-8-8A8,8,0,0,1,100,148Zm72,0a8,8,0,1,1-8-8A8,8,0,0,1,172,148Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M160,96H96a52,52,0,0,0,0,104h64a52,52,0,0,0,0-104Zm0,80H96a28,28,0,0,1,0-56h64a28,28,0,0,1,0,56Zm-48-28a16,16,0,1,1-16-16A16,16,0,0,1,112,148Zm64,0a16,16,0,1,1-16-16A16,16,0,0,1,176,148ZM212,36a36,36,0,0,0-33.94,24H77.94A36,36,0,0,0,8,72v76a88.1,88.1,0,0,0,88,88h64a88.1,88.1,0,0,0,88-88V72A36,36,0,0,0,212,36Zm12,112a64.07,64.07,0,0,1-64,64H96a64.07,64.07,0,0,1-64-64V72a12,12,0,0,1,24,0A12,12,0,0,0,68,84H188a12,12,0,0,0,12-12,12,12,0,0,1,24,0Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M168,106H88a38,38,0,0,0-38,38v8a38,38,0,0,0,38,38h80a38,38,0,0,0,38-38v-8A38,38,0,0,0,168,106Zm26,46a26,26,0,0,1-26,26H88a26,26,0,0,1-26-26v-8a26,26,0,0,1,26-26h80a26,26,0,0,1,26,26ZM208,42a30.05,30.05,0,0,0-29.4,24H77.4A30,30,0,0,0,18,72v80a70.08,70.08,0,0,0,70,70h80a70.08,70.08,0,0,0,70-70V72A30,30,0,0,0,208,42Zm18,110a58.07,58.07,0,0,1-58,58H88a58.07,58.07,0,0,1-58-58V72a18,18,0,0,1,36,0,6,6,0,0,0,6,6H184a6,6,0,0,0,6-6,18,18,0,0,1,36,0Zm-124-4a10,10,0,1,1-10-10A10,10,0,0,1,102,148Zm72,0a10,10,0,1,1-10-10A10,10,0,0,1,174,148Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M168,104H88a40,40,0,0,0-40,40v8a40,40,0,0,0,40,40h80a40,40,0,0,0,40-40v-8A40,40,0,0,0,168,104Zm24,48a24,24,0,0,1-24,24H88a24,24,0,0,1-24-24v-8a24,24,0,0,1,24-24h80a24,24,0,0,1,24,24ZM208,40a32.06,32.06,0,0,0-31,24H79a32,32,0,0,0-63,8v80a72.08,72.08,0,0,0,72,72h80a72.08,72.08,0,0,0,72-72V72A32,32,0,0,0,208,40Zm16,112a56.06,56.06,0,0,1-56,56H88a56.06,56.06,0,0,1-56-56V72a16,16,0,0,1,32,0,8,8,0,0,0,8,8H184a8,8,0,0,0,8-8,16,16,0,0,1,32,0Zm-120-4a12,12,0,1,1-12-12A12,12,0,0,1,104,148Zm72,0a12,12,0,1,1-12-12A12,12,0,0,1,176,148Z"></path>
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
