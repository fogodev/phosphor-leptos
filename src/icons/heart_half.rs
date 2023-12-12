//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn HeartHalf(
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
                <path d="M170.21,52.72a12,12,0,0,0-4.42-23.59C150.5,32,137.49,39.2,128.08,49.66,116,36,98.14,28,78,28A66.08,66.08,0,0,0,12,94c0,31,18.91,63,56.2,94.87a342.69,342.69,0,0,0,54.11,37.7A12.1,12.1,0,0,0,128,228a11.92,11.92,0,0,0,5.68-1.46v0A339.75,339.75,0,0,0,175.32,199a12,12,0,0,0-14.64-19c-7.48,5.76-14.63,10.72-20.68,14.68V74.55C145.43,63.23,156.31,55.32,170.21,52.72ZM116,194.66a318.88,318.88,0,0,1-32.51-24.3C61.82,151.77,36,123.42,36,94A42,42,0,0,1,78,52c17,0,31.35,8.57,38,22.52ZM233,99.93a12,12,0,0,1-13.25-10.61,41.89,41.89,0,0,0-18.41-30.25,12,12,0,0,1,13.34-20A65.84,65.84,0,0,1,243.6,86.68,12,12,0,0,1,233,99.93Zm1,33.42c-5.84,11.72-14.33,23.63-25.23,35.43a12,12,0,0,1-17.62-16.29c9.36-10.13,16.55-20.17,21.38-29.84A12,12,0,1,1,234,133.35Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M128,72V216S24,160,24,94A54,54,0,0,1,78,40C100.59,40,119.94,52.31,128,72Z"
        opacity="0.2"
    ></path>
    <path d="M178,32c-20.65,0-38.73,8.88-50,23.89C116.73,40.88,98.65,32,78,32A62.07,62.07,0,0,0,16,94c0,70,103.79,126.67,108.21,129a8,8,0,0,0,7.58,0C136.21,220.67,240,164,240,94A62.07,62.07,0,0,0,178,32ZM32,94A46.06,46.06,0,0,1,78,48c18.91,0,34.86,9.79,42,25.65V202C93.59,185.44,32,141.78,32,94ZM136,202V73.65C143.14,57.79,159.09,48,178,48a46.06,46.06,0,0,1,46,46C224,141.71,162.42,185.41,136,202Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M178,32c-20.65,0-38.73,8.88-50,23.89C116.73,40.88,98.65,32,78,32A62.07,62.07,0,0,0,16,94c0,70,103.79,126.67,108.21,129a8,8,0,0,0,7.58,0C136.21,220.67,240,164,240,94A62.07,62.07,0,0,0,178,32ZM128,206.8V96a48,48,0,0,1,41.61-47.56A83.85,83.85,0,0,1,178,48a46.06,46.06,0,0,1,46,46C224,147.61,146.25,196.15,128,206.8Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M169.1,46.82A6,6,0,1,0,166.9,35C150.3,38.13,136.65,46.82,128,59.4,117.3,43.5,99,34,78,34A60.07,60.07,0,0,0,18,94c0,29.2,18.2,59.59,54.1,90.31a334.68,334.68,0,0,0,53.06,37,5.93,5.93,0,0,0,5.68,0h0a329.42,329.42,0,0,0,40.82-27,6,6,0,0,0-7.32-9.51A341.88,341.88,0,0,1,134,205.56V73.28C140,59.5,152.76,49.88,169.1,46.82ZM122,205.54C97.4,190.51,30,144.7,30,94A48.05,48.05,0,0,1,78,46c19.87,0,36.62,10.4,44,27.22ZM232.33,94a5,5,0,0,1-.67,0,6,6,0,0,1-5.95-5.34,47.89,47.89,0,0,0-21.05-34.58,6,6,0,1,1,6.68-10,59.85,59.85,0,0,1,26.29,43.23A6,6,0,0,1,232.33,94Zm-3.66,36.72c-5.58,11.2-13.75,22.65-24.26,34a6,6,0,0,1-8.82-8.15c9.75-10.54,17.27-21.05,22.35-31.24a6,6,0,1,1,10.73,5.36Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M169.47,48.79a8,8,0,0,0-2.94-15.73C150.42,36.08,137,44.18,128,56c-11.26-15-29.36-24-50-24A62.07,62.07,0,0,0,16,94c0,70,103.79,126.67,108.21,129a7.93,7.93,0,0,0,7.58,0h0a332.57,332.57,0,0,0,41.09-27.22,8,8,0,1,0-9.76-12.67c-10.31,7.94-20,14.37-27.12,18.82V73.7C141.84,60.75,153.94,51.7,169.47,48.79ZM120,202C93.58,185.41,32,141.71,32,94A46.06,46.06,0,0,1,78,48c18.91,0,34.86,9.78,42,25.64ZM232.55,96a8.85,8.85,0,0,1-.89,0,8,8,0,0,1-7.94-7.12,45.88,45.88,0,0,0-20.17-33.14,8,8,0,1,1,8.9-13.29,61.83,61.83,0,0,1,27.17,44.67A8,8,0,0,1,232.55,96Zm-2.09,35.62c-5.67,11.37-13.94,23-24.59,34.49a8,8,0,1,1-11.74-10.86c9.61-10.4,17-20.75,22-30.77a8,8,0,1,1,14.31,7.14Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M168.74,44.86A4,4,0,0,0,167.26,37C150.09,40.21,136.19,49.57,128,63.12,118,46.21,99.46,36,78,36A58.07,58.07,0,0,0,20,94c0,28.59,18,58.47,53.4,88.79a333.81,333.81,0,0,0,52.7,36.73,4,4,0,0,0,1.9.48,4,4,0,0,0,1.89-.49h0a328.8,328.8,0,0,0,40.54-26.87,4,4,0,1,0-4.88-6.33A343.7,343.7,0,0,1,132,209.09V72.85C138.24,58.26,151.58,48.07,168.74,44.86ZM124,209.08c-21.73-13-96-61.16-96-115.08A50.06,50.06,0,0,1,78,44c20.83,0,38.39,11,46,28.8ZM232.11,92l-.44,0a4,4,0,0,1-4-3.56,49.88,49.88,0,0,0-21.92-36,4,4,0,0,1,4.44-6.65,57.88,57.88,0,0,1,25.43,41.79A4,4,0,0,1,232.11,92Zm-5.23,37.81c-5.5,11-13.55,22.32-23.94,33.56a4,4,0,0,1-5.88-5.43c9.88-10.68,17.5-21.35,22.66-31.71a4,4,0,1,1,7.16,3.58Z"></path>
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
