//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "design"))]
#[component]
pub fn TriangleDashed(
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
                <path d="M236.8,188.09,149.35,36.22a24.76,24.76,0,0,0-42.7,0L19.2,188.09a23.51,23.51,0,0,0,0,23.72A24.34,24.34,0,0,0,40.55,224h174.9a24.34,24.34,0,0,0,21.33-12.19A23.51,23.51,0,0,0,236.8,188.09ZM108,200H60.79A12,12,0,0,1,50.4,182l24.18-42a8,8,0,0,1,13.87,8L67.71,184H108a8,8,0,0,1,0,16Zm-1.12-84A8,8,0,0,1,93,108l24.59-42.7a12,12,0,0,1,20.8,0L163,108a8,8,0,0,1-13.87,8L128,79.31Zm98.72,78a12.05,12.05,0,0,1-10.39,6H148a8,8,0,0,1,0-16h40.29l-20.74-36a8,8,0,0,1,13.87-8l24.18,42A12,12,0,0,1,205.6,194Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M215.46,216H40.54C27.92,216,20,202.79,26.13,192.09L113.59,40.22c6.3-11,22.52-11,28.82,0l87.46,151.87C236,202.79,228.08,216,215.46,216Z"
        opacity="0.2"
    ></path>
    <path d="M160,216a8,8,0,0,1-8,8H104a8,8,0,0,1,0-16h48A8,8,0,0,1,160,216Zm76.8-27.91L232.14,180a8,8,0,0,0-13.86,8l4.65,8.09a7.59,7.59,0,0,1,0,7.72,8.5,8.5,0,0,1-7.48,4.2H192a8,8,0,0,0,0,16h23.45a24.34,24.34,0,0,0,21.33-12.19A23.51,23.51,0,0,0,236.8,188.09ZM64,208H40.55a8.5,8.5,0,0,1-7.48-4.2,7.59,7.59,0,0,1,0-7.72L37.72,188a8,8,0,1,0-13.86-8l-4.66,8.08a23.51,23.51,0,0,0,0,23.72A24.34,24.34,0,0,0,40.55,224H64a8,8,0,0,0,0-16Zm138.18-56a8,8,0,0,0,6.93-12l-23-40a8,8,0,0,0-13.86,8l23,40A8,8,0,0,0,202.18,152ZM149.35,36.22a24.76,24.76,0,0,0-42.7,0L93,60a8,8,0,1,0,13.86,8l13.7-23.78a8.75,8.75,0,0,1,15,0L149.18,68a8,8,0,0,0,6.94,4,7.91,7.91,0,0,0,4-1.07A8,8,0,0,0,163,60ZM80.85,97.07A8,8,0,0,0,69.93,100l-23,40a8,8,0,0,0,13.87,8l23-40A8,8,0,0,0,80.85,97.07Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M156,216a4,4,0,0,1-4,4H104a4,4,0,0,1,0-8h48A4,4,0,0,1,156,216Zm77.34-25.91L228.68,182a4,4,0,1,0-6.93,4l4.65,8.08a11.45,11.45,0,0,1,0,11.72,12.47,12.47,0,0,1-10.94,6.2H192a4,4,0,0,0,0,8h23.46a20.36,20.36,0,0,0,17.86-10.2A19.52,19.52,0,0,0,233.34,190.09ZM64,212H40.54a12.47,12.47,0,0,1-10.94-6.2,11.45,11.45,0,0,1,0-11.72L34.25,186a4,4,0,1,0-6.93-4l-4.66,8.09a19.52,19.52,0,0,0,0,19.71A20.36,20.36,0,0,0,40.54,220H64a4,4,0,0,0,0-8Zm138.18-64a4,4,0,0,0,3.47-6l-23-40a4,4,0,1,0-6.93,4l23,40A4,4,0,0,0,202.18,148ZM145.88,38.22a20.75,20.75,0,0,0-35.76,0L96.42,62a4,4,0,1,0,6.94,4l13.69-23.79a12.76,12.76,0,0,1,21.9,0L152.64,66a4,4,0,0,0,6.94-4Zm-67,62.31A4,4,0,0,0,73.39,102l-23,40a4,4,0,0,0,6.94,4l23-40A4,4,0,0,0,78.85,100.53Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M164,216a12,12,0,0,1-12,12H104a12,12,0,0,1,0-24h48A12,12,0,0,1,164,216Zm76.26-29.9L235.6,178a12,12,0,1,0-20.79,12l4.65,8.08a3.56,3.56,0,0,1,0,3.73,4.46,4.46,0,0,1-4,2.2H192a12,12,0,0,0,0,24h23.46a28.31,28.31,0,0,0,24.79-14.19A27.47,27.47,0,0,0,240.26,186.1ZM64,204H40.55a4.46,4.46,0,0,1-4-2.2,3.56,3.56,0,0,1,0-3.73L41.19,190A12,12,0,1,0,20.4,178l-4.66,8.09a27.47,27.47,0,0,0,0,27.71A28.31,28.31,0,0,0,40.55,228H64a12,12,0,0,0,0-24Zm138.17-48a12,12,0,0,0,10.39-18l-23-40a12,12,0,0,0-20.8,12l23,40A12,12,0,0,0,202.18,156ZM166.5,58,152.81,34.23a28.74,28.74,0,0,0-49.62,0L89.5,58a12,12,0,1,0,20.79,12L124,46.2a4.75,4.75,0,0,1,8,0L145.71,70a12,12,0,0,0,10.41,6,11.87,11.87,0,0,0,6-1.6A12,12,0,0,0,166.5,58ZM82.85,93.6A12,12,0,0,0,66.46,98l-23,40a12,12,0,0,0,20.8,12l23-40A12,12,0,0,0,82.85,93.6Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M158,216a6,6,0,0,1-6,6H104a6,6,0,0,1,0-12h48A6,6,0,0,1,158,216Zm77.07-26.91L230.41,181a6,6,0,1,0-10.4,6l4.66,8.09a9.49,9.49,0,0,1,0,9.72,10.46,10.46,0,0,1-9.21,5.2H192a6,6,0,0,0,0,12h23.45a22.37,22.37,0,0,0,19.6-11.19A21.53,21.53,0,0,0,235.07,189.09ZM64,210H40.55a10.46,10.46,0,0,1-9.21-5.2,9.49,9.49,0,0,1,0-9.72L36,187a6,6,0,0,0-10.4-6l-4.66,8.08a21.53,21.53,0,0,0,0,21.72A22.37,22.37,0,0,0,40.55,222H64a6,6,0,0,0,0-12Zm138.18-60a6,6,0,0,0,5.2-9l-23-40a6,6,0,1,0-10.4,6l23,40A6,6,0,0,0,202.18,150ZM147.61,37.22a22.75,22.75,0,0,0-39.22,0L94.69,61a6,6,0,1,0,10.4,6l13.7-23.78a10.75,10.75,0,0,1,18.42,0L150.91,67a6,6,0,0,0,10.4-6ZM79.85,98.8A6,6,0,0,0,71.66,101l-23,40A6,6,0,1,0,59,147l23-40A6,6,0,0,0,79.85,98.8Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M160,216a8,8,0,0,1-8,8H104a8,8,0,0,1,0-16h48A8,8,0,0,1,160,216Zm76.8-27.91L232.14,180a8,8,0,0,0-13.86,8l4.65,8.09a7.59,7.59,0,0,1,0,7.72,8.5,8.5,0,0,1-7.48,4.2H192a8,8,0,0,0,0,16h23.45a24.34,24.34,0,0,0,21.33-12.19A23.51,23.51,0,0,0,236.8,188.09ZM64,208H40.55a8.5,8.5,0,0,1-7.48-4.2,7.59,7.59,0,0,1,0-7.72L37.72,188a8,8,0,1,0-13.86-8l-4.66,8.08a23.51,23.51,0,0,0,0,23.72A24.34,24.34,0,0,0,40.55,224H64a8,8,0,0,0,0-16Zm138.18-56a8,8,0,0,0,6.93-12l-23-40a8,8,0,0,0-13.86,8l23,40A8,8,0,0,0,202.18,152ZM149.35,36.22a24.76,24.76,0,0,0-42.7,0L93,60a8,8,0,1,0,13.86,8l13.7-23.78a8.75,8.75,0,0,1,15,0L149.18,68a8,8,0,0,0,6.94,4,7.91,7.91,0,0,0,4-1.07A8,8,0,0,0,163,60ZM80.85,97.07A8,8,0,0,0,69.93,100l-23,40a8,8,0,0,0,13.87,8l23-40A8,8,0,0,0,80.85,97.07Z"></path>
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
