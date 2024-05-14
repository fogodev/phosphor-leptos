//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "commerce", feature = "map"))]
#[component]
pub fn Onigiri(
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
                <path d="M231.53,146.57,175.68,50.66l-.11-.19a56,56,0,0,0-95.14,0l-.11.19L24.47,146.57A56,56,0,0,0,72.09,232H183.91a56,56,0,0,0,47.62-85.43Zm-12.68,48.88A39.49,39.49,0,0,1,183.91,216H176V168a16,16,0,0,0-16-16H96a16,16,0,0,0-16,16v48H72.09a40,40,0,0,1-34-61.09,2,2,0,0,0,.11-.2l55.85-95.9a40,40,0,0,1,67.84,0l55.85,95.9a2,2,0,0,0,.11.2A39.5,39.5,0,0,1,218.85,195.45Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M168,168v56H88V168a8,8,0,0,1,8-8h64A8,8,0,0,1,168,168Z" opacity="0.2"></path>
    <path d="M231.53,146.57,175.68,50.66l-.11-.19a56,56,0,0,0-95.14,0l-.11.19L24.47,146.57A56,56,0,0,0,72.09,232H183.91a56,56,0,0,0,47.62-85.43ZM160,216H96V168h64Zm58.86-20.55A39.49,39.49,0,0,1,183.91,216H176V168a16,16,0,0,0-16-16H96a16,16,0,0,0-16,16v48H72.09a40,40,0,0,1-34-61.09,2,2,0,0,0,.11-.2l55.85-95.9a40,40,0,0,1,67.84,0l55.85,95.9a2,2,0,0,0,.11.2A39.5,39.5,0,0,1,218.85,195.45Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M228.11,148.63l-55.88-96-.06-.09a52,52,0,0,0-88.34,0l-.06.09-55.88,96A52,52,0,0,0,72.09,228H183.91a52,52,0,0,0,44.2-79.37ZM164,220H92V168a4,4,0,0,1,4-4h64a4,4,0,0,1,4,4Zm58.36-22.61A43.43,43.43,0,0,1,183.91,220H172V168a12,12,0,0,0-12-12H96a12,12,0,0,0-12,12v52H72.09a44,44,0,0,1-37.37-67.2.31.31,0,0,1,.05-.1L90.65,56.75a44,44,0,0,1,74.7,0l55.88,95.95a.31.31,0,0,1,.05.1A43.43,43.43,0,0,1,222.35,197.39Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M235,144.51,179.13,48.65a2.75,2.75,0,0,0-.17-.29A60,60,0,0,0,77,48.36a2.75,2.75,0,0,0-.17.29L21,144.51A60,60,0,0,0,72.1,236H183.9A60,60,0,0,0,235,144.51ZM156,212H100V172h56Zm59.36-18.5A35.55,35.55,0,0,1,183.9,212H180V168a20,20,0,0,0-20-20H96a20,20,0,0,0-20,20v44H72.1a36,36,0,0,1-30.58-55l.17-.28L97.51,60.88a36,36,0,0,1,61,0l55.82,95.85.17.28A35.58,35.58,0,0,1,215.35,193.5Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M229.82,147.6,174,51.67l-.09-.15a54,54,0,0,0-91.74,0l-.09.15L26.18,147.6A54,54,0,0,0,72.09,230H183.91a54,54,0,0,0,45.91-82.4ZM162,218H94V168a2,2,0,0,1,2-2h64a2,2,0,0,1,2,2Zm58.61-21.58A41.47,41.47,0,0,1,183.91,218H174V168a14,14,0,0,0-14-14H96a14,14,0,0,0-14,14v50H72.09a42,42,0,0,1-35.67-64.15l.08-.14L92.37,57.78a42,42,0,0,1,71.26,0l55.87,95.93.08.14A41.48,41.48,0,0,1,220.6,196.42Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M231.53,146.57,175.68,50.66l-.11-.19a56,56,0,0,0-95.14,0l-.11.19L24.47,146.57A56,56,0,0,0,72.09,232H183.91a56,56,0,0,0,47.62-85.43ZM160,216H96V168h64Zm58.86-20.55A39.49,39.49,0,0,1,183.91,216H176V168a16,16,0,0,0-16-16H96a16,16,0,0,0-16,16v48H72.09a40,40,0,0,1-34-61.09,2,2,0,0,0,.11-.2l55.85-95.9a40,40,0,0,1,67.84,0l55.85,95.9a2,2,0,0,0,.11.2A39.5,39.5,0,0,1,218.85,195.45Z"></path>
}.into_view()
        }
    });

    let transform = move || if mirrored.get() { "scale(-1, 1)" } else { "" };
    let height = size.clone();

    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width=move || size.get()
            height=move || height.get()
            fill=color
            transform=transform
            viewBox="0 0 256 256"
            id=move || id.get().unwrap_or(TextProp::from(""))
            class=move || class.get().unwrap_or(TextProp::from(""))
        >
            {body}
        </svg>
    }
}