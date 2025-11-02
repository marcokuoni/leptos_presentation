use leptos::prelude::*;

//TODO: können wir das zeigen: https://book.leptos.dev/deployment/binary_size.html?highlight=bundle#optimizing-wasm-binary-size
//https://www.youtube.com/watch?v=w5fhcoxQnII

#[component]
pub fn Performance() -> impl IntoView {
    view! {
        <section class="slide">
            <h2 class="h2">"Performance"</h2>
            <ul class="list">
                <li>"Granulare Abhängigkeiten statt globalem Diff"</li>
                <li>"SSR -> schnelle TTFB & Hydration"</li>
                <li>"Kleine WASM Bundles"</li>
            </ul>
        </section>
    }
}
