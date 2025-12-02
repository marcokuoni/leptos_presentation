use leptos::prelude::*;

#[component]
pub fn Architektur() -> impl IntoView {
    view! {
        <section class="slide">
            <h2 class="h2">"Architektur von Leptos"</h2>
            <p>"Leptos ist ein Full-Stack-Webframework für Rust, das auf modernen Konzepten für reaktive UIs aufbaut."</p>
            <ul class="list">
                <li>
                    <b>"Rendering Optionen"</b>
                    <ul class="list">
                        <li>"Client-Side Rendering (CSR): Der Server gibt eine HTML-Seite mit der URL zur Leptos app, die mit WebAssembly (WASM) kompiliert worden ist, die URL zum JavaScript-Code,
                        welcher für das Laden des WASM blobs verwendet wird, und ein leeres <body>-Element. Sobald JS und WASM geladen sind, rendert Leptos die App in den <body>."</li>
                        <li>"Server-Side Rendering (SSR): Rückgabe einer initialen HTML-Seite vom Server, die den tatsächlichen Startzustand der Anwendung repräsentiert,
                        sodass der Benutzer während des Ladens von JS/WASM und bis zum Abschluss des Ladens auf die einfache HTML-Version zugreifen kann."</li>
                        <li>
                            "Server-Side Rendering (SSR) mit Hydration"
                            <ul class="list">
                                <li>"Die App wird initial auf dem Server zu HTML gerendert. Das sorgt für einen schnellen First-Paint und gute SEO (Search Engine Optimization)."</li>
                                <li>"Auf dem Client \"hydriert\" Leptos die statische HTML, macht sie reaktiv und übernimmt das weitere Routing."</li>
                            </ul>
                        </li>
                    </ul>
                </li>
                <li>
                    <b>"Fine-Grained Reactivity (statt Virtual DOM)"</b>
                    <ul class="list">
                        <li>"Anstatt bei jeder Zustandsänderung ganze Komponenten neu zu rendern wie dies React tut, aktualisiert Leptos nur die DOM-Elemente, die sich tatsächlich geändert haben."</li>
                        <li>
                            <b>"Signals"</b>
                            ": Die grundlegenden reaktiven Bausteine. Sie enthalten einen Wert und benachrichtigen abhängige Teile der UI bei Änderungen."
                        </li>
                        <li>
                            <b>"Memos & Derived Signals"</b>
                            ": Zwischengespeicherte, abgeleitete Werte, die nur bei Bedarf neu berechnet werden."
                        </li>
                    </ul>
                </li>
                <li>
                    <b>"Integrierter Router"</b>
                    <ul class="list">
                        <li>"Unterstützt verschachtelte Routen (Nested Routes) und Routen-Parameter."</li>
                        <li>"Funktioniert sowohl auf dem Server für das initiale Rendering als auch auf dem Client für schnelle Navigation."</li>
                    </ul>
                </li>
                <li>
                    <b>"Server Functions"</b>
                    <ul class="list">
                        <li>"Ermöglichen typsichere API-Aufrufe vom Client zum Server ohne manuellen Boilerplate-Code."</li>
                        <li>"Fühlen sich an wie normale asynchrone Rust-Funktionen, werden aber auf dem Server ausgeführt."</li>
                        <li>"Ideal für Datenbankabfragen, Authentifizierung und andere serverseitige Logik."</li>
                    </ul>
                </li>
            </ul>
        </section>
    }
}
