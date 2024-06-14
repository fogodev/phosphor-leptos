//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "games", feature = "health", feature = "objects"))]
#[component]
pub fn BeachBall(
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
                <path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm83.44,76A195.88,195.88,0,0,0,165,91,195.88,195.88,0,0,0,156,44.56,88.43,88.43,0,0,1,211.44,100ZM85,51.24a188.27,188.27,0,0,1,67.3,39.21A196.29,196.29,0,0,0,40.08,124.51,88.07,88.07,0,0,1,85,51.24Zm46.48,164.68a196.29,196.29,0,0,0,34.06-112.23A188.27,188.27,0,0,1,204.76,171,88.07,88.07,0,0,1,131.49,215.92Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M161.23,93.36a191.07,191.07,0,0,0-82-48.09,96.4,96.4,0,0,1,68.68-11.2A191.91,191.91,0,0,1,161.23,93.36Zm60.7,14.7a192.2,192.2,0,0,0-59.29-13.29,191,191,0,0,1,48.09,82,96.37,96.37,0,0,0,11.2-68.67ZM32.27,135.19a96,96,0,0,0,88.54,88.54,191.56,191.56,0,0,0,40.5-129A191.61,191.61,0,0,0,32.27,135.19Z"
        opacity="0.2"
    ></path>
    <path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm81.7,71.3a199.77,199.77,0,0,0-40.94-8.06A199.77,199.77,0,0,0,160.7,46.3,88.57,88.57,0,0,1,209.7,95.3ZM216,128a87.83,87.83,0,0,1-4.28,27.12,200.28,200.28,0,0,0-29.16-49.93,183.12,183.12,0,0,1,32.31,8.75A88.14,88.14,0,0,1,216,128ZM142.06,41.13a183.12,183.12,0,0,1,8.75,32.31,200.28,200.28,0,0,0-49.93-29.16,88.05,88.05,0,0,1,41.18-3.15ZM80.44,54a183.88,183.88,0,0,1,61.25,32.64A200.21,200.21,0,0,0,40.41,119.5,88.11,88.11,0,0,1,80.44,54ZM40.67,138.86a184.08,184.08,0,0,1,112.88-36.41,184.08,184.08,0,0,1-36.41,112.88A88.18,88.18,0,0,1,40.67,138.86Zm95.83,76.73a200.21,200.21,0,0,0,32.87-101.28A183.88,183.88,0,0,1,202,175.56,88.11,88.11,0,0,1,136.5,215.59Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M128,28A100,100,0,1,0,228,128,100.11,100.11,0,0,0,128,28Zm88.14,73.64A195.13,195.13,0,0,0,165,91a195.13,195.13,0,0,0-10.68-51.1A92.33,92.33,0,0,1,216.14,101.64ZM145,37.58A187.54,187.54,0,0,1,156.3,83.26,196.37,196.37,0,0,0,89.67,44.38,91.34,91.34,0,0,1,128,36,92.41,92.41,0,0,1,145,37.58ZM79.86,49.63a188.08,188.08,0,0,1,72.45,40.82A196.19,196.19,0,0,0,36,127.39,92,92,0,0,1,79.86,49.63ZM36.44,137a188.17,188.17,0,0,1,121-38.48,188.17,188.17,0,0,1-38.48,121A92.17,92.17,0,0,1,36.44,137Zm92.17,83a196.19,196.19,0,0,0,36.94-116.3,188.08,188.08,0,0,1,40.82,72.45A92,92,0,0,1,128.61,220Zm83-53.66A196.37,196.37,0,0,0,172.74,99.7,187.54,187.54,0,0,1,218.42,111,92.41,92.41,0,0,1,220,128,91.34,91.34,0,0,1,211.62,166.33Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M128,20A108,108,0,1,0,236,128,108.12,108.12,0,0,0,128,20Zm84,108a84.26,84.26,0,0,1-1.32,14.74,204.79,204.79,0,0,0-18.52-31.49,176.78,176.78,0,0,1,19.08,5.62A85,85,0,0,1,212,128Zm-9.61-38.94a204.25,204.25,0,0,0-30-5.47,204.25,204.25,0,0,0-5.47-30A84.53,84.53,0,0,1,202.39,89.06Zm-63.26-44.3a176.78,176.78,0,0,1,5.62,19.08,204.79,204.79,0,0,0-31.49-18.52,82.49,82.49,0,0,1,25.87-.56ZM81.07,58.36a179.84,179.84,0,0,1,49.8,25.06,204.19,204.19,0,0,0-85.23,28.05A84.23,84.23,0,0,1,81.07,58.36ZM45,140.65a180.19,180.19,0,0,1,104.62-34.22,180.19,180.19,0,0,1-34.22,104.62A84.22,84.22,0,0,1,45,140.65Zm99.58,69.71a204.19,204.19,0,0,0,28.05-85.23,179.84,179.84,0,0,1,25.06,49.8A84.23,84.23,0,0,1,144.53,210.36Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M128,26A102,102,0,1,0,230,128,102.12,102.12,0,0,0,128,26Zm85,72.45a197.9,197.9,0,0,0-46.09-9.37A197.9,197.9,0,0,0,157.55,43,90.47,90.47,0,0,1,213,98.45ZM143.52,39.34a186.57,186.57,0,0,1,10.11,39A198.45,198.45,0,0,0,95.15,44.21a90.17,90.17,0,0,1,48.37-4.87ZM80.15,51.81A186.09,186.09,0,0,1,147,88.47a198.32,198.32,0,0,0-108.92,35A90.05,90.05,0,0,1,80.15,51.81ZM38.55,138a186.16,186.16,0,0,1,117-37.46,186.16,186.16,0,0,1-37.46,117A90.18,90.18,0,0,1,38.55,138Zm94,79.94a198.33,198.33,0,0,0,35-108.93,186.21,186.21,0,0,1,36.66,66.89A90.05,90.05,0,0,1,132.55,217.89Zm79.24-57a198.45,198.45,0,0,0-34.11-58.48,186.57,186.57,0,0,1,39,10.11,90.17,90.17,0,0,1-4.87,48.37Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm81.7,71.3a199.77,199.77,0,0,0-40.94-8.06A199.77,199.77,0,0,0,160.7,46.3,88.57,88.57,0,0,1,209.7,95.3ZM216,128a87.83,87.83,0,0,1-4.28,27.12,200.28,200.28,0,0,0-29.16-49.93,183.12,183.12,0,0,1,32.31,8.75A88.14,88.14,0,0,1,216,128ZM142.06,41.13a183.12,183.12,0,0,1,8.75,32.31,200.28,200.28,0,0,0-49.93-29.16,88.05,88.05,0,0,1,41.18-3.15ZM80.44,54a183.88,183.88,0,0,1,61.25,32.64A200.21,200.21,0,0,0,40.41,119.5,88.11,88.11,0,0,1,80.44,54ZM40.67,138.86a184.08,184.08,0,0,1,112.88-36.41,184.08,184.08,0,0,1-36.41,112.88A88.18,88.18,0,0,1,40.67,138.86Zm95.83,76.73a200.21,200.21,0,0,0,32.87-101.28A183.88,183.88,0,0,1,202,175.56,88.11,88.11,0,0,1,136.5,215.59Z"></path>
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
