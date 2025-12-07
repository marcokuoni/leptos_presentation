use leptos::prelude::*;

#[component]
pub fn Intro() -> impl IntoView {
    view! {
        <section class="slide">
            <h1 class="h1">"Leptos: Reactive Rust fürs Web"</h1>
            <p class="lead">
                "Präsentation in Leptos"
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
            <h2 class="h2">"Eignung / Erfahrung"</h2>
            <ul class="list" style="margin-top: 2rem">
                <li>"SEP Project: URL Shortener"</li>
                <li>"Full-stack Rust"</li>
                <li>"Steile Lernkurve"</li>
                <li>"Performant und typsicher"</li>
                <li>"Ökosystem und Tooling"</li>
            </ul>
        </section>
    }
}
