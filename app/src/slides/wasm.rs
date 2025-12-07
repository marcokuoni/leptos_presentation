use leptos::prelude::*;

// TODO: https://docs.rs/web-sys/latest/web_sys/

#[component]
pub fn Wasm() -> impl IntoView {
    view! {
        <section class="slide">
            <h2 class="h2">"WebAssembly"</h2>
            <ul class="list">
                <li>"Intermediate language"</li>
                <li>"Kompiliert von verschiedenen Sprachen"</li>
                <li>"Harvard Stackmachine"</li>
                <li>"Sandboxed"</li>
                <li>"WASI/WIT"</li>
                <li>"wasm-bindgen, web-sys"</li>
            </ul>
        </section>
    }
}
