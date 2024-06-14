//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "design"))]
#[component]
pub fn ScribbleLoop(
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
                <path d="M128,128a81.4,81.4,0,0,1,25.69,4.28C151.56,154.87,137.33,176,112,176c-15.8,0-24.06-10.85-24.06-21.58,0-6.59,3-12.75,8.56-17.35C103.62,131.14,114.52,128,128,128Zm96-80V208a16,16,0,0,1-16,16H48a16,16,0,0,1-16-16V48A16,16,0,0,1,48,32H208A16,16,0,0,1,224,48Zm-18.11,98.59a120.21,120.21,0,0,0-36.08-25.21c-.9-14.35-5.75-27.54-13.89-37.55C145.38,70.86,130.19,64,112,64,76.44,64,50.68,97.76,49.6,99.2a8,8,0,0,0,12.79,9.62C62.61,108.53,84.51,80,112,80c13.4,0,24,4.68,31.5,13.92a47.54,47.54,0,0,1,9.48,21.4A96.75,96.75,0,0,0,128,112c-17.27,0-31.71,4.42-41.74,12.78C77,132.47,71.94,143,71.94,154.42,71.94,172.64,86,192,112,192a54,54,0,0,0,43.53-21.23A70,70,0,0,0,169,138.89a106.24,106.24,0,0,1,25.13,18.52,8,8,0,1,0,11.78-10.82Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M108.6,208c-64,0-64-96,32-96a134.44,134.44,0,0,1,44.73,7.83h0C187,162.36,159.88,208,108.6,208Z"
        opacity="0.2"
    ></path>
    <path d="M253.93,154.63c-1.32-1.46-24.09-26.22-61-40.56-1.72-18.42-8.46-35.17-19.41-47.92C158.87,49,137.58,40,112,40,60.48,40,26.89,86.18,25.49,88.15a8,8,0,0,0,13,9.31C38.8,97.05,68.81,56,112,56c20.77,0,37.86,7.11,49.41,20.57,7.42,8.64,12.44,19.69,14.67,32A140.87,140.87,0,0,0,140.6,104c-26.06,0-47.93,6.81-63.26,19.69C63.78,135.09,56,151,56,167.25A47.59,47.59,0,0,0,69.87,201.3c9.66,9.62,23.06,14.7,38.73,14.7,51.81,0,81.18-42.13,84.49-84.42a161.43,161.43,0,0,1,49,33.79,8,8,0,1,0,11.86-10.74Zm-94.46,21.64C150.64,187.09,134.66,200,108.6,200,83.32,200,72,183.55,72,167.25,72,144.49,93.47,120,140.6,120a124.34,124.34,0,0,1,36.78,5.68C176.93,144.44,170.46,162.78,159.47,176.27Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M251,157.32c-1.32-1.46-24.47-26.63-61.79-40.43-1.26-18.56-7.78-35.45-18.66-48.13C156.62,52.56,136.38,44,112,44,62.51,44,30.1,88.58,28.74,90.48a4,4,0,0,0,6.51,4.65C35.56,94.7,66.68,52,112,52c22,0,40.11,7.6,52.45,22,9.11,10.61,14.81,24.62,16.46,40.13A137.84,137.84,0,0,0,140.6,108c-25.1,0-46.09,6.48-60.69,18.75C67.26,137.39,60,152.15,60,167.25a43.64,43.64,0,0,0,12.69,31.22C81.59,207.32,94,212,108.6,212c51.63,0,79.87-44.08,80.78-86.32,34.07,13.58,55.36,36.67,55.65,37a4,4,0,1,0,5.94-5.36Zm-88.4,21.47c-9.37,11.5-26.34,25.21-54,25.21C80.71,204,68,185,68,167.25,68,142.57,90.72,116,140.6,116a129.23,129.23,0,0,1,40.8,6.77v.81C181.4,144,174.54,164.1,162.57,178.79Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M252.9,152c-1.33-1.47-23.75-25.85-60.21-40.69-2.15-18.28-9.1-34.9-20.1-47.71C157.12,45.52,134.79,36,108,36,54.44,36,19.69,83.79,18.24,85.82a12,12,0,0,0,19.53,14C38.05,99.38,66.65,60,108,60c19.85,0,35.45,6.45,46.38,19.18a61.35,61.35,0,0,1,12.4,24A143.6,143.6,0,0,0,136.61,100c-27,0-49.79,7.13-65.85,20.63C56.3,132.79,48,149.78,48,167.25,48,193.46,67.44,220,104.61,220c51.93,0,82.34-40.28,87.87-82.43a156.46,156.46,0,0,1,42.62,30.48A12,12,0,1,0,252.9,152ZM152.37,173.74C144.09,183.9,129.09,196,104.61,196,82.08,196,72,181.56,72,167.25,72,146.42,92.22,124,136.61,124a119.85,119.85,0,0,1,32.64,4.62C168.24,145.44,162.21,161.66,152.37,173.74Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M252.45,156c-1.32-1.46-24.27-26.42-61.37-40.5-1.5-18.49-8.13-35.31-19-48C157.74,50.8,137,42,112,42,61.49,42,28.5,87.38,27.12,89.31a6,6,0,1,0,9.76,7C37.18,95.87,67.75,54,112,54c21.37,0,39,7.35,50.93,21.27,8.26,9.62,13.64,22.14,15.62,36.06a139,139,0,0,0-38-5.33c-25.58,0-47,6.65-62,19.22-13.1,11-20.62,26.34-20.62,42a45.65,45.65,0,0,0,13.28,32.64C80.56,209.12,93.47,214,108.6,214c51.73,0,80.55-43.09,82.68-85.38,32.05,13.49,52,35.09,52.27,35.4a6,6,0,0,0,8.9-8ZM161,177.53C151.92,188.69,135.44,202,108.6,202,90,202,70,191.12,70,167.25,70,143.53,92.09,118,140.6,118a126.74,126.74,0,0,1,38.8,6.22C179.26,143.94,172.58,163.34,161,177.53Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M253.93,154.63c-1.32-1.46-24.09-26.22-61-40.56-1.72-18.42-8.46-35.17-19.41-47.92C158.87,49,137.58,40,112,40,60.48,40,26.89,86.18,25.49,88.15a8,8,0,0,0,13,9.31C38.8,97.05,68.81,56,112,56c20.77,0,37.86,7.11,49.41,20.57,7.42,8.64,12.44,19.69,14.67,32A140.87,140.87,0,0,0,140.6,104c-26.06,0-47.93,6.81-63.26,19.69C63.78,135.09,56,151,56,167.25A47.59,47.59,0,0,0,69.87,201.3c9.66,9.62,23.06,14.7,38.73,14.7,51.81,0,81.18-42.13,84.49-84.42a161.43,161.43,0,0,1,49,33.79,8,8,0,1,0,11.86-10.74Zm-94.46,21.64C150.64,187.09,134.66,200,108.6,200,83.32,200,72,183.55,72,167.25,72,144.49,93.47,120,140.6,120a124.34,124.34,0,0,1,36.78,5.68C176.93,144.44,170.46,162.78,159.47,176.27Z"></path>
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
