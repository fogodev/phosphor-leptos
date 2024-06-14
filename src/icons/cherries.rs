//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "nature", feature = "commerce"))]
#[component]
pub fn Cherries(
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
                <path d="M178.42,72a75.24,75.24,0,0,0-10.21.37,91.9,91.9,0,0,0-21.59-25.09C108.78,16.79,57.05,23.77,54.87,24.08A8,8,0,0,0,48,31.43a8.19,8.19,0,0,0,3.69,7.32c17.4,11.68,25.37,30.91,28.7,49.65a72.08,72.08,0,1,0,16.26.14C93.54,68.59,86.56,52,76,39.37c16.67.72,41.24,4.78,60.64,20.48a74.76,74.76,0,0,1,15,16.39c-1.9.69-3.79,1.44-5.65,2.29a8.42,8.42,0,0,0-4.49,4.63,8,8,0,0,0,2.41,9,88.9,88.9,0,0,1,13.59,14,3.64,3.64,0,0,0,.65.65C160,108.15,165.83,112,176,112c12.15,0,18.18-5.51,18.43-5.75l-.09.09a8,8,0,1,1,11.32,11.32C204.6,118.72,194.77,128,176,128l-1.61,0a3,3,0,0,0-3,4,87.91,87.91,0,0,1-7,71.6,8.39,8.39,0,0,0-1,6.24,8,8,0,0,0,7.16,6c1.78.13,3.59.2,5.37.2a72,72,0,0,0,2.42-144Zm-72,50.21-.09.09a8,8,0,0,1,11.32,11.32C116.6,134.72,106.77,144,88,144s-28.6-9.28-29.66-10.34a8,8,0,0,1,11.32-11.32l-.09-.09c.25.24,6.28,5.75,18.43,5.75S106.18,122.49,106.43,122.25Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M240,144a64,64,0,0,1-99.78,53.07l0-.07a64,64,0,1,1-16.39-90A64,64,0,0,1,240,144Z"
        opacity="0.2"
    ></path>
    <path d="M176,72a71.5,71.5,0,0,0-7.76.43,91.77,91.77,0,0,0-21.62-25.11C108.78,16.79,57.05,23.77,54.87,24.08a8,8,0,0,0-3.31,14.58c17.5,11.66,25.51,31,28.85,49.74A72,72,0,1,0,142,207.5,72,72,0,1,0,176,72ZM76,39.37c16.67.72,41.24,4.78,60.64,20.48a75.28,75.28,0,0,1,15,16.4A72.2,72.2,0,0,0,121.9,96.5a71.46,71.46,0,0,0-25.23-8C93.54,68.59,86.56,52,76,39.37ZM88,216a56,56,0,0,1-5.76-111.7,176.49,176.49,0,0,1-1,31.08c-7.58-1.43-11.35-4.85-11.55-5a8,8,0,0,0-11.32,11.32C59.4,142.71,69.2,152,87.92,152h.25c18.66-.05,28.43-9.28,29.49-10.33a8,8,0,0,0-11.32-11.32,22.31,22.31,0,0,1-8.93,4.44A190.36,190.36,0,0,0,98.34,105,56,56,0,0,1,88,216Zm57-99.89a72.27,72.27,0,0,0-9.82-10.42,56.15,56.15,0,0,1,24.22-15.16A110.84,110.84,0,0,1,167,118.88c-6.09-1.6-9.16-4.37-9.33-4.54A8,8,0,0,0,145,116.11ZM176,200a55.76,55.76,0,0,1-24.69-5.73,71.83,71.83,0,0,0,2.5-63.42A47.47,47.47,0,0,0,175.67,136H176c18.77,0,28.6-9.28,29.66-10.34a8,8,0,0,0-11.32-11.32c-.19.19-3.84,3.49-11.15,5A131.66,131.66,0,0,0,175.7,88h.3a56,56,0,0,1,0,112Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M176,76a67.05,67.05,0,0,0-9.94.74,88.12,88.12,0,0,0-21.95-26.3C107.61,21,57.55,27.74,55.43,28a4,4,0,0,0-1.65,7.29C73.74,48.64,82,71.09,85,92.08a68,68,0,1,0,56.12,110.29A68,68,0,1,0,176,76ZM66.62,35.27c16-.4,48,1.66,72.47,21.39a79.89,79.89,0,0,1,18.75,21.82,68.29,68.29,0,0,0-35,23.14,67.44,67.44,0,0,0-29.66-9.4C89.88,67.78,80.83,48.37,66.62,35.27ZM88,220a60,60,0,0,1-2.09-119.95,177.82,177.82,0,0,1-1.28,39.8c-11.7-.93-17.56-6.44-17.8-6.68a4,4,0,0,0-5.66,5.66c.38.37,9.37,9.17,26.83,9.17s26.45-8.8,26.83-9.17a4,4,0,0,0-5.64-5.68c-.25.25-5.65,5.35-16.43,6.58A183.7,183.7,0,0,0,94,100.3,60,60,0,0,1,88,220Zm88-16a59.75,59.75,0,0,1-30.24-8.17,67.86,67.86,0,0,0-16.31-89.68,60.25,60.25,0,0,1,32.22-20.4c5.17,11,8.49,23.68,9.93,38-11-1.16-16.53-6.36-16.77-6.59a4,4,0,0,0-5.66,5.66c.38.37,9.37,9.17,26.83,9.17s26.45-8.8,26.83-9.17a4,4,0,0,0-5.64-5.68c-.26.25-6,5.68-17.55,6.68q-2.1-22.14-9.81-39.49A60,60,0,1,1,176,204Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M176,68c-1.88,0-3.76.08-5.62.22a95.72,95.72,0,0,0-21.24-24C110,12.6,56.56,19.8,54.3,20.12a12,12,0,0,0-5,21.86C64.52,52.1,72.13,68.45,75.71,85a76,76,0,1,0,67.22,127.43A76,76,0,1,0,176,68Zm-22.76,53.06A76.78,76.78,0,0,0,141,105.59a52.11,52.11,0,0,1,16.14-10,106.78,106.78,0,0,1,5.12,18.9A12,12,0,0,0,153.24,121.06ZM134.19,63a70.79,70.79,0,0,1,11.23,11.46A76.31,76.31,0,0,0,121,91.55,75.37,75.37,0,0,0,100.19,85C97.72,71.05,93,56.65,84.58,44,99.78,45.68,118.75,50.48,134.19,63ZM88,212a52,52,0,0,1-9.53-103.11,173.37,173.37,0,0,1-.75,21.59,12,12,0,0,0-5.09,23.38,56.27,56.27,0,0,0,30.74,0,12,12,0,0,0-1.5-23.36,193.45,193.45,0,0,0,.69-20.41A52,52,0,0,1,88,212Zm88-16a51.92,51.92,0,0,1-19.24-3.68,75.83,75.83,0,0,0,4-54.43,56.36,56.36,0,0,0,30.66,0,12,12,0,0,0-4.79-23.42,136.4,136.4,0,0,0-5.12-22.15A52,52,0,0,1,176,196Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M176,74a68.2,68.2,0,0,0-8.83.57,89.81,89.81,0,0,0-21.8-25.69c-37.18-30-88.07-23.13-90.22-22.82A6,6,0,0,0,52.67,37C71.39,49.47,79.54,70.31,82.72,90.22A70,70,0,1,0,141.59,205,70,70,0,1,0,176,74ZM137.9,58.27a77.61,77.61,0,0,1,16.86,19A70.31,70.31,0,0,0,122.35,99a69.47,69.47,0,0,0-27.42-8.69c-3.22-22.13-11.2-40.14-23.46-53.1C88.23,37.43,116.19,40.72,137.9,58.27ZM146.1,121a70.74,70.74,0,0,0-13.77-15.12,58.17,58.17,0,0,1,28.23-17.74,113.53,113.53,0,0,1,8.78,33.31c-8.61-1.5-12.92-5.51-13.1-5.68A6,6,0,0,0,146.1,121ZM88,218a58,58,0,0,1-3.92-115.85,177.17,177.17,0,0,1-1.15,35.52c-9.66-1.25-14.5-5.73-14.69-5.91a6,6,0,0,0-8.48,8.48c.4.4,10,9.76,28.24,9.76s27.84-9.36,28.24-9.76a6,6,0,1,0-8.43-8.53c-.22.2-4.41,4.08-12.72,5.64a186.4,186.4,0,0,0,1.11-34.76A58,58,0,0,1,88,218Zm88-16a57.75,57.75,0,0,1-27.47-6.91,69.73,69.73,0,0,0,0-70.12c2.65,2.26,11.88,9,27.44,9,18.28,0,27.84-9.36,28.24-9.76a6,6,0,0,0,0-8.51,6,6,0,0,0-8.46,0c-.23.22-5,4.59-14.36,5.91a129.06,129.06,0,0,0-8.66-35.52c1.06-.06,2.14-.1,3.21-.1a58,58,0,0,1,0,116Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M176,72a71.5,71.5,0,0,0-7.76.43,91.77,91.77,0,0,0-21.62-25.11C108.78,16.79,57.05,23.77,54.87,24.08a8,8,0,0,0-3.31,14.58c17.5,11.66,25.51,31,28.85,49.74A72,72,0,1,0,142,207.5,72,72,0,1,0,176,72ZM76,39.37c16.67.72,41.24,4.78,60.64,20.48a75.28,75.28,0,0,1,15,16.4A72.2,72.2,0,0,0,121.9,96.5a71.46,71.46,0,0,0-25.23-8C93.54,68.59,86.56,52,76,39.37ZM88,216a56,56,0,0,1-5.76-111.7,176.49,176.49,0,0,1-1,31.08c-7.58-1.43-11.35-4.85-11.55-5a8,8,0,0,0-11.32,11.32C59.4,142.71,69.2,152,87.92,152h.25c18.66-.05,28.43-9.28,29.49-10.33a8,8,0,0,0-11.32-11.32,22.31,22.31,0,0,1-8.93,4.44A190.36,190.36,0,0,0,98.34,105,56,56,0,0,1,88,216Zm57-99.89a72.27,72.27,0,0,0-9.82-10.42,56.15,56.15,0,0,1,24.22-15.16A110.84,110.84,0,0,1,167,118.88c-6.09-1.6-9.16-4.37-9.33-4.54A8,8,0,0,0,145,116.11ZM176,200a55.76,55.76,0,0,1-24.69-5.73,71.83,71.83,0,0,0,2.5-63.42A47.47,47.47,0,0,0,175.67,136H176c18.77,0,28.6-9.28,29.66-10.34a8,8,0,0,0-11.32-11.32c-.19.19-3.84,3.49-11.15,5A131.66,131.66,0,0,0,175.7,88h.3a56,56,0,0,1,0,112Z"></path>
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
