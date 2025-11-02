use leptos::prelude::*;

#[component]
pub fn Ecosystem() -> impl IntoView {
    view! {
        <section class="slide">
            <h2 class="h2">"Ecosystem & Tooling"</h2>
            <ul class="list">
                <li>"leptos_router – Routing"</li>
                <li>"leptos-use – Browser Hooks"</li>
                <li>"Axum – SSR Integration"</li>
                <li>"cargo-leptos – Build & Dev"</li>
            </ul>
        </section>
    }
}
