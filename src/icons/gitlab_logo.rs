//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[component]
pub fn GitlabLogo(
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
                <path d="M234,116.09,214.13,40a15.94,15.94,0,0,0-30.42-1.48L167,84H89L72.29,38.49A15.94,15.94,0,0,0,41.87,40L22,116.09a61.19,61.19,0,0,0,23.57,65.23l73.27,51.77a15.93,15.93,0,0,0,18.36,0l73.27-51.77A61.19,61.19,0,0,0,234,116.09ZM58.61,70.86l10.76,29.28A12,12,0,0,0,80.64,108h94.72a12,12,0,0,0,11.27-7.86l10.76-29.28,9.14,35L128,161.31,49.47,105.83ZM44,131.37,107.2,176l-13.79,9.74-34-24A36.86,36.86,0,0,1,44,131.37Zm84,78.82-13.79-9.75L128,190.7l13.79,9.74Zm68.6-48.47-34,24L148.8,176,212,131.37A36.86,36.86,0,0,1,196.6,161.72Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M220.23,110.84,128,176,35.77,110.84,53.5,43A3.93,3.93,0,0,1,61,42.62L80.65,96h94.7L195,42.62a3.93,3.93,0,0,1,7.53.38Z"
        opacity="0.2"
    ></path>
    <path d="M230.15,117.1,210.25,41a11.94,11.94,0,0,0-22.79-1.11L169.78,88H86.22L68.54,39.87A11.94,11.94,0,0,0,45.75,41L25.85,117.1a57.19,57.19,0,0,0,22,61l73.27,51.76a11.91,11.91,0,0,0,13.74,0l73.27-51.76A57.19,57.19,0,0,0,230.15,117.1ZM58,57.5,73.13,98.76A8,8,0,0,0,80.64,104h94.72a8,8,0,0,0,7.51-5.24L198,57.5l13.07,50L128,166.21,44.9,107.5ZM40.68,124.11,114.13,176,93.41,190.65,57.09,165A41.06,41.06,0,0,1,40.68,124.11Zm87.32,91-20.73-14.65L128,185.8l20.73,14.64ZM198.91,165l-36.32,25.66L141.87,176l73.45-51.9A41.06,41.06,0,0,1,198.91,165Z"></path>
}.into_view(),
IconWeight::Fill => view! {
    <path d="M230.15,117.1,210.25,41a11.94,11.94,0,0,0-22.79-1.11L169.78,88H86.22L68.54,39.87A11.94,11.94,0,0,0,45.75,41L25.85,117.1a57.19,57.19,0,0,0,22,61l73.27,51.76a11.91,11.91,0,0,0,13.74,0l73.27-51.76A57.19,57.19,0,0,0,230.15,117.1Zm-189.47,7L114.13,176,93.41,190.65,57.09,165A41.06,41.06,0,0,1,40.68,124.11Zm87.32,91-20.73-14.65L128,185.8l20.73,14.64ZM198.91,165l-36.32,25.66L141.87,176l73.45-51.9A41.06,41.06,0,0,1,198.91,165Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M228.21,117.61,208.32,41.49a9.94,9.94,0,0,0-19-.93L171.17,90H84.83L66.66,40.56a9.94,9.94,0,0,0-19,.93L27.79,117.61A55.18,55.18,0,0,0,49,176.42l73.27,51.77a9.94,9.94,0,0,0,11.44,0L207,176.42A55.18,55.18,0,0,0,228.21,117.61ZM57.65,50.82,75,98.07A6,6,0,0,0,80.64,102h94.72A6,6,0,0,0,181,98.07l17.36-47.25,15,57.52L128,168.66,42.62,108.34ZM39.38,120.74,117.6,176,93.41,193.1,55.94,166.62A43.1,43.1,0,0,1,39.38,120.74ZM128,217.53l-24.19-17.09L128,183.35l24.19,17.09Zm72.06-50.91L162.59,193.1,138.4,176l78.22-55.26A43.1,43.1,0,0,1,200.06,166.62Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M230.15,117.1,210.25,41a11.94,11.94,0,0,0-22.79-1.11L169.78,88H86.22L68.54,39.87A11.94,11.94,0,0,0,45.75,41L25.85,117.1a57.19,57.19,0,0,0,22,61l73.27,51.76a11.91,11.91,0,0,0,13.74,0l73.27-51.76A57.19,57.19,0,0,0,230.15,117.1ZM58,57.5,73.13,98.76A8,8,0,0,0,80.64,104h94.72a8,8,0,0,0,7.51-5.24L198,57.5l13.07,50L128,166.21,44.9,107.5ZM40.68,124.11,114.13,176,93.41,190.65,57.09,165A41.06,41.06,0,0,1,40.68,124.11Zm87.32,91-20.73-14.65L128,185.8l20.73,14.64ZM198.91,165l-36.32,25.66L141.87,176l73.45-51.9A41.06,41.06,0,0,1,198.91,165Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M226.27,118.11,206.38,42a7.94,7.94,0,0,0-15.16-.75L172.57,92H83.43L64.78,41.24A7.94,7.94,0,0,0,49.62,42L29.73,118.11a53.16,53.16,0,0,0,20.44,56.68l73.27,51.76a7.9,7.9,0,0,0,9.12,0l73.27-51.76A53.16,53.16,0,0,0,226.27,118.11Zm-169-74L76.89,97.38A4,4,0,0,0,80.64,100h94.72a4,4,0,0,0,3.75-2.62l19.57-53.22,17,65L128,171.11,40.33,109.17Zm-19.84,76,.7-2.7L121.07,176,93.41,195.54,54.78,168.25A45.11,45.11,0,0,1,37.47,120.14ZM128,220l-27.66-19.54L128,180.9l27.66,19.54Zm73.22-51.73-38.63,27.29L134.93,176l82.9-58.56.7,2.7A45.11,45.11,0,0,1,201.22,168.25Z"></path>
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
