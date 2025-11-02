use leptos::prelude::*;
#[allow(unused_imports)]
use leptos::web_sys::KeyboardEvent;
use leptos_router::hooks::{use_location, use_navigate};
#[allow(unused_imports)]
use leptos_use::use_event_listener;

#[component]
pub fn Nav() -> impl IntoView {
    let location = use_location();
    let navigate = use_navigate();
    let nav_back = navigate.clone();
    let nav_next = navigate.clone();
    const SLIDES: &[&str] = &[
        "/",
        "/architektur",
        "/reaktivitaet",
        "/serverfunktionen",
        "/performance",
        "/ecosystem",
        "/playground",
        "/fazit",
    ];

    let index = Memo::new(move |_| {
        let path = location.pathname.get();
        SLIDES.iter().position(|s| *s == path).unwrap_or(0)
    });

    // Tastatur-Events
    #[cfg(target_arch = "wasm32")]
    let _cleanup = use_event_listener(
        document().body(),
        leptos::ev::keydown,
        move |ev: KeyboardEvent| {
            let i = index.get();
            match ev.key().as_str() {
                "ArrowRight" | "PageDown" => {
                    let j = (i + 1).min(SLIDES.len() - 1);
                    navigate(SLIDES[j], Default::default());
                }
                "ArrowLeft" | "PageUp" => {
                    let j = i.saturating_sub(1);
                    navigate(SLIDES[j], Default::default());
                }
                _ => {}
            }
        },
    );

    let progress_width = move || ((index.get() + 1) as f32 / SLIDES.len() as f32) * 100.0;

    view! {
        <nav class="navbar">
            <div class="note">
                "Tasten: "
                <kbd>{"←"}</kbd> "/"
                <kbd>{"→"}</kbd>
                </div>
            <div>
                <button class="btn" on:click=move |_| {
                    let i = index.get();
                    let j = i.saturating_sub(1);
                    nav_back(SLIDES[j], Default::default());
                }>"Zurueck"</button>
                <span style="display:inline-block;width:.5rem"></span>
                <button class="btn" on:click=move |_| {
                    let i = index.get();
                    let j = (i + 1).min(SLIDES.len() - 1);
                    nav_next(SLIDES[j], Default::default());
                }>"Weiter"</button>
            </div>
        </nav>
        <div class="progress"><span style=move || format!("width: {:.2}%", progress_width())></span></div>
    }
}
