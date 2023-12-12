//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn BracketsCurly(
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
                <path d="M54.8,119.49A35.06,35.06,0,0,1,49.05,128a35.06,35.06,0,0,1,5.75,8.51C60,147.24,60,159.83,60,172c0,25.94,1.84,32,20,32a12,12,0,0,1,0,24c-19.14,0-32.2-6.9-38.8-20.51C36,196.76,36,184.17,36,172c0-25.94-1.84-32-20-32a12,12,0,0,1,0-24c18.16,0,20-6.06,20-32,0-12.17,0-24.76,5.2-35.49C47.8,34.9,60.86,28,80,28a12,12,0,0,1,0,24c-18.16,0-20,6.06-20,32C60,96.17,60,108.76,54.8,119.49ZM240,116c-18.16,0-20-6.06-20-32,0-12.17,0-24.76-5.2-35.49C208.2,34.9,195.14,28,176,28a12,12,0,0,0,0,24c18.16,0,20,6.06,20,32,0,12.17,0,24.76,5.2,35.49A35.06,35.06,0,0,0,207,128a35.06,35.06,0,0,0-5.75,8.51C196,147.24,196,159.83,196,172c0,25.94-1.84,32-20,32a12,12,0,0,0,0,24c19.14,0,32.2-6.9,38.8-20.51C220,196.76,220,184.17,220,172c0-25.94,1.84-32,20-32a12,12,0,0,0,0-24Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M240,128c-64,0,0,88-64,88H80c-64,0,0-88-64-88,64,0,0-88,64-88h96C240,40,176,128,240,128Z"
        opacity="0.2"
    ></path>
    <path d="M43.18,128a29.78,29.78,0,0,1,8,10.26c4.8,9.9,4.8,22,4.8,33.74,0,24.31,1,36,24,36a8,8,0,0,1,0,16c-17.48,0-29.32-6.14-35.2-18.26-4.8-9.9-4.8-22-4.8-33.74,0-24.31-1-36-24-36a8,8,0,0,1,0-16c23,0,24-11.69,24-36,0-11.72,0-23.84,4.8-33.74C50.68,38.14,62.52,32,80,32a8,8,0,0,1,0,16C57,48,56,59.69,56,84c0,11.72,0,23.84-4.8,33.74A29.78,29.78,0,0,1,43.18,128ZM240,120c-23,0-24-11.69-24-36,0-11.72,0-23.84-4.8-33.74C205.32,38.14,193.48,32,176,32a8,8,0,0,0,0,16c23,0,24,11.69,24,36,0,11.72,0,23.84,4.8,33.74a29.78,29.78,0,0,0,8,10.26,29.78,29.78,0,0,0-8,10.26c-4.8,9.9-4.8,22-4.8,33.74,0,24.31-1,36-24,36a8,8,0,0,0,0,16c17.48,0,29.32-6.14,35.2-18.26,4.8-9.9,4.8-22,4.8-33.74,0-24.31,1-36,24-36a8,8,0,0,0,0-16Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M216,40H40A16,16,0,0,0,24,56V200a16,16,0,0,0,16,16H216a16,16,0,0,0,16-16V56A16,16,0,0,0,216,40ZM88,155.84c.29,14.26.41,20.16,16,20.16a8,8,0,0,1,0,16c-31.27,0-31.72-22.43-32-35.84C71.71,141.9,71.59,136,56,136a8,8,0,0,1,0-16c15.59,0,15.71-5.9,16-20.16C72.28,86.43,72.73,64,104,64a8,8,0,0,1,0,16c-15.59,0-15.71,5.9-16,20.16-.17,8.31-.41,20.09-8,27.84C87.59,135.75,87.83,147.53,88,155.84ZM200,136c-15.59,0-15.71,5.9-16,20.16-.28,13.41-.73,35.84-32,35.84a8,8,0,0,1,0-16c15.59,0,15.71-5.9,16-20.16.17-8.31.41-20.09,8-27.84-7.6-7.75-7.84-19.53-8-27.84C167.71,85.9,167.59,80,152,80a8,8,0,0,1,0-16c31.27,0,31.72,22.43,32,35.84.29,14.26.41,20.16,16,20.16a8,8,0,0,1,0,16Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M39.91,128a27.68,27.68,0,0,1,9.49,11.13C54,148.62,54,160.51,54,172c0,24.27,1.21,38,26,38a6,6,0,0,1,0,12c-16.88,0-27.81-5.6-33.4-17.13C42,195.38,42,183.49,42,172c0-24.27-1.21-38-26-38a6,6,0,0,1,0-12c24.79,0,26-13.73,26-38,0-11.49,0-23.38,4.6-32.87C52.19,39.6,63.12,34,80,34a6,6,0,0,1,0,12C55.21,46,54,59.73,54,84c0,11.49,0,23.38-4.6,32.87A27.68,27.68,0,0,1,39.91,128ZM240,122c-24.79,0-26-13.73-26-38,0-11.49,0-23.38-4.6-32.87C203.81,39.6,192.88,34,176,34a6,6,0,0,0,0,12c24.79,0,26,13.73,26,38,0,11.49,0,23.38,4.6,32.87A27.68,27.68,0,0,0,216.09,128a27.68,27.68,0,0,0-9.49,11.13C202,148.62,202,160.51,202,172c0,24.27-1.21,38-26,38a6,6,0,0,0,0,12c16.88,0,27.81-5.6,33.4-17.13,4.6-9.49,4.6-21.38,4.6-32.87,0-24.27,1.21-38,26-38a6,6,0,0,0,0-12Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M43.18,128a29.78,29.78,0,0,1,8,10.26c4.8,9.9,4.8,22,4.8,33.74,0,24.31,1,36,24,36a8,8,0,0,1,0,16c-17.48,0-29.32-6.14-35.2-18.26-4.8-9.9-4.8-22-4.8-33.74,0-24.31-1-36-24-36a8,8,0,0,1,0-16c23,0,24-11.69,24-36,0-11.72,0-23.84,4.8-33.74C50.68,38.14,62.52,32,80,32a8,8,0,0,1,0,16C57,48,56,59.69,56,84c0,11.72,0,23.84-4.8,33.74A29.78,29.78,0,0,1,43.18,128ZM240,120c-23,0-24-11.69-24-36,0-11.72,0-23.84-4.8-33.74C205.32,38.14,193.48,32,176,32a8,8,0,0,0,0,16c23,0,24,11.69,24,36,0,11.72,0,23.84,4.8,33.74a29.78,29.78,0,0,0,8,10.26,29.78,29.78,0,0,0-8,10.26c-4.8,9.9-4.8,22-4.8,33.74,0,24.31-1,36-24,36a8,8,0,0,0,0,16c17.48,0,29.32-6.14,35.2-18.26,4.8-9.9,4.8-22,4.8-33.74,0-24.31,1-36,24-36a8,8,0,0,0,0-16Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M35.89,128C52,136.23,52,155.64,52,172c0,24.8,1.35,40,28,40a4,4,0,0,1,0,8c-36,0-36-26.61-36-48,0-24.8-1.35-40-28-40a4,4,0,0,1,0-8c26.65,0,28-15.2,28-40,0-21.39,0-48,36-48a4,4,0,0,1,0,8C53.35,44,52,59.2,52,84,52,100.36,52,119.77,35.89,128ZM240,124c-26.65,0-28-15.2-28-40,0-21.39,0-48-36-48a4,4,0,0,0,0,8c26.65,0,28,15.2,28,40,0,16.36,0,35.77,16.11,44C204,136.23,204,155.64,204,172c0,24.8-1.35,40-28,40a4,4,0,0,0,0,8c36,0,36-26.61,36-48,0-24.8,1.35-40,28-40a4,4,0,0,0,0-8Z"></path>
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
