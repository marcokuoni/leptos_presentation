use leptos::prelude::*;
use leptos::logging;

// TODO: add code on site, maybe interactive?

#[component]
pub fn Reaktivitaet() -> impl IntoView {
    let (count, set_count) = signal(0);
    
    let code_signals = r#"let (count, set_count) = signal(0);

view! {
    <button class="btn" on:click=move |_| set_count.update(|c| *c += 1)>
    {move || format!("Klicks: {}", count.get())}
    </button>
    <button class="btn" on:click=move |_| set_count.set(count.get() + 1)>
    {move || format!("Klicks: {}", count.get())}
    </button>
    <button class="btn" on:click=move |_| *set_count.write() += 1>
        {move || format!("Klicks: {}", count.get())}
    </button>
}"#;

    let (first_name, set_first_name) = signal("John".to_string());
    let (last_name, set_last_name) = signal("Doe".to_string());
    let derived_signal_full_name = move || 
        format!("{} {}", first_name.get(), last_name.get());
    let memo_full_name = Memo::new(move |_| {
        format!("{} {}", first_name.get(), last_name.get())
    });

    let code_memos = r#"let (first_name, set_first_name) = signal("John".to_string());
let (last_name, set_last_name) = signal("Doe".to_string());
let derived_signal_full_name = move ||
    format!("{} {}", first_name.get(), last_name.get());
let memo_full_name = Memo::new(move |_| {
    format!("{} {}", first_name.get(), last_name.get())
});

view! {
    <input
        type="text"
        placeholder="Vorname"
        on:input:target=move |ev| {
            set_first_name(ev.target().value());
        }
        prop:value=first_name
    />
    <input
        type="text"
        placeholder="Nachname"
        on:input:target=move |ev| {
            set_last_name(ev.target().value());
        }
        prop:value=last_name
    />
    <p>"Derived Signal: " {derived_signal_full_name}</p>
    <p>"Memo: " {memo_full_name}</p>}
}"#;

    Effect::new(move |_| {
        logging::log!("Count changed to: {}", count.get());
    });

    let code_effect = r#"Effect::new(move |_| {
    logging::log!("Count changed to: {}", count.get());
});
"#;

    let code_view_effect = r#"let (count, set_count) = signal(0);
view! {
    <p>{count}</p>
}"#;

    let code_view_effect_expanded = r#"let (count, set_count) = signal(0);

// create a DOM element
let document = leptos::document();
let p = document.create_element("p").unwrap();

