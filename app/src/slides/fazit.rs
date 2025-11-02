use leptos::prelude::*;

#[component]
pub fn Fazit() -> impl IntoView {
    view! {
        <section class="slide">
            <h2 class="h2">"Fazit"</h2>
            <ul class="list">
                <li>"Produktiv: Reaktiv ohne Boilerplate"</li>
                <li>"Schnell: SSR + Hydration + WASM"</li>
                <li>"Vielseitig: SPA bis Fullstack"</li>
            </ul>
        </section>
    }
}
