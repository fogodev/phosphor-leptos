//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn HandGrabbing(
    #[prop(into, default = MaybeSignal::Static(IconWeight::Regular))] weight: MaybeSignal<
        IconWeight,
    >,
    #[prop(into, default = TextProp::from("1em"))] size: TextProp,
    #[prop(into, default = TextProp::from("currentColor"))] color: TextProp,
    #[prop(into, default = MaybeSignal::Static(false))] mirrored: MaybeSignal<bool>,
) -> impl IntoView {
    let body = move || {
        match weight.get() {
            IconWeight::Bold => view! {
                <path d="M188,76a31.85,31.85,0,0,0-11.21,2,32,32,0,0,0-48.79-11A32,32,0,0,0,76,92v16H68a32,32,0,0,0-32,32v12a92,92,0,0,0,184,0V108A32,32,0,0,0,188,76Zm8,76a68,68,0,0,1-136,0V140a8,8,0,0,1,8-8h8v20a12,12,0,0,0,24,0V92a8,8,0,0,1,16,0v28a12,12,0,0,0,24,0V92a8,8,0,0,1,16,0v28a12,12,0,0,0,24,0V108a8,8,0,0,1,16,0Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M208,108v44a80,80,0,0,1-160,0V140a20,20,0,0,1,20-20H88V92a20,20,0,0,1,40,0,20,20,0,0,1,40,0v16a20,20,0,0,1,40,0Z"
        opacity="0.2"
    ></path>
    <path d="M188,80a27.79,27.79,0,0,0-13.36,3.4,28,28,0,0,0-46.64-11A28,28,0,0,0,80,92v20H68a28,28,0,0,0-28,28v12a88,88,0,0,0,176,0V108A28,28,0,0,0,188,80Zm12,72a72,72,0,0,1-144,0V140a12,12,0,0,1,12-12H80v24a8,8,0,0,0,16,0V92a12,12,0,0,1,24,0v28a8,8,0,0,0,16,0V92a12,12,0,0,1,24,0v28a8,8,0,0,0,16,0V108a12,12,0,0,1,24,0Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M216,104v48a88,88,0,0,1-176,0V136a16,16,0,0,1,32,0v8a8,8,0,0,0,16,0V88a16,16,0,0,1,32,0v16a8,8,0,0,0,16,0V88a16,16,0,0,1,32,0v16a8,8,0,0,0,16,0,16,16,0,0,1,32,0Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M188,82a25.85,25.85,0,0,0-14.59,4.49A26,26,0,0,0,128,75.41,26,26,0,0,0,82,92v22H68a26,26,0,0,0-26,26v12a86,86,0,0,0,172,0V108A26,26,0,0,0,188,82Zm14,70a74,74,0,0,1-148,0V140a14,14,0,0,1,14-14H82v26a6,6,0,0,0,12,0V92a14,14,0,0,1,28,0v28a6,6,0,0,0,12,0V92a14,14,0,0,1,28,0v28a6,6,0,0,0,12,0V108a14,14,0,0,1,28,0Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M188,80a27.79,27.79,0,0,0-13.36,3.4,28,28,0,0,0-46.64-11A28,28,0,0,0,80,92v20H68a28,28,0,0,0-28,28v12a88,88,0,0,0,176,0V108A28,28,0,0,0,188,80Zm12,72a72,72,0,0,1-144,0V140a12,12,0,0,1,12-12H80v24a8,8,0,0,0,16,0V92a12,12,0,0,1,24,0v28a8,8,0,0,0,16,0V92a12,12,0,0,1,24,0v28a8,8,0,0,0,16,0V108a12,12,0,0,1,24,0Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M188,84a23.87,23.87,0,0,0-16.07,6.2A24,24,0,0,0,128,78.75,24,24,0,0,0,84,92v24H68a24,24,0,0,0-24,24v12a84,84,0,0,0,168,0V108A24,24,0,0,0,188,84Zm16,68a76,76,0,0,1-152,0V140a16,16,0,0,1,16-16H84v28a4,4,0,0,0,8,0V92a16,16,0,0,1,32,0v28a4,4,0,0,0,8,0V92a16,16,0,0,1,32,0v28a4,4,0,0,0,8,0V108a16,16,0,0,1,32,0Z"></path>
}.into_view()
        }
    };

    let transform = move || if mirrored.get() { "scale(-1, 1)" } else { "" };

    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width=size.get()
            height=size.get()
            fill=color
            transform=transform
            viewBox="0 0 256 256"
        >
            {body}
        </svg>
    }
}
