//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "nature"))]
#[component]
pub fn FlowerLotus(
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
                <path d="M245.83,121.63a15.53,15.53,0,0,0-9.52-7.33,73.55,73.55,0,0,0-22.17-2.22c4-19.85,1-35.55-2-44.86a16.17,16.17,0,0,0-18.8-10.88,85.53,85.53,0,0,0-28.55,12.12,94.58,94.58,0,0,0-27.11-33.25,16.05,16.05,0,0,0-19.26,0A94.58,94.58,0,0,0,91.26,68.46,85.53,85.53,0,0,0,62.71,56.34,16.14,16.14,0,0,0,43.92,67.22c-3,9.31-6,25-2.06,44.86a73.55,73.55,0,0,0-22.17,2.22,15.53,15.53,0,0,0-9.52,7.33,16,16,0,0,0-1.6,12.26c3.39,12.58,13.8,36.49,45.33,55.33S113.13,208,128.05,208s42.67,0,74-18.78c31.53-18.84,41.94-42.75,45.33-55.33A16,16,0,0,0,245.83,121.63ZM62.1,175.49C35.47,159.57,26.82,140.05,24,129.7a59.61,59.61,0,0,1,22.5-1.17,129.08,129.08,0,0,0,9.15,19.41,142.28,142.28,0,0,0,34,39.56A114.92,114.92,0,0,1,62.1,175.49ZM128,190.4c-9.33-6.94-32-28.23-32-71.23C96,76.7,118.38,55.24,128,48c9.62,7.26,32,28.72,32,71.19C160,162.17,137.33,183.46,128,190.4Zm104-60.68c-2.77,10.24-11.4,29.81-38.09,45.77a114.92,114.92,0,0,1-27.55,12,142.28,142.28,0,0,0,34-39.56,129.08,129.08,0,0,0,9.15-19.41A59.69,59.69,0,0,1,232,129.71Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M128,200c-15.37,0-40.77-.18-70-17.64S19.31,143,16.28,131.82A7.94,7.94,0,0,1,21.74,122a69.59,69.59,0,0,1,30.82-.64h0c-7.63-23.88-4-42.44-1-51.71A8.2,8.2,0,0,1,61,64.18C68.56,65.79,81.28,70,94.6,80.8v0A108.49,108.49,0,0,0,88,119.18C88,178,128,200,128,200Zm106.26-78a69.59,69.59,0,0,0-30.82-.64h0c7.63-23.88,4-42.44,1-51.71A8.2,8.2,0,0,0,195,64.18c-7.52,1.61-20.24,5.8-33.56,16.62v0A108.49,108.49,0,0,1,168,119.18C168,178,128,200,128,200c15.37,0,40.77-.18,70-17.64s38.69-39.34,41.72-50.54A7.94,7.94,0,0,0,234.26,122Z"
        opacity="0.2"
    ></path>
    <path d="M245.83,121.63a15.53,15.53,0,0,0-9.52-7.33,73.51,73.51,0,0,0-22.17-2.22c4-19.85,1-35.55-2.06-44.86a16.15,16.15,0,0,0-18.79-10.88,85.53,85.53,0,0,0-28.55,12.12,94.58,94.58,0,0,0-27.11-33.25,16.05,16.05,0,0,0-19.26,0A94.48,94.48,0,0,0,91.26,68.46,85.53,85.53,0,0,0,62.71,56.34,16.15,16.15,0,0,0,43.92,67.22c-3,9.31-6,25-2.06,44.86a73.51,73.51,0,0,0-22.17,2.22,15.53,15.53,0,0,0-9.52,7.33,16,16,0,0,0-1.6,12.27c3.39,12.57,13.8,36.48,45.33,55.32S113.13,208,128.05,208s42.67,0,74-18.78c31.53-18.84,41.94-42.75,45.33-55.32A16,16,0,0,0,245.83,121.63ZM59.14,72.14a.2.2,0,0,1,.23-.15A70.43,70.43,0,0,1,85.18,83.66,118.65,118.65,0,0,0,80,119.17c0,18.74,3.77,34,9.11,46.28A123.59,123.59,0,0,1,69.57,140C51.55,108.62,55.3,84,59.14,72.14Zm3,103.35C35.47,159.57,26.82,140.05,24,129.7a59.82,59.82,0,0,1,22.5-1.17,129.08,129.08,0,0,0,9.15,19.41,142.28,142.28,0,0,0,34,39.56A114.92,114.92,0,0,1,62.1,175.49ZM128,190.4c-9.33-6.94-32-28.23-32-71.23C96,76.7,118.38,55.24,128,48c9.62,7.26,32,28.72,32,71.19C160,162.17,137.33,183.46,128,190.4ZM170.82,83.66A70.43,70.43,0,0,1,196.63,72a.2.2,0,0,1,.23.15C200.7,84,204.45,108.62,186.43,140a123.32,123.32,0,0,1-19.54,25.48c5.34-12.26,9.11-27.54,9.11-46.28A118.65,118.65,0,0,0,170.82,83.66ZM232,129.72c-2.77,10.25-11.4,29.81-38.09,45.77a114.92,114.92,0,0,1-27.55,12,142.28,142.28,0,0,0,34-39.56,129.08,129.08,0,0,0,9.15-19.41A59.69,59.69,0,0,1,232,129.71Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M242.37,123.65a11.54,11.54,0,0,0-7.08-5.46A71.5,71.5,0,0,0,209,116.52c5.48-21.46,2.38-38.36-.75-48a12.16,12.16,0,0,0-14.16-8.19,82.85,82.85,0,0,0-31,14.17,91.06,91.06,0,0,0-27.9-36,11.91,11.91,0,0,0-14.44,0,91.06,91.06,0,0,0-27.9,36,82.92,82.92,0,0,0-31-14.17,12.16,12.16,0,0,0-14.16,8.19c-3.13,9.68-6.23,26.58-.75,48a71.5,71.5,0,0,0-26.26,1.67,11.54,11.54,0,0,0-7.08,5.46,12,12,0,0,0-1.2,9.22c3.24,12,13.2,34.81,43.52,52.92S113.45,204,128,204s41.61,0,72.07-18.21,40.28-40.93,43.52-52.92A12,12,0,0,0,242.37,123.65ZM195.8,68.11A4.2,4.2,0,0,1,200.67,71c4,12.5,8,38.35-10.77,71-10.45,18.19-25.11,32.2-38.11,41.44C162,170,172,149.24,172,119.19a113.35,113.35,0,0,0-5.88-37A75.28,75.28,0,0,1,195.8,68.11ZM55.33,71a4.19,4.19,0,0,1,4.87-2.84A75.28,75.28,0,0,1,89.88,82.19a113.35,113.35,0,0,0-5.88,37c0,30.05,10,50.82,20.21,64.23-13-9.24-27.66-23.25-38.11-41.44C47.32,109.3,51.29,83.45,55.33,71Zm4.72,108c-27.9-16.67-37-37.32-39.9-48.15a4,4,0,0,1,.41-3.13,3.59,3.59,0,0,1,2.21-1.73,64.62,64.62,0,0,1,26.73-1,123.48,123.48,0,0,0,9.66,21c13.28,23.1,32.66,39.67,48.27,49.11A116.34,116.34,0,0,1,60.05,178.93Zm68,16.34a75.75,75.75,0,0,1-17.08-16.4C98.37,162.58,92,142.5,92,119.19c0-44.25,23.49-66.75,33.59-74.36a4,4,0,0,1,4.82,0C140.51,52.44,164,74.94,164,119.19c0,23.31-6.37,43.39-18.92,59.68A75.75,75.75,0,0,1,128,195.27Zm107.85-64.49c-2.92,10.83-12,31.48-39.9,48.15a116.34,116.34,0,0,1-47.38,16.15c15.61-9.44,35-26,48.27-49.11a123.48,123.48,0,0,0,9.66-21,64.45,64.45,0,0,1,26.73,1,3.59,3.59,0,0,1,2.21,1.73A4,4,0,0,1,235.85,130.78Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M249.3,119.63a19.51,19.51,0,0,0-12-9.2A76,76,0,0,0,218.92,108a92.19,92.19,0,0,0-3-42,20.15,20.15,0,0,0-23.44-13.57A88.39,88.39,0,0,0,166.3,62.8,98,98,0,0,0,140,32,20.07,20.07,0,0,0,116,32,98,98,0,0,0,89.7,62.8,88.39,88.39,0,0,0,63.55,52.42,20.15,20.15,0,0,0,40.11,66a92.19,92.19,0,0,0-3,42,76,76,0,0,0-18.41,2.43,19.51,19.51,0,0,0-12,9.2,20,20,0,0,0-2,15.31c3.55,13.16,14.4,38.16,47.14,57.72C84.16,212,112.76,212,128.08,212s43.76,0,76.07-19.34c32.74-19.56,43.59-44.56,47.14-57.72A20,20,0,0,0,249.3,119.63ZM64.16,172.05c-22.29-13.32-31.35-29.13-35-39.35a57.54,57.54,0,0,1,14.54-.46,134,134,0,0,0,8.55,17.69,142.54,142.54,0,0,0,19.59,26.32C69.3,175,66.74,173.6,64.16,172.05ZM79,147.27c-2.1-3-4.12-6-6-9.29C57.37,110.7,58.84,88.94,62,76.76A69,69,0,0,1,80.5,85.33,124.18,124.18,0,0,0,76,119.17,127.59,127.59,0,0,0,79,147.27Zm49,38c-9.72-8.07-28-28.28-28-66.13,0-37.42,18-57.7,28-66.08,10,8.38,28,28.66,28,66.08C156,157,137.72,177.23,128,185.3Zm49-38a127.59,127.59,0,0,0,3-28.1,124.18,124.18,0,0,0-4.5-33.84A69.08,69.08,0,0,1,194,76.75c3.13,12.19,4.6,34-11.06,61.23C181.1,141.23,179.08,144.32,177,147.27Zm14.86,24.78c-2.56,1.53-5.09,2.91-7.58,4.16a142.54,142.54,0,0,0,19.51-26.28,134,134,0,0,0,8.55-17.69,57.54,57.54,0,0,1,14.54.46C223.19,142.92,214.13,158.73,191.84,172.05Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M244.1,122.63a13.56,13.56,0,0,0-8.3-6.4,72.62,72.62,0,0,0-24.17-2c4.67-20.63,1.62-36.91-1.45-46.4A14.16,14.16,0,0,0,193.7,58.3a84.21,84.21,0,0,0-29.76,13.11,92.6,92.6,0,0,0-27.52-34.6,14,14,0,0,0-16.85,0,92.7,92.7,0,0,0-27.51,34.6A84.16,84.16,0,0,0,62.29,58.3a14.15,14.15,0,0,0-16.47,9.54c-3.07,9.49-6.12,25.77-1.45,46.4a72.62,72.62,0,0,0-24.17,2,13.56,13.56,0,0,0-8.3,6.4,14,14,0,0,0-1.4,10.74C13.81,145.66,24,169,54.92,187.51S113.29,206,128,206s42.12,0,73.06-18.49,41.11-41.85,44.42-54.14A14,14,0,0,0,244.1,122.63ZM168.48,82.9A73.1,73.1,0,0,1,196.22,70a2.2,2.2,0,0,1,2.54,1.5C202.7,83.72,206.57,109,188.17,141a129.75,129.75,0,0,1-28,33.37C167.85,161,174,142.93,174,119.17A116.13,116.13,0,0,0,168.48,82.9ZM57.24,71.53A2.2,2.2,0,0,1,59.78,70,73.1,73.1,0,0,1,87.52,82.9,116.13,116.13,0,0,0,82,119.17c0,23.76,6.15,41.85,13.81,55.17a129.58,129.58,0,0,1-28-33.37C49.43,109,53.3,83.72,57.24,71.53ZM61.08,177.2c-27.3-16.31-36.15-36.42-39-47a2.08,2.08,0,0,1,.21-1.61,1.71,1.71,0,0,1,1-.8A62.16,62.16,0,0,1,48,126.72,126.25,126.25,0,0,0,57.43,147a141,141,0,0,0,41,44.72A114.83,114.83,0,0,1,61.08,177.2ZM128,192.86c-8.68-6.2-34-28.2-34-73.69,0-43.36,22.94-65.34,32.8-72.78a2,2,0,0,1,2.4,0c9.86,7.44,32.8,29.42,32.8,72.78C162,164.94,136.81,186.67,128,192.86Zm105.9-62.62c-2.85,10.54-11.7,30.65-39,47a114.83,114.83,0,0,1-37.38,14.47,141,141,0,0,0,41-44.72A126.25,126.25,0,0,0,208,126.72a62.16,62.16,0,0,1,24.73,1.11,1.71,1.71,0,0,1,1,.8A2.08,2.08,0,0,1,233.92,130.24Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M245.83,121.63a15.53,15.53,0,0,0-9.52-7.33,73.51,73.51,0,0,0-22.17-2.22c4-19.85,1-35.55-2.06-44.86a16.15,16.15,0,0,0-18.79-10.88,85.53,85.53,0,0,0-28.55,12.12,94.58,94.58,0,0,0-27.11-33.25,16.05,16.05,0,0,0-19.26,0A94.48,94.48,0,0,0,91.26,68.46,85.53,85.53,0,0,0,62.71,56.34,16.15,16.15,0,0,0,43.92,67.22c-3,9.31-6,25-2.06,44.86a73.51,73.51,0,0,0-22.17,2.22,15.53,15.53,0,0,0-9.52,7.33,16,16,0,0,0-1.6,12.27c3.39,12.57,13.8,36.48,45.33,55.32S113.13,208,128.05,208s42.67,0,74-18.78c31.53-18.84,41.94-42.75,45.33-55.32A16,16,0,0,0,245.83,121.63ZM59.14,72.14a.2.2,0,0,1,.23-.15A70.43,70.43,0,0,1,85.18,83.66,118.65,118.65,0,0,0,80,119.17c0,18.74,3.77,34,9.11,46.28A123.59,123.59,0,0,1,69.57,140C51.55,108.62,55.3,84,59.14,72.14Zm3,103.35C35.47,159.57,26.82,140.05,24,129.7a59.82,59.82,0,0,1,22.5-1.17,129.08,129.08,0,0,0,9.15,19.41,142.28,142.28,0,0,0,34,39.56A114.92,114.92,0,0,1,62.1,175.49ZM128,190.4c-9.33-6.94-32-28.23-32-71.23C96,76.7,118.38,55.24,128,48c9.62,7.26,32,28.72,32,71.19C160,162.17,137.33,183.46,128,190.4ZM170.82,83.66A70.43,70.43,0,0,1,196.63,72a.2.2,0,0,1,.23.15C200.7,84,204.45,108.62,186.43,140a123.32,123.32,0,0,1-19.54,25.48c5.34-12.26,9.11-27.54,9.11-46.28A118.65,118.65,0,0,0,170.82,83.66ZM232,129.72c-2.77,10.25-11.4,29.81-38.09,45.77a114.92,114.92,0,0,1-27.55,12,142.28,142.28,0,0,0,34-39.56,129.08,129.08,0,0,0,9.15-19.41A59.69,59.69,0,0,1,232,129.71Z"></path>
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
