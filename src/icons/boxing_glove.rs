//! GENERATED FILE

use crate::IconWeight;
use leptos::*;

#[cfg(any(feature = "games", feature = "health", feature = "objects"))]
#[component]
pub fn BoxingGlove(
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
                <path d="M168,16H120A56,56,0,0,0,64,72v31.73A8.17,8.17,0,0,1,56.53,112,8,8,0,0,1,48,104V78.7a4,4,0,0,0-5.63-3.65A32,32,0,0,0,24,104v29.19a16.14,16.14,0,0,0,3.5,10q.3.36.63.69L64,179.34V216a16,16,0,0,0,16,16H192a16,16,0,0,0,16-16V177.12l15.38-53.84a16,16,0,0,0,.62-4.4V72A56,56,0,0,0,168,16Zm3.58,168.84a8,8,0,0,1-7.16,14.32L136,184.94l-28.42,14.22a8,8,0,1,1-7.16-14.32L118.11,176l-17.69-8.84a8,8,0,1,1,7.16-14.32L136,167.06l28.42-14.22a8,8,0,1,1,7.16,14.32L153.89,176Z"></path>
            }.into_view(),
IconWeight::Duotone => view! {
    <path
        d="M216,72v46.88a7.85,7.85,0,0,1-.31,2.2L200,176v40a8,8,0,0,1-8,8H80a8,8,0,0,1-8-8V176L33.75,138.19a8,8,0,0,1-1.75-5V104A24,24,0,0,1,56,80H72V72a48,48,0,0,1,48-48h48A48,48,0,0,1,216,72Z"
        opacity="0.2"
    ></path>
    <path d="M168,16H120A56.06,56.06,0,0,0,64,72H56a32,32,0,0,0-32,32v29.19a16.14,16.14,0,0,0,3.5,10q.3.36.63.69L64,179.34V216a16,16,0,0,0,16,16H192a16,16,0,0,0,16-16V177.12l15.38-53.84a16,16,0,0,0,.62-4.4V72A56.06,56.06,0,0,0,168,16Zm40,102.88L192.31,173.8A7.85,7.85,0,0,0,192,176v40H80V176a8,8,0,0,0-2.38-5.69L40,133.12V104A16,16,0,0,1,56,88h8v16a8,8,0,0,0,16,0V72a40,40,0,0,1,40-40h48a40,40,0,0,1,40,40Zm-36.42,48.28L153.89,176l17.69,8.84a8,8,0,0,1-7.16,14.32L136,184.94l-28.42,14.22a8,8,0,1,1-7.16-14.32L118.11,176l-17.69-8.84a8,8,0,1,1,7.16-14.32L136,167.06l28.42-14.22a8,8,0,1,1,7.16,14.32Z"></path>
}.into_view(),
IconWeight::Thin => view! {
    <path d="M168,20H120A52.06,52.06,0,0,0,68,72v4H56a28,28,0,0,0-28,28v29.19a12,12,0,0,0,2.63,7.5c.1.12.2.24.31.35L68,177.67V216a12,12,0,0,0,12,12H192a12,12,0,0,0,12-12V176.56l15.54-54.38a12.34,12.34,0,0,0,.46-3.3V72A52.06,52.06,0,0,0,168,20Zm44,98.88a4.11,4.11,0,0,1-.15,1.1l-15.7,54.92A4.11,4.11,0,0,0,196,176v40a4,4,0,0,1-4,4H80a4,4,0,0,1-4-4V176a4,4,0,0,0-1.19-2.84L36.76,135.54a4,4,0,0,1-.76-2.35V104A20,20,0,0,1,56,84H68v20a4,4,0,0,0,8,0V72a44.05,44.05,0,0,1,44-44h48a44.05,44.05,0,0,1,44,44Zm-42.21,44.7L145,176l24.84,12.42a4,4,0,0,1-3.58,7.16L136,180.47l-30.21,15.11a4,4,0,1,1-3.58-7.16L127.05,176l-24.84-12.42a4,4,0,1,1,3.58-7.16L136,171.53l30.21-15.11a4,4,0,0,1,3.58,7.16Z"></path>
}.into_view(),
IconWeight::Bold => view! {
    <path d="M168,12H120A60.08,60.08,0,0,0,60.13,68H56a36,36,0,0,0-36,36v29.19a20.13,20.13,0,0,0,4.38,12.5,11.46,11.46,0,0,0,.94,1L60,181v35a20,20,0,0,0,20,20H192a20,20,0,0,0,20-20V177.68l15.23-53.3a20.07,20.07,0,0,0,.77-5.5V72A60.07,60.07,0,0,0,168,12Zm36,106.32L188.46,172.7A12.28,12.28,0,0,0,188,176v36H84V176a12,12,0,0,0-3.56-8.53L44,131.45V104A12,12,0,0,1,56,92h4v12a12,12,0,0,0,24,0V72a36,36,0,0,1,36-36h48a36,36,0,0,1,36,36ZM166.66,162l-9,6,9,6a12,12,0,1,1-13.32,20L136,182.42,118.66,194a12,12,0,0,1-13.32-20l9-6-9-6a12,12,0,0,1,13.32-20L136,153.58,153.34,142a12,12,0,1,1,13.32,20Z"></path>
}.into_view(),
IconWeight::Light => view! {
    <path d="M168,18H120A54.06,54.06,0,0,0,66,72v2H56a30,30,0,0,0-30,30v29.19a14,14,0,0,0,3.07,8.75,5.66,5.66,0,0,0,.47.52l36.46,36V216a14,14,0,0,0,14,14H192a14,14,0,0,0,14-14V176.84l15.46-54.11a13.93,13.93,0,0,0,.54-3.85V72A54.06,54.06,0,0,0,168,18Zm42,100.88a2,2,0,0,1-.08.55l-15.69,54.92A6.14,6.14,0,0,0,194,176v40a2,2,0,0,1-2,2H80a2,2,0,0,1-2-2V176a6,6,0,0,0-1.78-4.27L38.3,134.25a2,2,0,0,1-.3-1.06V104A18,18,0,0,1,56,86H66v18a6,6,0,0,0,12,0V72a42,42,0,0,1,42-42h48a42,42,0,0,1,42,42Zm-39.32,46.49L149.42,176l21.26,10.63a6,6,0,0,1-5.36,10.74L136,182.71l-29.32,14.66a6,6,0,0,1-5.36-10.74L122.58,176l-21.26-10.63a6,6,0,0,1,5.36-10.74L136,169.29l29.32-14.66a6,6,0,1,1,5.36,10.74Z"></path>
}.into_view(),
IconWeight::Regular => view! {
    <path d="M168,16H120A56.06,56.06,0,0,0,64,72H56a32,32,0,0,0-32,32v29.19a16.09,16.09,0,0,0,3.51,10,8,8,0,0,0,.62.69L64,179.34V216a16,16,0,0,0,16,16H192a16,16,0,0,0,16-16V177.12l15.38-53.85a15.89,15.89,0,0,0,.62-4.39V72A56.06,56.06,0,0,0,168,16Zm40,102.88L192.31,173.8A7.85,7.85,0,0,0,192,176v40H80V176a8,8,0,0,0-2.38-5.69L40,133.12V104A16,16,0,0,1,56,88h8v16a8,8,0,0,0,16,0V72a40,40,0,0,1,40-40h48a40,40,0,0,1,40,40Zm-36.42,48.28L153.89,176l17.69,8.84a8,8,0,0,1-7.16,14.32L136,184.94l-28.42,14.22a8,8,0,1,1-7.16-14.32L118.11,176l-17.69-8.84a8,8,0,1,1,7.16-14.32L136,167.06l28.42-14.22a8,8,0,1,1,7.16,14.32Z"></path>
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
