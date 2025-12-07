use leptos::prelude::*;

#[component]
pub fn Different() -> impl IntoView {
    view! {
        <section class="slide">
            <h2 class="h2">"Die Wege trennen sich"</h2>
            <p>"Wie viel Zeit haben wir noch? Was wollt ihr noch interaktiv sehen?"</p>
            <ul class="list">
                <li>"Kleinere Applikationen und deren Code"</li>
                <li>"Wie funktioniert Rust im Frontend (WASM)"</li>
                <li>"Qwik vs. leptos"</li>
            </ul>
        </section>
    }
}
