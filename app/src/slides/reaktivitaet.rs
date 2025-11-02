use leptos::prelude::*;

// TODO: add code on site, maybe interactive?

#[component]
pub fn Reaktivitaet() -> impl IntoView {
    let (count, set_count) = signal(0);
    let double = Memo::new(move |_| count() * 2);
    view! {
        <section class="slide">
            <h2 class="h2">"Fine-grained Reaktivitaet (Live)"</h2>
            <button class="btn" on:click=move |_| set_count(count() + 1)>
                {move || format!("Klicks: {}", count())}
            </button>
            <p>{move || format!("Doppelt: {}", double())}</p>
        </section>
    }
}
