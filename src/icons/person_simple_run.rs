//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "map", feature = "people", feature = "health"))]
#[component]
pub fn PersonSimpleRun(
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
                <path d="M120,56a32,32,0,1,1,32,32A32,32,0,0,1,120,56Zm103.28,74.08a8,8,0,0,0-10.6-4c-.25.12-26.71,10.72-72.18-20.19-52.29-35.54-88-7.77-89.51-6.57a8,8,0,1,0,10,12.48c.26-.21,25.12-19.5,64.07,3.27-4.25,13.35-12.76,31.82-25.25,47-18.56,22.48-41.11,32.56-67,30A8,8,0,0,0,31.2,208a92.29,92.29,0,0,0,9.34.47c27.38,0,52-12.38,71.63-36.18.57-.69,1.14-1.4,1.69-2.1C133.31,175.29,168,190.3,168,232a8,8,0,0,0,16,0c0-24.65-10.08-45.35-29.15-59.86a104.29,104.29,0,0,0-31.31-15.81A169.31,169.31,0,0,0,139,124c26.14,16.09,46.84,20,60.69,20,12.18,0,19.06-3,19.67-3.28A8,8,0,0,0,223.28,130.08Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M176,56a24,24,0,1,1-24-24A24,24,0,0,1,176,56Z" opacity="0.2"></path>
    <path d="M152,88a32,32,0,1,0-32-32A32,32,0,0,0,152,88Zm0-48a16,16,0,1,1-16,16A16,16,0,0,1,152,40Zm67.31,100.68c-.61.28-7.49,3.28-19.67,3.28-13.85,0-34.55-3.88-60.69-20a169.31,169.31,0,0,1-15.41,32.34,104.29,104.29,0,0,1,31.31,15.81C173.92,186.65,184,207.35,184,232a8,8,0,0,1-16,0c0-41.7-34.69-56.71-54.14-61.85-.55.7-1.12,1.41-1.69,2.1-19.64,23.8-44.25,36.18-71.63,36.18A92.29,92.29,0,0,1,31.2,208,8,8,0,0,1,32.8,192c25.92,2.59,48.47-7.49,67-30,12.49-15.14,21-33.61,25.25-47C86.13,92.34,61.27,111.63,61,111.84A8,8,0,1,1,51,99.36c1.5-1.2,37.22-29,89.51,6.57,45.47,30.91,71.93,20.31,72.18,20.19a8,8,0,1,1,6.63,14.56Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M152,84a28,28,0,1,0-28-28A28,28,0,0,0,152,84Zm0-48a20,20,0,1,1-20,20A20,20,0,0,1,152,36Zm65.66,101c-.57.26-6.84,3-18.08,3-13.86,0-35.25-4.15-62.81-22.16a162.59,162.59,0,0,1-19.49,40.78c9.47,2.56,23.08,7.5,35.14,16.67,18.3,13.92,27.58,33,27.58,56.68a4,4,0,0,1-8,0c0-15.89-5.88-53.77-59.7-66.37q-1.56,2.06-3.22,4.08c-18.85,22.83-42.42,34.72-68.6,34.72q-4.4,0-8.89-.45a4,4,0,1,1,.8-8c27.33,2.73,51.06-7.83,70.52-31.41,13.82-16.74,22.89-37.44,26.9-51.32-42.84-26.69-71-4.8-71.32-4.57a4,4,0,1,1-5-6.24c.36-.29,9-7.1,23.84-9.58,13.5-2.27,35-1.26,60.91,16.34,25,17,44.41,21.64,56.29,22.56,12.75,1,19.77-2,19.84-2.05a4,4,0,0,1,3.29,7.29Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M152,92a36,36,0,1,0-36-36A36,36,0,0,0,152,92Zm0-48a12,12,0,1,1-12,12A12,12,0,0,1,152,44Zm76,93.4a12,12,0,0,1-7,10.91,66,66,0,0,1-21.47,3.78c-14,0-34.25-3.82-59.77-19a177,177,0,0,1-10.27,21C153.12,162.83,188,183.8,188,232a12,12,0,0,1-24,0c0-18.69-6.95-33.06-21.26-43.94-9.16-7-19.55-11-27.43-13.34-.81,1-1.64,2-2.5,2.95-20,22.87-44.82,34.76-72.25,34.76a97.33,97.33,0,0,1-9.75-.49,12,12,0,1,1,2.39-23.88c52.3,5.22,77.48-45.92,85.79-67.75C84.8,102.46,63.74,118.78,63.51,119a12,12,0,0,1-15-18.72C50.08,99,88,69.44,142.75,106.62c43.1,29.31,68.1,19.92,68.5,19.76a12,12,0,0,1,16.75,11Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M152,86a30,30,0,1,0-30-30A30,30,0,0,0,152,86Zm0-48a18,18,0,1,1-18,18A18,18,0,0,1,152,38Zm66.49,100.86c-.59.27-7.17,3.13-18.88,3.13-13.86,0-34.9-4-61.73-21a165.89,165.89,0,0,1-17.43,36.51c9.43,2.78,22,7.72,33.19,16.26C172.46,188.05,182,207.65,182,232a6,6,0,0,1-12,0c0-44-37.23-59.18-56.91-64.11q-1.2,1.55-2.46,3.09c-19.25,23.31-43.34,35.45-70.11,35.45A90.72,90.72,0,0,1,31.4,206,6,6,0,0,1,32.6,194c26.63,2.66,49.77-7.66,68.77-30.69,13.16-15.94,21.94-35.51,26.08-49.15-40.51-24.52-66.59-4.78-67.72-3.89a6,6,0,0,1-7.48-9.38c.37-.3,9.39-7.43,24.76-10,13.86-2.31,35.92-1.3,62.36,16.67,47.14,32,73.88,20.47,74.14,20.35a6,6,0,1,1,5,10.92Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M152,88a32,32,0,1,0-32-32A32,32,0,0,0,152,88Zm0-48a16,16,0,1,1-16,16A16,16,0,0,1,152,40Zm67.31,100.68c-.61.28-7.49,3.28-19.67,3.28-13.85,0-34.55-3.88-60.69-20a169.31,169.31,0,0,1-15.41,32.34,104.29,104.29,0,0,1,31.31,15.81C173.92,186.65,184,207.35,184,232a8,8,0,0,1-16,0c0-41.7-34.69-56.71-54.14-61.85-.55.7-1.12,1.41-1.69,2.1-19.64,23.8-44.25,36.18-71.63,36.18A92.29,92.29,0,0,1,31.2,208,8,8,0,0,1,32.8,192c25.92,2.58,48.47-7.49,67-30,12.49-15.14,21-33.61,25.25-47C86.13,92.35,61.27,111.63,61,111.84A8,8,0,1,1,51,99.36c1.5-1.2,37.22-29,89.51,6.57,45.47,30.91,71.93,20.31,72.18,20.19a8,8,0,1,1,6.63,14.56Z"></path>
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
