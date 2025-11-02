use crate::app::get_repo_stars;
use leptos::prelude::*;

//TODO: Stream zeigen im Network und auch gleich schon serverseit request ist drin
//TODO: Wechsel zu marockuoni vim-golf 0 :(, github bereit halten und star setzen, refresh juhui 1.
// https://github.com/marcokuoni/vim-golf

#[component]
pub fn Serverfunktionen() -> impl IntoView {
    let (owner, set_owner) = signal("leptos-rs".to_string());
    let (repo, set_repo) = signal("leptos".to_string());
    let stars = Resource::new(
        move || (owner(), repo()),
        // every time `count` changes, this will run
        |(o, r)| async move { get_repo_stars(o, r).await.ok() },
    );

    view! {
        <section class="slide">
            <h2 class="h2">"Server Functions"</h2>
            <p class="note">"Server fragt GitHub‑API ab (User‑Agent gesetzt)."</p>
            <div style="display:flex; gap:.5rem; flex-wrap:wrap;">
                <input
                    class="btn"
                    prop:value=owner()
                    on:input=move |ev| set_owner(event_target_value(&ev))
                />
                <input
                    class="btn"
                    prop:value=repo()
                    on:input=move |ev| set_repo(event_target_value(&ev))
                />
                <button class="btn" on:click=move |_| stars.refetch()>
                    "Sterne laden"
                </button>
            </div>
            <p>
                <Suspense>
                    {move || match stars.get() {
                        None => view! { <span class="note">"(noch nichts geladen)"</span> },
                        Some(None) => view! { <span class="note">"Fehler oder Rate Limit."</span> },
                        Some(Some(n)) => view! { <span class="note">{format!("⭐ {} Sterne", n)}</span> },
                    }}
                </Suspense>
            </p>
        </section>
    }
}
