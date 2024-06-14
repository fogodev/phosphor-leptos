//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "commerce"))]
#[component]
pub fn GrainsSlash(
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
                <path d="M216,64v80a87.66,87.66,0,0,1-5.45,30.49,4,4,0,0,1-6.7,1.27L162.52,130.3a4,4,0,0,1,1-6.16A87.36,87.36,0,0,1,200,112.37V72.45a72.33,72.33,0,0,0-50.35,29.36A8,8,0,0,1,137.72,103a8.17,8.17,0,0,1-.89-10.75,88.06,88.06,0,0,1,25-23.11C152.62,49.8,135.45,37.74,128,33.2a99.79,99.79,0,0,0-23.4,19.94,8,8,0,0,1-12,.27,8.18,8.18,0,0,1-.06-10.8,112.35,112.35,0,0,1,31.86-25.76,8,8,0,0,1,7.16,0c1.32.66,30.27,15.43,44.59,45.15a87.86,87.86,0,0,1,31.74-6A8,8,0,0,1,216,64Zm-2.08,146.62a8,8,0,1,1-11.84,10.76l-12.9-14.19A87.77,87.77,0,0,1,128.52,232C79.83,232.28,40,191.51,40,142.83V64a8,8,0,0,1,8.09-8c1.25,0,2.48,0,3.72.09L42.08,45.38A8,8,0,1,1,53.92,34.62Zm-77.6-61.57L69.18,75.19A71.31,71.31,0,0,0,56,72.44v39.94a88.17,88.17,0,0,1,72,51A88.22,88.22,0,0,1,136.32,149.05Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M208,64v56a80,80,0,0,0-80,80,80,80,0,0,0-80-80V64a79.67,79.67,0,0,1,36.05,8.59v0C95.62,40.19,128,24,128,24S160.39,40.19,172,72.58h0A79.6,79.6,0,0,1,208,64Z"
        opacity="0.2"
    ></path>
    <path d="M53.92,34.62A8,8,0,1,0,42.08,45.38l9.73,10.71Q49.91,56,48,56a8,8,0,0,0-8,8v80a88,88,0,0,0,149.21,63.22l12.87,14.16a8,8,0,1,0,11.84-10.76ZM120,215.56A72.1,72.1,0,0,1,56,144V128.44A72.1,72.1,0,0,1,120,200ZM56,112.37V72.44a71.31,71.31,0,0,1,13.18,2.75L136.29,149A88.17,88.17,0,0,0,128,163.37,88.16,88.16,0,0,0,56,112.37Zm80,103.19V200a72.09,72.09,0,0,1,11.36-38.81l31.08,34.19A71.85,71.85,0,0,1,136,215.56ZM216,64v80a88.13,88.13,0,0,1-3.15,23.4,8,8,0,0,1-7.71,5.88A7.79,7.79,0,0,1,203,173a8,8,0,0,1-5.59-9.83A72.55,72.55,0,0,0,200,144V128.43a71.07,71.07,0,0,0-24.56,7.33,8,8,0,1,1-7.24-14.26,86.64,86.64,0,0,1,31.8-9.14V72.45a72.33,72.33,0,0,0-50.35,29.36,8,8,0,1,1-13-9.39,88.15,88.15,0,0,1,25.16-23.3C152.62,49.8,135.45,37.74,128,33.2a99.79,99.79,0,0,0-23.4,19.94A8,8,0,0,1,92.39,42.81a112.32,112.32,0,0,1,32-26,8,8,0,0,1,7.16,0c1.32.66,30.27,15.43,44.59,45.15A87.91,87.91,0,0,1,208,56,8,8,0,0,1,216,64Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M51,37.31A4,4,0,0,0,45,42.69L61.81,61.13A84.83,84.83,0,0,0,48,60a4,4,0,0,0-4,4v80a84.09,84.09,0,0,0,84,84h0a83.82,83.82,0,0,0,61.33-26.6L205,218.69a4,4,0,1,0,5.92-5.38ZM71.39,71.68l70.06,77.06A84.23,84.23,0,0,0,128,174.41,84.17,84.17,0,0,0,52,116.1v-48A75.86,75.86,0,0,1,71.39,71.68ZM124,219.9A76.11,76.11,0,0,1,52,144V124.1A76.11,76.11,0,0,1,124,200Zm8,0V200a76,76,0,0,1,14.94-45.22l37,40.68A75.86,75.86,0,0,1,132,219.9ZM212,144a84.32,84.32,0,0,1-3,22.34,4,4,0,0,1-3.86,2.94,3.86,3.86,0,0,1-1.06-.14,4,4,0,0,1-2.8-4.92A76,76,0,0,0,204,144V124.1a74.9,74.9,0,0,0-30.37,8.1,4,4,0,1,1-3.62-7.14,82.94,82.94,0,0,1,34-9v-48a76.32,76.32,0,0,0-57.59,31.35,4,4,0,1,1-6.48-4.69,84.14,84.14,0,0,1,27-24C156.88,46.53,134.48,32.28,128,28.56a103.64,103.64,0,0,0-26.45,22,4,4,0,0,1-6.11-5.17,108.09,108.09,0,0,1,30.77-25,4,4,0,0,1,3.58,0c1.32.66,31.31,16,44.33,46.75A83.91,83.91,0,0,1,208,60a4,4,0,0,1,4,4Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M56.88,31.93A12,12,0,1,0,39.12,48.07l4.37,4.81A12,12,0,0,0,36,64v80a92,92,0,0,0,153,68.9l10.16,11.17a12,12,0,0,0,17.76-16.14Zm3.31,117.16a68.16,68.16,0,0,1,55.62,61.82A68.16,68.16,0,0,1,60.19,149.09ZM128,170.61a92.22,92.22,0,0,0-68-45.83V77.05a68.53,68.53,0,0,1,6.94,1.63l70.83,77.9A92.45,92.45,0,0,0,128,170.61Zm12.19,40.29a68.26,68.26,0,0,1,13.92-36.34l18.7,20.57A67.74,67.74,0,0,1,140.19,210.9ZM220,64v80a92.8,92.8,0,0,1-1,13.34,12,12,0,0,1-11.86,10.28,12.74,12.74,0,0,1-1.74-.13,12,12,0,0,1-10.15-13.6c.23-1.59.4-3.19.52-4.8-.76.14-1.53.29-2.29.45a12,12,0,1,1-5.08-23.45c2.49-.54,5-1,7.56-1.31V77.07a68,68,0,0,0-38.79,21.72,12,12,0,1,1-17.92-16A91.73,91.73,0,0,1,156.62,67.7c-8.2-14.93-21.31-25-28.63-29.76a97.84,97.84,0,0,0-15.56,12.6,12,12,0,0,1-17-16.95,116,116,0,0,1,27.19-20.32,12,12,0,0,1,10.74,0c1.33.66,29.36,15,44.74,43.74A91.83,91.83,0,0,1,208,52,12,12,0,0,1,220,64Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M52.44,36A6,6,0,0,0,43.56,44l13.1,14.41A86,86,0,0,0,48,58a6,6,0,0,0-6,6v80a86.1,86.1,0,0,0,86,86h0a85.8,85.8,0,0,0,61.28-25.67L203.56,220a6,6,0,0,0,8.88-8.08ZM70.27,73.42l68.6,75.45A85.9,85.9,0,0,0,128,168.5a86.18,86.18,0,0,0-74-54.27v-44A73.11,73.11,0,0,1,70.27,73.42ZM122,217.76A74.1,74.1,0,0,1,54,144V126.24A74.1,74.1,0,0,1,122,200Zm12,0V200A74.05,74.05,0,0,1,147.12,158l34.06,37.47A73.81,73.81,0,0,1,134,217.76ZM214,64v80a86.2,86.2,0,0,1-3.08,22.87,6,6,0,0,1-5.78,4.41,6.26,6.26,0,0,1-1.59-.21,6,6,0,0,1-4.2-7.38A74.07,74.07,0,0,0,202,144V126.24A72.92,72.92,0,0,0,174.54,134a6,6,0,1,1-5.44-10.7,84.85,84.85,0,0,1,32.9-9.07v-44a74.29,74.29,0,0,0-54,30.39,6,6,0,1,1-9.72-7,86.23,86.23,0,0,1,26.1-23.7c-9.65-21.59-29.26-34.77-36.41-39a101.52,101.52,0,0,0-24.92,21,6,6,0,1,1-9.16-7.75,110.31,110.31,0,0,1,31.4-25.47,6,6,0,0,1,5.36,0c1.33.67,30.79,15.69,44.49,45.93A85.68,85.68,0,0,1,208,58,6,6,0,0,1,214,64Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M53.92,34.62A8,8,0,1,0,42.08,45.38l9.73,10.71Q49.91,56,48,56a8,8,0,0,0-8,8v80a88.1,88.1,0,0,0,88,88h0a87.82,87.82,0,0,0,61.21-24.78l12.87,14.16a8,8,0,1,0,11.84-10.76ZM136.29,149A88.17,88.17,0,0,0,128,163.37a88.16,88.16,0,0,0-72-51V72.44a71.31,71.31,0,0,1,13.18,2.75ZM120,215.56A72.1,72.1,0,0,1,56,144V128.44A72.1,72.1,0,0,1,120,200Zm16,0V200a72.09,72.09,0,0,1,11.36-38.81l31.08,34.19A71.85,71.85,0,0,1,136,215.56ZM216,144a88.13,88.13,0,0,1-3.15,23.4,8,8,0,0,1-7.71,5.88A7.79,7.79,0,0,1,203,173a8,8,0,0,1-5.59-9.83A72.55,72.55,0,0,0,200,144V128.43a71.07,71.07,0,0,0-24.56,7.33,8,8,0,1,1-7.24-14.26,86.64,86.64,0,0,1,31.8-9.14V72.45a72.33,72.33,0,0,0-50.35,29.36,8,8,0,1,1-13-9.39,88.15,88.15,0,0,1,25.16-23.3C152.62,49.8,135.45,37.74,128,33.2A100.2,100.2,0,0,0,104.6,53.14,8,8,0,1,1,92.39,42.81a112.32,112.32,0,0,1,32-26,8,8,0,0,1,7.16,0c1.32.66,30.27,15.43,44.59,45.15A87.91,87.91,0,0,1,208,56a8,8,0,0,1,8,8Z"></path>
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
