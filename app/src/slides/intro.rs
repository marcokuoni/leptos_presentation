use leptos::prelude::*;

#[component]
pub fn Intro() -> impl IntoView {
    view! {
        <section class="slide">
            <h1 class="h1">"Leptos: Reactive Rust fürs Web"</h1>
            <p class="lead">
                "Präsentation in Leptos: SSR + Hydration, Router, Tasten & Serverfunktionen."
            </p>
            <div style="margin-top:2rem">
                <span class="badge">"Rust"</span>
                <span style="display:inline-block;width:.5rem"></span>
                <span class="badge">"WASM"</span>
                <span style="display:inline-block;width:.5rem"></span>
                <span class="badge">"SSR (Streaming)"</span>
                <span style="display:inline-block;width:.5rem"></span>
                <span class="badge">"Hydration/Island"</span>
                <span style="display:inline-block;width:.5rem"></span>
                <span class="badge">"Signals"</span>
            </div>
        </section>
    }
}
