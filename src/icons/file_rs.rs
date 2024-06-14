//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "office", feature = "editor", feature = "development"))]
#[component]
pub fn FileRs(
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
                <path d="M213.66,82.34l-56-56A8,8,0,0,0,152,24H56A16,16,0,0,0,40,40v76a4,4,0,0,0,4,4H172a4,4,0,0,1,4,4V228a4,4,0,0,0,4,4h20a16,16,0,0,0,16-16V88A8,8,0,0,0,213.66,82.34ZM152,88V44l44,44ZM63.42,144H48a8,8,0,0,0-8,8v55.73A8.17,8.17,0,0,0,47.47,216,8,8,0,0,0,56,208v-8h8a28.48,28.48,0,0,0,5.73-.59L77.09,212a8,8,0,0,0,11.81,2.3,8.14,8.14,0,0,0,1.91-10.54l-7-12A27.92,27.92,0,0,0,92,171.36C91.65,156.05,78.74,144,63.42,144Zm.23,40H56V160h8a12,12,0,0,1,12,13.16A12.25,12.25,0,0,1,63.65,184Zm92.16,12.31a20.82,20.82,0,0,1-9.19,15.23C141.43,215,135,216,129.13,216A61.14,61.14,0,0,1,114,214a8,8,0,1,1,4.3-15.41c4.38,1.2,14.95,2.7,19.55-.36.88-.59,1.83-1.52,2.14-3.93.35-2.67-.71-4.1-12.78-7.59-9.35-2.7-25-7.23-23-23.11a20.56,20.56,0,0,1,9-14.95c11.84-8,30.71-3.31,32.83-2.76a8,8,0,0,1-4.07,15.48c-4.49-1.17-15.23-2.56-19.83.56a4.54,4.54,0,0,0-2,3.67c-.12.9-.14,1.09,1.11,1.9,2.31,1.49,6.45,2.68,10.45,3.84C141.49,174.17,158.05,179,155.81,196.31Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M208,88H152V32Z" opacity="0.2"></path>
    <path d="M213.66,82.34l-56-56A8,8,0,0,0,152,24H56A16,16,0,0,0,40,40v72a8,8,0,0,0,16,0V40h88V88a8,8,0,0,0,8,8h48V216H184a8,8,0,0,0,0,16h16a16,16,0,0,0,16-16V88A8,8,0,0,0,213.66,82.34ZM160,51.31,188.69,80H160ZM64,144H48a8,8,0,0,0-8,8v56a8,8,0,0,0,16,0v-8h8a28.48,28.48,0,0,0,5.73-.59L77.09,212A8,8,0,0,0,90.91,204L83.8,191.78A28,28,0,0,0,64,144Zm-8,40V160h8a12,12,0,0,1,0,24Zm99.81,12.31a20.82,20.82,0,0,1-9.19,15.23C141.43,215,135,216,129.13,216A61.14,61.14,0,0,1,114,214a8,8,0,1,1,4.3-15.41c4.38,1.2,14.95,2.7,19.55-.36.88-.59,1.83-1.52,2.14-3.93.35-2.67-.71-4.1-12.78-7.59-9.35-2.7-25-7.23-23-23.11a20.56,20.56,0,0,1,9-14.95c11.84-8,30.71-3.31,32.83-2.76a8,8,0,0,1-4.07,15.48c-4.49-1.17-15.23-2.56-19.83.56a4.54,4.54,0,0,0-2,3.67c-.12.9-.14,1.09,1.11,1.9,2.31,1.49,6.45,2.68,10.45,3.84C141.49,174.17,158.05,179,155.81,196.31Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M210.83,85.17l-56-56A4,4,0,0,0,152,28H56A12,12,0,0,0,44,40v72a4,4,0,0,0,8,0V40a4,4,0,0,1,4-4h92V88a4,4,0,0,0,4,4h52V216a4,4,0,0,1-4,4H184a4,4,0,0,0,0,8h16a12,12,0,0,0,12-12V88A4,4,0,0,0,210.83,85.17ZM156,41.65,198.34,84H156ZM88,172a24,24,0,0,0-24-24H48a4,4,0,0,0-4,4v56a4,4,0,0,0,8,0V196H64a23.85,23.85,0,0,0,7.64-1.25L80.54,210a4,4,0,1,0,6.92-4l-8.77-15A24,24,0,0,0,88,172ZM64,188H52V156H64a16,16,0,0,1,0,32Zm87.84,7.8a17,17,0,0,1-7.43,12.41c-4.39,2.91-10,3.77-15.22,3.77A57.89,57.89,0,0,1,115,210.11a4,4,0,0,1,2.15-7.7c4.22,1.17,16.56,3.29,22.83-.88a8.94,8.94,0,0,0,3.91-6.75c.83-6.45-4.38-8.69-15.64-11.94-9.68-2.8-21.72-6.28-20.14-18.77a16.66,16.66,0,0,1,7.22-12.13c4.56-3.07,11-4.36,19.1-3.82a61.33,61.33,0,0,1,10.48,1.61,4,4,0,0,1-2.05,7.74c-4.29-1.13-16.81-3.12-23.06,1.11a8.51,8.51,0,0,0-3.75,6.49c-.66,5.17,3.89,7,14.42,10.08C140.26,178,153.64,181.84,151.84,195.8Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M216.49,79.51l-56-56A12,12,0,0,0,152,20H56A20,20,0,0,0,36,40v68a12,12,0,0,0,24,0V44h76V92a12,12,0,0,0,12,12h48V212h-8a12,12,0,0,0,0,24h12a20,20,0,0,0,20-20V88A12,12,0,0,0,216.49,79.51ZM160,57l23,23H160ZM64,140H48a12,12,0,0,0-12,12v56a12,12,0,0,0,24,0v-4h4a33.9,33.9,0,0,0,3.65-.21l6,10.26A12,12,0,0,0,94.37,202l-5.64-9.66A32,32,0,0,0,64,140Zm-4,24h4a8,8,0,0,1,0,16H60Zm103.78,32.82a24.75,24.75,0,0,1-10.95,18.06c-6,4-13.27,5.15-19.73,5.15a63.75,63.75,0,0,1-16.23-2.21,12,12,0,0,1,6.46-23.12c6.81,1.86,15,1.61,16.39.06a2.48,2.48,0,0,0,.21-.71c-1.94-1.23-6.83-2.64-9.88-3.52-5.39-1.56-11-3.18-15.75-6.27-7.62-4.92-11.21-12.45-10.11-21.2a24.45,24.45,0,0,1,10.69-17.75c6.06-4.09,14.17-5.83,24.1-5.18A68.53,68.53,0,0,1,151,142a12,12,0,0,1-6.1,23.21c-6.36-1.63-13.62-1.51-16.07-.33a79.5,79.5,0,0,0,7.91,2.59c5.48,1.58,11.68,3.37,16.8,6.82C161.33,179.55,165,187.55,163.78,196.82Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M212.24,83.76l-56-56A6,6,0,0,0,152,26H56A14,14,0,0,0,42,40v72a6,6,0,0,0,12,0V40a2,2,0,0,1,2-2h90V88a6,6,0,0,0,6,6h50V216a2,2,0,0,1-2,2H184a6,6,0,0,0,0,12h16a14,14,0,0,0,14-14V88A6,6,0,0,0,212.24,83.76ZM158,46.48,193.52,82H158ZM90,172a26,26,0,0,0-26-26H48a6,6,0,0,0-6,6v56a6,6,0,0,0,12,0V198H64a26,26,0,0,0,6.71-.88L78.82,211A6,6,0,0,0,84,214a5.91,5.91,0,0,0,3-.82,6,6,0,0,0,2.16-8.2l-7.91-13.57A25.93,25.93,0,0,0,90,172ZM54,186V158H64a14,14,0,0,1,0,28Zm99.83,10.06a18.89,18.89,0,0,1-8.31,13.81c-4.82,3.19-10.87,4.14-16.36,4.14a58.89,58.89,0,0,1-14.68-2,6,6,0,0,1,3.23-11.56c3.71,1,15.58,3.11,21.19-.62a6.85,6.85,0,0,0,3-5.34c.58-4.43-2.08-6.26-14.2-9.76-9.31-2.69-23.37-6.75-21.57-20.94a18.61,18.61,0,0,1,8.08-13.54c11.11-7.49,29.18-3,31.21-2.48a6,6,0,0,1-3.06,11.6c-3.78-1-15.85-3-21.45.84a6.59,6.59,0,0,0-2.88,5.08c-.41,3.22,2.14,4.78,13,7.91C140.92,176.09,155.84,180.4,153.83,196.06Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M213.66,82.34l-56-56A8,8,0,0,0,152,24H56A16,16,0,0,0,40,40v72a8,8,0,0,0,16,0V40h88V88a8,8,0,0,0,8,8h48V216H184a8,8,0,0,0,0,16h16a16,16,0,0,0,16-16V88A8,8,0,0,0,213.66,82.34ZM160,51.31,188.69,80H160ZM64,144H48a8,8,0,0,0-8,8v56a8,8,0,0,0,16,0v-8h8a28.48,28.48,0,0,0,5.73-.59L77.09,212A8,8,0,0,0,90.91,204L83.8,191.78A28,28,0,0,0,64,144Zm-8,40V160h8a12,12,0,0,1,0,24Zm99.81,12.31a20.82,20.82,0,0,1-9.19,15.23C141.43,215,135,216,129.13,216a61.34,61.34,0,0,1-15.19-2,8,8,0,0,1,4.31-15.41c4.38,1.2,14.95,2.7,19.55-.36.88-.59,1.83-1.52,2.14-3.93.34-2.67-.71-4.1-12.78-7.59-9.35-2.7-25-7.23-23-23.11a20.56,20.56,0,0,1,9-14.95c11.84-8,30.71-3.31,32.83-2.76a8,8,0,0,1-4.07,15.48c-4.49-1.17-15.23-2.56-19.83.56a4.54,4.54,0,0,0-2,3.67c-.12.9-.14,1.09,1.11,1.9,2.31,1.49,6.45,2.68,10.45,3.84C141.49,174.17,158.05,179,155.81,196.31Z"></path>
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