// create an effect to reactively update the text
Effect::new(move |prev_value: Option<String>| {
    // first, access the signal’s value and convert it to a string
    let text = count.get().to_string();

    // if this is different from the previous value, update the node
    if prev_value.as_ref() != Some(&text) {
        p.set_text_content(Some(&text));
    }

    // return this value so we can memoize the next update
    text
});"#;

    view! {
        <section class="slide">
            <h2 class="h2">"Fine-grained Reactivity (Live)"</h2>

            <p>"In Leptos basiert die Reaktivität auf Signalen und Memos, die es ermöglichen,
            UI-Komponenten effizient zu aktualisieren, wenn sich die zugrunde liegenden Daten ändern."</p>

            <h3 class="h3">"Signals"</h3>

            <p>"Signals sind veränderbare Werte, die automatisch Updates an alle abhängigen Komponenten auslösen,
            wenn sie geändert werden."</p>

            <b>"Lesen von Signal-Werten:"</b>
            <ul>
                <li>".get()": "Liest den aktuellen Wert des Signals (klont den Wert)."</li>
                <li>".read()": "Liest den aktuellen Wert des Signals (referenziert den Wert)."</li>
                <li>".with(|v| ...)": "Führt eine Funktion mit dem aktuellen Wert des Signals aus."</li>
            </ul>

            <b>"Schreiben von Signal-Werten:"</b>
            <ul>
                <li>".set(new_value)": "Setzt den Wert des Signals auf einen neuen Wert und benachrichtigt alle Observers. Es ersetzt unter Umständen ein komplettes Objekt, nicht nur dessen Inhalt."</li>
                <li>".update(|v| ...)": "Aktualisiert den Wert des Signals basierend auf dem aktuellen Wert (referenziert den Wert) und benachrichtigt alle Observers."</li>
                <li>".write(): Gibt eine mutable reference auf den Wert des Signals zurück, der direkt geändert werden kann und alle Observers benachrichtigt."</li>
            </ul>

            <div class="h-bar">
                <div class="code-container">
                    <pre><code class="language-rust">{code_signals}</code></pre>
                </div>
                <div class="live-container">
                    <button class="btn" on:click=move |_| set_count.update(|c| *c += 1)>
                        {move || format!("Klicks: {}", count.get())}
                    </button>
                    <button class="btn" on:click=move |_| set_count.set(count.get() + 1)>
                        {move || format!("Klicks: {}", count.get())}
                    </button>
                    <button class="btn" on:click=move |_| *set_count.write() += 1>
                        {move || format!("Klicks: {}", count.get())}
                    </button>
                </div>
            </div>

            <h3 class="h3">"Memos"</h3>

            <p>"Memos sind abgeleitete Werte, die automatisch neu berechnet werden,
            wenn sich die Signale, von denen sie abhängen, ändern."</p>
            
            <p>"
            Es existiert zudem ein Konzept namens \"derived signals\", welches Rust-Closures verwendet.
            Dadurch wird der Wert der Closure bei jedem Zugriff neu berechnet, 
            während ein Memo das Ergebnis zwischenspeichert (cached) und nur aktualisiert, wenn sich die abhängigen Signale tatsächlich geändert haben.
            Derived Signals sollten standardmässig für schnelle, einfache Operationen (kein Overhead) und Memos nur für rechenintensive Aufgaben verwendet werden.
            "</p>

            <div class="h-bar">
                <div class="code-container">
                    <pre><code class="language-rust">{code_memos}</code></pre>
                </div>
                <div class="live-container">
                    <input
                        type="text"
                        placeholder="Vorname"
                        on:input:target=move |ev| {
                            set_first_name(ev.target().value());
                        }
                        prop:value=first_name
                    />
                    <input
                        type="text"
                        placeholder="Nachname"
                        on:input:target=move |ev| {
                            set_last_name(ev.target().value());
                        }
                        prop:value=last_name
                    />
                    <p>"Derived Signal: " {derived_signal_full_name}</p>
                    <p>"Memo: " {memo_full_name}</p>
                </div>
            </div>

            <h3 class="h3">"Effects"</h3>

            <p>"Effects ermöglichen es, Side Effects zu verwalten,
            wie z.B. das Aktualisieren des DOM oder das Durchführen von API-Aufrufen, wenn sich Signals verändern."</p>

            <p>"Effects dienen dazu, das reaktive System mit der nicht-reaktiven Aussenwelt zu synchronisieren, 
            nicht dazu, verschiedene reaktive Werte miteinander zu synchronisieren. Für diesen Use-Case sollten Memos oder derived signals verwendet werden."</p>

            <div class="h-bar">
                <div class="code-container">
                    <pre><code class="language-rust">{code_effect}</code></pre>
                </div>
                <div class="live-container">
                    <p>"Öffne die Konsole, um den Effect zu sehen."</p>
                </div>
            </div>

            <p>"Der Leptos DOM-Renderer stellt die Funktionalität von Effects bereit. Effektiv werden Aufrufe von Signals innerhalb von Views in einen Effect übersetzt, wie unten dargestellt."</p>

            <div class="code-container">
                <pre><code class="language-rust">{code_view_effect}</code></pre>
            </div>

            <p>"wird zu:"</p>

            <div class="code-container">
                <pre><code class="language-rust">{code_view_effect_expanded}</code></pre>
            </div>

            <h3 class="h3">"Weiterführende Ressourcen"</h3>

            <ul>
                <li><a href="https://book.leptos.dev/reactivity/working_with_signals.html" target="_blank">"Leptos Docs: Signals and Computed"</a></li>
                <li><a href="https://book.leptos.dev/reactivity/14_create_effect.html" target="_blank">"Leptos Docs: Effects"</a></li>
                <li><a href="https://book.leptos.dev/appendix_reactive_graph.html" target="_blank">"Leptos Docs: How does the Reactive System Work?"</a></li>
            </ul>

        </section>
    }
}
