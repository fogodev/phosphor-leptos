//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn BellSimpleRinging(
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
                <path d="M172,228a12,12,0,0,1-12,12H96a12,12,0,0,1,0-24h64A12,12,0,0,1,172,228ZM230.94,58.48A115.25,115.25,0,0,0,190.4,13.86a12,12,0,1,0-12.8,20.29,90.1,90.1,0,0,1,32,35.38A12,12,0,0,0,220.3,76a11.86,11.86,0,0,0,5.51-1.35A12,12,0,0,0,230.94,58.48ZM46.37,69.53a90.1,90.1,0,0,1,32-35.38A12,12,0,1,0,65.6,13.86,115.25,115.25,0,0,0,25.06,58.48a12,12,0,0,0,5.13,16.17A11.86,11.86,0,0,0,35.7,76,12,12,0,0,0,46.37,69.53Zm173.51,98.35A20,20,0,0,1,204,200H52a20,20,0,0,1-15.91-32.12c7.17-9.33,15.73-26.62,15.88-55.94A76,76,0,0,1,204,112C204.15,141.26,212.71,158.55,219.88,167.88ZM196.34,176c-8.16-13-16.19-33.57-16.34-63.94A52,52,0,1,0,76,112c-.15,30.42-8.18,51-16.34,64Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M208,192H48a8,8,0,0,1-6.88-12C47.71,168.6,56,147.81,56,112a72,72,0,0,1,144,0c0,35.82,8.3,56.6,14.9,68A8,8,0,0,1,208,192Z"
        opacity="0.2"
    ></path>
    <path d="M168,224a8,8,0,0,1-8,8H96a8,8,0,0,1,0-16h64A8,8,0,0,1,168,224ZM227.39,60.32a111.36,111.36,0,0,0-39.12-43.08,8,8,0,1,0-8.54,13.53,94.13,94.13,0,0,1,33.46,36.91,8,8,0,0,0,14.2-7.36ZM35.71,72a8,8,0,0,0,7.1-4.32A94.13,94.13,0,0,1,76.27,30.77a8,8,0,1,0-8.54-13.53A111.36,111.36,0,0,0,28.61,60.32,8,8,0,0,0,35.71,72Zm186.1,103.94A16,16,0,0,1,208,200H48a16,16,0,0,1-13.79-24.06C43.22,160.39,48,138.28,48,112a80,80,0,0,1,160,0C208,138.27,212.78,160.38,221.81,175.94ZM208,184c-10.64-18.27-16-42.49-16-72a64,64,0,0,0-128,0c0,29.52-5.38,53.74-16,72Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M168,224a8,8,0,0,1-8,8H96a8,8,0,0,1,0-16h64A8,8,0,0,1,168,224ZM227.39,60.32a111.36,111.36,0,0,0-39.12-43.08,8,8,0,1,0-8.54,13.53,94.13,94.13,0,0,1,33.46,36.91,8,8,0,0,0,14.2-7.36ZM35.71,72a8,8,0,0,0,7.1-4.32A94.13,94.13,0,0,1,76.27,30.77a8,8,0,1,0-8.54-13.53A111.36,111.36,0,0,0,28.61,60.32,8,8,0,0,0,35.71,72ZM208,112a80,80,0,0,0-160,0c0,26.28-4.78,48.39-13.81,63.94A16,16,0,0,0,48,200H208a16,16,0,0,0,13.79-24.06C212.78,160.38,208,138.27,208,112Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M165.92,224a6,6,0,0,1-6,6h-64a6,6,0,0,1,0-12h64A6,6,0,0,1,165.92,224ZM225.61,61.23a109.23,109.23,0,0,0-38.41-42.3,6,6,0,0,0-6.4,10.14A96,96,0,0,1,215,66.76a6,6,0,1,0,10.65-5.53ZM41,66.76A96,96,0,0,1,75.2,29.07a6,6,0,0,0-6.4-10.14,109.23,109.23,0,0,0-38.41,42.3A6,6,0,1,0,41,66.76Zm179,110.17A14,14,0,0,1,208,198H48a14,14,0,0,1-12.06-21C45.13,161.08,50,138.62,50,112a78,78,0,0,1,156,0C206,139,210.74,160.84,220.08,176.93Zm-10.37,6C199.29,165,194,141.14,194,112a66,66,0,0,0-132,0c0,29.16-5.29,53-15.71,71a2,2,0,0,0,0,2,1.9,1.9,0,0,0,1.7,1H208a1.9,1.9,0,0,0,1.7-1A2,2,0,0,0,209.71,183Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M168,224a8,8,0,0,1-8,8H96a8,8,0,0,1,0-16h64A8,8,0,0,1,168,224ZM227.39,60.32a111.36,111.36,0,0,0-39.12-43.08,8,8,0,1,0-8.54,13.53,94.13,94.13,0,0,1,33.46,36.91,8,8,0,0,0,14.2-7.36ZM35.71,72a8,8,0,0,0,7.1-4.32A94.13,94.13,0,0,1,76.27,30.77a8,8,0,1,0-8.54-13.53A111.36,111.36,0,0,0,28.61,60.32,8,8,0,0,0,35.71,72Zm186.1,103.94A16,16,0,0,1,208,200H48a16,16,0,0,1-13.79-24.06C43.22,160.39,48,138.28,48,112a80,80,0,0,1,160,0C208,138.27,212.78,160.38,221.81,175.94ZM208,184c-10.64-18.27-16-42.49-16-72a64,64,0,0,0-128,0c0,29.52-5.38,53.74-16,72Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M164,224a4,4,0,0,1-4,4H96a4,4,0,0,1,0-8h64A4,4,0,0,1,164,224ZM223.84,62.16a107.34,107.34,0,0,0-37.71-41.54,4,4,0,1,0-4.26,6.76,99.41,99.41,0,0,1,34.87,38.46A4,4,0,0,0,220.29,68a3.94,3.94,0,0,0,1.84-.45A4,4,0,0,0,223.84,62.16ZM39.26,65.84A99.41,99.41,0,0,1,74.13,27.38a4,4,0,0,0-4.26-6.76A107.34,107.34,0,0,0,32.16,62.16a4,4,0,0,0,1.71,5.39,3.94,3.94,0,0,0,1.84.45A4,4,0,0,0,39.26,65.84ZM218.36,178A12,12,0,0,1,208,196H48A12,12,0,0,1,37.64,178C47.17,161.56,52,139.37,52,112a76,76,0,0,1,152,0C204,139.36,208.83,161.55,218.36,178Zm-6.92,4C201.19,164.34,196,140.79,196,112a68,68,0,0,0-136,0c0,28.8-5.19,52.34-15.44,70a4,4,0,0,0,0,4A3.89,3.89,0,0,0,48,188H208a3.89,3.89,0,0,0,3.43-2A4,4,0,0,0,211.44,182Z"></path>
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
