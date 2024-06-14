//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "office", feature = "editor", feature = "development"))]
#[component]
pub fn FileCpp(
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
                <path d="M44,120H212a4,4,0,0,0,4-4V88a8,8,0,0,0-2.34-5.66l-56-56A8,8,0,0,0,152,24H56A16,16,0,0,0,40,40v76A4,4,0,0,0,44,120ZM152,44l44,44H152ZM48,180c0,11,7.18,20,16,20a14.07,14.07,0,0,0,10.07-4.51,8.19,8.19,0,0,1,10.88-.9,8,8,0,0,1,.83,11.81A30.06,30.06,0,0,1,64,216c-17.65,0-32-16.15-32-36s14.35-36,32-36a30,30,0,0,1,21.39,9.2,8.24,8.24,0,0,1,.73,11.08,8,8,0,0,1-11.9.38A14.18,14.18,0,0,0,64,160C55.18,160,48,169,48,180Zm108,.53a8.18,8.18,0,0,1-8.25,7.47H136v11.73a8.17,8.17,0,0,1-7.47,8.25,8,8,0,0,1-8.53-8V188H108.27a8.18,8.18,0,0,1-8.25-7.47,8,8,0,0,1,8-8.53h12V160.27a8.17,8.17,0,0,1,7.47-8.25,8,8,0,0,1,8.53,8v12h12A8,8,0,0,1,156,180.53Zm68,0a8.18,8.18,0,0,1-8.25,7.47H204v11.73a8.17,8.17,0,0,1-7.47,8.25,8,8,0,0,1-8.53-8V188H176.27a8.18,8.18,0,0,1-8.25-7.47,8,8,0,0,1,8-8.53h12V160.27a8.17,8.17,0,0,1,7.47-8.25,8,8,0,0,1,8.53,8v12h12A8,8,0,0,1,224,180.53Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M208,88H152V32Z" opacity="0.2"></path>
    <path d="M48,180c0,11,7.18,20,16,20a14.18,14.18,0,0,0,10.22-4.66A8,8,0,0,1,85.78,206.4,30.06,30.06,0,0,1,64,216c-17.65,0-32-16.15-32-36s14.35-36,32-36a30.06,30.06,0,0,1,21.78,9.6,8,8,0,0,1-11.56,11.06A14.18,14.18,0,0,0,64,160C55.18,160,48,169,48,180Zm-8-68V40A16,16,0,0,1,56,24h96a8,8,0,0,1,5.66,2.34l56,56A8,8,0,0,1,216,88v24a8,8,0,0,1-16,0V96H152a8,8,0,0,1-8-8V40H56v72a8,8,0,0,1-16,0ZM160,80h28.69L160,51.31Zm-12,92H136V160a8,8,0,0,0-16,0v12H108a8,8,0,0,0,0,16h12v12a8,8,0,0,0,16,0V188h12a8,8,0,0,0,0-16Zm68,0H204V160a8,8,0,0,0-16,0v12H176a8,8,0,0,0,0,16h12v12a8,8,0,0,0,16,0V188h12a8,8,0,0,0,0-16Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M44,180c0,13.23,9,24,20,24a18.15,18.15,0,0,0,13.11-5.9,4,4,0,1,1,5.78,5.54A26.11,26.11,0,0,1,64,212c-15.44,0-28-14.35-28-32s12.56-32,28-32a26.11,26.11,0,0,1,18.89,8.36,4,4,0,1,1-5.78,5.54A18.15,18.15,0,0,0,64,156C53,156,44,166.77,44,180ZM204,92H152a4,4,0,0,1-4-4V36H56a4,4,0,0,0-4,4v72a4,4,0,0,1-8,0V40A12,12,0,0,1,56,28h96a4,4,0,0,1,2.83,1.17l56,56A4,4,0,0,1,212,88v24a4,4,0,0,1-8,0Zm-48-8h42.34L156,41.65Zm-8,92H132V160a4,4,0,0,0-8,0v16H108a4,4,0,0,0,0,8h16v16a4,4,0,0,0,8,0V184h16a4,4,0,0,0,0-8Zm68,0H200V160a4,4,0,0,0-8,0v16H176a4,4,0,0,0,0,8h16v16a4,4,0,0,0,8,0V184h16a4,4,0,0,0,0-8Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M48,180c0,8.67,5.5,16,12,16a10.23,10.23,0,0,0,7.33-3.43,12,12,0,1,1,17.34,16.6A34,34,0,0,1,60,220c-19.85,0-36-17.94-36-40s16.15-40,36-40a34,34,0,0,1,24.67,10.83,12,12,0,1,1-17.34,16.6A10.27,10.27,0,0,0,60,164C53.5,164,48,171.33,48,180ZM36,108V40A20,20,0,0,1,56,20h96a12,12,0,0,1,8.49,3.51l56,56A12,12,0,0,1,220,88v20a12,12,0,0,1-24,0v-4H148a12,12,0,0,1-12-12V44H60v64a12,12,0,0,1-24,0ZM160,57V80h23ZM148,168h-8v-8a12,12,0,0,0-24,0v8h-8a12,12,0,0,0,0,24h8v8a12,12,0,0,0,24,0v-8h8a12,12,0,0,0,0-24Zm72,0h-8v-8a12,12,0,0,0-24,0v8h-8a12,12,0,0,0,0,24h8v8a12,12,0,0,0,24,0v-8h8a12,12,0,0,0,0-24Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M46,180c0,12.13,8.07,22,18,22a16.23,16.23,0,0,0,11.67-5.28,6,6,0,0,1,8.66,8.3A28.06,28.06,0,0,1,64,214c-16.54,0-30-15.25-30-34s13.46-34,30-34a28.06,28.06,0,0,1,20.33,9,6,6,0,0,1-8.66,8.3A16.23,16.23,0,0,0,64,158C54.07,158,46,167.87,46,180Zm-4-68V40A14,14,0,0,1,56,26h96a6,6,0,0,1,4.25,1.76l56,56A6,6,0,0,1,214,88v24a6,6,0,0,1-12,0V94H152a6,6,0,0,1-6-6V38H56a2,2,0,0,0-2,2v72a6,6,0,0,1-12,0ZM158,82h35.52L158,46.48Zm-10,92H134V160a6,6,0,0,0-12,0v14H108a6,6,0,0,0,0,12h14v14a6,6,0,0,0,12,0V186h14a6,6,0,0,0,0-12Zm68,0H202V160a6,6,0,0,0-12,0v14H176a6,6,0,0,0,0,12h14v14a6,6,0,0,0,12,0V186h14a6,6,0,0,0,0-12Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M48,180c0,11,7.18,20,16,20a14.18,14.18,0,0,0,10.22-4.66A8,8,0,0,1,85.78,206.4,30.06,30.06,0,0,1,64,216c-17.65,0-32-16.15-32-36s14.35-36,32-36a30.06,30.06,0,0,1,21.78,9.6,8,8,0,0,1-11.56,11.06A14.24,14.24,0,0,0,64,160C55.18,160,48,169,48,180Zm-8-68V40A16,16,0,0,1,56,24h96a8,8,0,0,1,5.66,2.34l56,56A8,8,0,0,1,216,88v24a8,8,0,0,1-16,0V96H152a8,8,0,0,1-8-8V40H56v72a8,8,0,0,1-16,0ZM160,80h28.69L160,51.31Zm-12,92H136V160a8,8,0,0,0-16,0v12H108a8,8,0,0,0,0,16h12v12a8,8,0,0,0,16,0V188h12a8,8,0,0,0,0-16Zm68,0H204V160a8,8,0,0,0-16,0v12H176a8,8,0,0,0,0,16h12v12a8,8,0,0,0,16,0V188h12a8,8,0,0,0,0-16Z"></path>
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
