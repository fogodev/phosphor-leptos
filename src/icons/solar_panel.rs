//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "commerce", feature = "objects"))]
#[component]
pub fn SolarPanel(
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
                <path d="M32,104a8,8,0,0,1,8-8H56a8,8,0,0,1,0,16H40A8,8,0,0,1,32,104ZM71.43,58.75A8,8,0,0,0,82.75,47.43L71.43,36.12A8,8,0,0,0,60.12,47.43ZM128,40a8,8,0,0,0,8-8V16a8,8,0,0,0-16,0V32A8,8,0,0,0,128,40Zm50.91,21.09a8,8,0,0,0,5.66-2.34l11.31-11.32a8,8,0,0,0-11.31-11.31L173.25,47.43a8,8,0,0,0,5.66,13.66ZM192,104a8,8,0,0,0,8,8h16a8,8,0,0,0,0-16H200A8,8,0,0,0,192,104ZM88,112a8,8,0,0,0,8-8,32,32,0,0,1,64,0,8,8,0,0,0,16,0,48,48,0,0,0-96,0A8,8,0,0,0,88,112Zm55.2,24H112.8a4,4,0,0,0-3.91,3.15L102.62,168h50.76l-6.27-28.85A4,4,0,0,0,143.2,136ZM31.75,186,17,212.06a8,8,0,0,0,1.16,9.45,8.22,8.22,0,0,0,6,2.49H70.85a4,4,0,0,0,3.91-3.15l8-36.85H35.23A4,4,0,0,0,31.75,186Zm207.21,26-14.71-26a4,4,0,0,0-3.48-2H173.23l8,36.85a4,4,0,0,0,3.91,3.15h46.62a8.22,8.22,0,0,0,6-2.49A8,8,0,0,0,239,212.06Zm-28.27-50-12.42-22a8,8,0,0,0-7-4.06H167.76a4,4,0,0,0-3.91,4.85l5.9,27.15H207.2A4,4,0,0,0,210.69,162ZM88.24,136H64.7a8,8,0,0,0-7,4.06L45.31,162a4,4,0,0,0,3.49,6H86.25l5.9-27.15A4,4,0,0,0,88.24,136Zm68.62,48H99.14L91.5,219.15A4,4,0,0,0,95.41,224h65.18a4,4,0,0,0,3.91-4.85Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path d="M232,216H24l40.7-72H191.3Z" opacity="0.2"></path>
    <path d="M32,104a8,8,0,0,1,8-8H56a8,8,0,0,1,0,16H40A8,8,0,0,1,32,104ZM71.43,58.75A8,8,0,0,0,82.75,47.43L71.43,36.12A8,8,0,0,0,60.12,47.43ZM128,40a8,8,0,0,0,8-8V16a8,8,0,0,0-16,0V32A8,8,0,0,0,128,40Zm50.91,21.09a8,8,0,0,0,5.66-2.34l11.31-11.32a8,8,0,0,0-11.31-11.31L173.25,47.43a8,8,0,0,0,5.66,13.66ZM192,104a8,8,0,0,0,8,8h16a8,8,0,0,0,0-16H200A8,8,0,0,0,192,104ZM88,112a8,8,0,0,0,8-8,32,32,0,0,1,64,0,8,8,0,0,0,16,0,48,48,0,0,0-96,0A8,8,0,0,0,88,112ZM238.91,220a8,8,0,0,1-6.91,4H24a8,8,0,0,1-7-11.94l40.69-72a8,8,0,0,1,7-4.06H191.3a8,8,0,0,1,7,4.06l40.69,72A8,8,0,0,1,238.91,220Zm-52.27-68H162.27l3.48,16h29.93Zm-37.26,16-3.48-16H110.1l-3.48,16Zm-46.24,16-5.21,24h60.14l-5.21-24ZM60.32,168H90.25l3.48-16H69.36ZM37.71,208H81.55l5.22-24H51.28Zm180.58,0-13.57-24H169.23l5.22,24Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M36,104a4,4,0,0,1,4-4H56a4,4,0,0,1,0,8H40A4,4,0,0,1,36,104ZM74.26,55.92a4,4,0,1,0,5.66-5.66L68.6,39A4,4,0,0,0,63,44.6ZM128,36a4,4,0,0,0,4-4V16a4,4,0,0,0-8,0V32A4,4,0,0,0,128,36Zm50.91,21.09a4,4,0,0,0,2.83-1.17L193.05,44.6A4,4,0,1,0,187.4,39L176.08,50.26a4,4,0,0,0,2.83,6.83ZM200,108h16a4,4,0,0,0,0-8H200a4,4,0,0,0,0,8ZM88,108a4,4,0,0,0,4-4,36,36,0,0,1,72,0,4,4,0,0,0,8,0,44,44,0,0,0-88,0A4,4,0,0,0,88,108ZM235.45,218a4,4,0,0,1-3.45,2H24a4,4,0,0,1-3.48-6l40.69-72a4,4,0,0,1,3.49-2H191.3a4,4,0,0,1,3.49,2l40.69,72A4,4,0,0,1,235.45,218ZM189,148H157.31l5.22,24h40Zm-34.63,24-5.22-24H106.88l-5.22,24Zm-54.42,8-7,32H163l-7-32Zm-46.46-8h40l5.22-24H67Zm-22.6,40H84.78l7-32H48.94Zm194.28,0-18.08-32H164.27l6.95,32Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M32,100A12,12,0,0,1,44,88h8a12,12,0,0,1,0,24H44A12,12,0,0,1,32,100Zm96-64a12,12,0,0,0,12-12V16a12,12,0,0,0-24,0v8A12,12,0,0,0,128,36ZM65.77,54.74a12,12,0,1,0,17-17l-5.66-5.65a12,12,0,0,0-17,17Zm116,3.52a12,12,0,0,0,8.49-3.52l5.65-5.65a12,12,0,0,0-17-17l-5.66,5.65a12,12,0,0,0,8.49,20.49ZM192,100a12,12,0,0,0,12,12h8a12,12,0,0,0,0-24h-8A12,12,0,0,0,192,100ZM88,112a12,12,0,0,0,12-12,28,28,0,0,1,56,0,12,12,0,0,0,24,0,52,52,0,0,0-104,0A12,12,0,0,0,88,112ZM242.36,222.05A12,12,0,0,1,232,228H24a12,12,0,0,1-10.45-17.9l43-76A12,12,0,0,1,67,128H189a12,12,0,0,1,10.45,6.1l43,76A12,12,0,0,1,242.36,222.05ZM182,152H140v12h48.82ZM74,152l-6.78,12H116V152ZM44.57,204H116V188H53.61Zm166.86,0-9-16H140v16Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M34,104a6,6,0,0,1,6-6H56a6,6,0,0,1,0,12H40A6,6,0,0,1,34,104ZM72.84,57.33a6,6,0,1,0,8.49-8.48L70,37.53A6,6,0,0,0,61.53,46ZM128,38a6,6,0,0,0,6-6V16a6,6,0,0,0-12,0V32A6,6,0,0,0,128,38Zm50.91,21.09a6,6,0,0,0,4.25-1.76L194.47,46A6,6,0,0,0,186,37.53L174.67,48.85a6,6,0,0,0,4.24,10.24ZM194,104a6,6,0,0,0,6,6h16a6,6,0,0,0,0-12H200A6,6,0,0,0,194,104ZM88,110a6,6,0,0,0,6-6,34,34,0,0,1,68,0,6,6,0,0,0,12,0,46,46,0,0,0-92,0A6,6,0,0,0,88,110ZM237.18,219a6,6,0,0,1-5.18,3H24a6,6,0,0,1-5.22-8.95l40.69-72A6,6,0,0,1,64.7,138H191.3a6,6,0,0,1,5.23,3.05l40.69,72A6,6,0,0,1,237.18,219ZM187.8,150h-28l4.35,20h35Zm-35.94,20-4.35-20h-39l-4.35,20Zm-50.33,12-6.09,28h65.12l-6.09-28ZM56.89,170h35l4.35-20h-28ZM34.28,210H83.16l6.09-28H50.11Zm187.44,0-15.83-28H166.75l6.09,28Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M32,104a8,8,0,0,1,8-8H56a8,8,0,0,1,0,16H40A8,8,0,0,1,32,104ZM71.43,58.75A8,8,0,0,0,82.75,47.43L71.43,36.12A8,8,0,0,0,60.12,47.43ZM128,40a8,8,0,0,0,8-8V16a8,8,0,0,0-16,0V32A8,8,0,0,0,128,40Zm50.91,21.09a8,8,0,0,0,5.66-2.34l11.31-11.32a8,8,0,0,0-11.31-11.31L173.25,47.43a8,8,0,0,0,5.66,13.66ZM192,104a8,8,0,0,0,8,8h16a8,8,0,0,0,0-16H200A8,8,0,0,0,192,104ZM88,112a8,8,0,0,0,8-8,32,32,0,0,1,64,0,8,8,0,0,0,16,0,48,48,0,0,0-96,0A8,8,0,0,0,88,112ZM238.91,220a8,8,0,0,1-6.91,4H24a8,8,0,0,1-7-11.94l40.69-72a8,8,0,0,1,7-4.06H191.3a8,8,0,0,1,7,4.06l40.69,72A8,8,0,0,1,238.91,220Zm-52.27-68H162.27l3.48,16h29.93Zm-37.26,16-3.48-16H110.1l-3.48,16Zm-46.24,16-5.21,24h60.14l-5.21-24ZM60.32,168H90.25l3.48-16H69.36ZM37.71,208H81.55l5.22-24H51.28Zm180.58,0-13.57-24H169.23l5.22,24Z"></path>
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
