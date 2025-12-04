use leptos::prelude::*;

#[component]
pub fn Qwik() -> impl IntoView {
    view! {
        <section class="slide">
            <h2 class="h2">"Qwik"</h2>
            <ul class="list">
                <li>"Granulare Abhängigkeiten statt globalem Diff"</li>
            </ul>
            <h2 class="h2">"vs"</h2>
            <h2 class="h2">"leptos"</h2>
        </section>
    }
}
