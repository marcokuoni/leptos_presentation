use leptos::prelude::*;

#[component]
pub fn Qwik() -> impl IntoView {
    view! {
        <section class="slide">
            <h2 class="h2">"Qwik"</h2>
            <ul class="list">
                <li>"State Transfer"</li>
                <li>"Hydration"</li>
                <li>"Splitting"</li>
                <li>"Lazy Loading"</li>
            </ul>
            <h2 class="h2" style="margin: 2rem 0">"vs"</h2>
            <h2 class="h2">"leptos"</h2>
            <ul>
                <li>"Island"</li>
                <ul>
                   <li>"State Transfer"</li>
                   <li>"Hydration"</li>
                </ul>
                <li>"Lazy"</li>
                <ul>
                   <li>"Splitting"</li>
                   <li>"Lazy Loading"</li>
                </ul>
            </ul>
        </section>
    }
}
