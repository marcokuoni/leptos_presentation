use leptos::prelude::*;

//TODO: Hier kann man noch viel mehr erzählen, spezielle Konzepte Highlighten :) Noch überlegen
//wie. Vielleich braucht es keine Folie sondern man zeigt es im Browser oder zeichnet was.

#[component]
pub fn Architektur() -> impl IntoView {
    view! {
        <section class="slide">
            <h2 class="h2">"Architektur"</h2>
            <ul class="list">
                <li>
                    <b>SSR</b>
                    :
                    "schneller First Paint, SEO"
                </li>
                <li>
                    <b>Hydration</b>
                    :
                    "Server gerendert, Client reaktiv"
                </li>
                <li>
                    <b>Fine-grained Reactivity</b>
                    :
                    "Signale, Memos, Ressourcen"
                </li>
                <li>
                    <b>Router</b>
                    :
                    "Nested Routes, Params"
                </li>
                <li>
                    <b>Server Functions</b>
                    :
                    "typsichere API ohne Boilerplate"
                </li>
            </ul>
        </section>
    }
}
