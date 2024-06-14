//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "system", feature = "objects"))]
#[component]
pub fn ShieldSlash(
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
                <path d="M224,56v56c0,25.24-5.85,45.72-14.3,62.14a4,4,0,0,1-6.53.87L86.52,46.69a4,4,0,0,1,3-6.69H208A16,16,0,0,1,224,56ZM53.92,34.62A8,8,0,0,0,40.26,42,16,16,0,0,0,32,56v56c0,52.72,25.52,84.67,46.93,102.19,23.06,18.86,46,25.27,47,25.53a8,8,0,0,0,4.2,0c1-.26,23.91-6.67,47-25.53A131.92,131.92,0,0,0,187.18,205l14.9,16.38a8,8,0,1,0,11.84-10.76Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M216,56v56c0,96-88,120-88,120S40,208,40,112V56a8,8,0,0,1,8-8H208A8,8,0,0,1,216,56Z"
        opacity="0.2"
    ></path>
    <path d="M53.92,34.62A8,8,0,0,0,40.26,42,16,16,0,0,0,32,56v56c0,52.72,25.52,84.67,46.93,102.19,23.06,18.86,46,25.27,47,25.53a8,8,0,0,0,4.2,0c1.36-.37,31.27-8.78,57.09-34.72l14.89,16.38a8,8,0,1,0,11.84-10.76Zm74.07,189a128.48,128.48,0,0,1-38.92-21.81C61.82,179.51,48,149.3,48,112l0-56h3.71L176.41,193.15A129.26,129.26,0,0,1,128,223.62ZM224,56v56c0,20.58-3.89,39.61-11.56,56.59A8,8,0,1,1,197.86,162c6.73-14.89,10.14-31.71,10.14-50V56L98.52,56a8,8,0,1,1,0-16H208A16,16,0,0,1,224,56Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M51,37.31A4,4,0,0,0,45,42.69l1.31,1.44A12,12,0,0,0,36,56v56c0,51.16,24.73,82.12,45.47,99.1,22.4,18.32,44.55,24.5,45.48,24.76a4,4,0,0,0,2.1,0c1.37-.37,32.56-9.14,58.23-36.71L205,218.69a4,4,0,1,0,5.92-5.38Zm77,190.51a132.23,132.23,0,0,1-41.71-23.11C58.23,181.63,44,150.44,44,112V56a4,4,0,0,1,4-4H53.5L181.88,193.21C160.23,216.7,133.58,226.06,128,227.82ZM220,56v56c0,20-3.77,38.49-11.2,54.94a4,4,0,0,1-3.65,2.36,4.06,4.06,0,0,1-1.65-.36,4,4,0,0,1-2-5.29c7-15.41,10.49-32.79,10.49-51.65V56a4,4,0,0,0-4-4H98.52a4,4,0,1,1,0-8H208A12,12,0,0,1,220,56Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M56.88,31.93A12,12,0,0,0,36,40a20,20,0,0,0-8,16v56c0,54.29,26.32,87.22,48.4,105.29,23.71,19.39,47.44,26,48.44,26.29a12.1,12.1,0,0,0,6.32,0c1.35-.37,30-8.42,55.86-32.82l12.1,13.31a12,12,0,0,0,17.76-16.14ZM128,219.38a126.38,126.38,0,0,1-37.09-21.23C65.09,176.69,52,147.71,52,112V62.24L170.87,193A126,126,0,0,1,128,219.38ZM228,56v56a144,144,0,0,1-8.23,49.16,12,12,0,0,1-11.28,7.92,11.86,11.86,0,0,1-4.08-.72,12,12,0,0,1-7.2-15.37A120.31,120.31,0,0,0,204,112V60H109.33a12,12,0,1,1,0-24H208A20,20,0,0,1,228,56Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M52.44,36a6,6,0,0,0-9.63,7A14,14,0,0,0,34,56v56c0,51.94,25.12,83.4,46.2,100.64,22.73,18.6,45.27,24.89,46.22,25.15a6,6,0,0,0,3.16,0c1.36-.37,31.91-8.95,57.67-35.7L203.56,220a6,6,0,0,0,8.88-8.08ZM128,225.72a130.83,130.83,0,0,1-40.56-22.66C59.94,180.39,46,149.75,46,112V56a2,2,0,0,1,2-2h4.6L179.16,193.19A130.68,130.68,0,0,1,128,225.72ZM222,56v56c0,20.29-3.83,39.05-11.38,55.77a6,6,0,0,1-5.47,3.53,5.86,5.86,0,0,1-2.47-.54,6,6,0,0,1-3-7.93C206.53,147.67,210,130.57,210,112V56a2,2,0,0,0-2-2H98.52a6,6,0,1,1,0-12H208A14,14,0,0,1,222,56Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M53.92,34.62A8,8,0,0,0,40.26,42,16,16,0,0,0,32,56v56c0,52.72,25.52,84.67,46.93,102.19,23.06,18.86,46,25.27,47,25.53a8,8,0,0,0,4.2,0c1.36-.37,31.27-8.78,57.09-34.72l14.89,16.38a8,8,0,1,0,11.84-10.76Zm74.07,189a128.48,128.48,0,0,1-38.92-21.81C61.82,179.51,48,149.3,48,112l0-56h3.71L176.41,193.15A129.26,129.26,0,0,1,128,223.62ZM224,56v56c0,20.58-3.89,39.61-11.56,56.59A8,8,0,1,1,197.86,162c6.73-14.89,10.14-31.71,10.14-50V56L98.52,56a8,8,0,1,1,0-16H208A16,16,0,0,1,224,56Z"></path>
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
