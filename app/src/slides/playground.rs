use leptos::prelude::*;
use leptos::wasm_bindgen::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys;
use pulldown_cmark::{html, Options, Parser};
use web_sys::{window, HtmlScriptElement};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = initMonacoLang)]
    fn init_monaco_lang(
        el: &web_sys::HtmlElement,
        lang: &str,
        value: &str,
        on_change: &js_sys::Function,
    );
}

/// Call this once (e.g. at app start) to load Monaco and install the shims.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn load_monaco_into_head() -> Result<(), JsValue> {
    let win = window().ok_or("no window")?;

    // Guard: if we're already loading or loaded, bail out
    if js_sys::Reflect::has(&win, &JsValue::from_str("__monaco_loading")).unwrap_or(false) {
        return Ok(());
    }
    js_sys::Reflect::set(&win, &"__monaco_loading".into(), &true.into()).ok();

    let doc = win.document().ok_or("no document")?;
    let head = doc.head().ok_or("no <head>")?;

    // 1) Set the AMD 'require' config before loading loader.js
    let cfg = r#"
      var require = { paths: { vs: "https://unpkg.com/monaco-editor@0.45.0/min/vs" } };
    "#;
    let cfg_tag: HtmlScriptElement = doc.create_element("script")?.unchecked_into();
    cfg_tag.set_type("text/javascript");
    cfg_tag.set_text(cfg);
    head.append_child(&cfg_tag)?;

    // 2) Load the AMD loader
    let loader: HtmlScriptElement = doc.create_element("script")?.unchecked_into();
    loader.set_src("https://unpkg.com/monaco-editor@0.45.0/min/vs/loader.js");

    // When loader finishes, define your window.initMonaco* shims
    let shim_js = r#"
      // Shim 1: Rust-Editor (Playground)
      window.initMonaco = (el, value, onChange) => {
        require(["vs/editor/editor.main"], () => {
          const ed = monaco.editor.create(el, {
            value,
            language: "rust",
            theme: "vs-dark",
            automaticLayout: true,
            minimap: { enabled: false },
            fontSize: 14,
          });
          ed.onDidChangeModelContent(() => onChange(ed.getValue()));
        });
      };
      // Shim 2: generische Sprache (Markdown etc.)
      window.initMonacoLang = (el, lang, value, onChange) => {
        require(["vs/editor/editor.main"], () => {
          const ed = monaco.editor.create(el, {
            value,
            language: lang,
            theme: "vs-dark",
            automaticLayout: true,
            minimap: { enabled: false },
            fontSize: 14,
          });
          ed.onDidChangeModelContent(() => onChange(ed.getValue()));
        });
      };
    "#;

    let doc2 = doc.clone();
    let win2 = win.clone();
    let onload = Closure::<dyn FnMut()>::new(move || {
        let shim: HtmlScriptElement = doc2.create_element("script").unwrap().unchecked_into();
        shim.set_text(shim_js);
        doc2.head().unwrap().append_child(&shim).unwrap();

        // Mark as loaded
        js_sys::Reflect::set(&win2, &"__monaco_loaded".into(), &true.into()).ok();

        // Signal readiness
        let evt = web_sys::CustomEvent::new("monaco-ready").unwrap();
        let _ = win2.dispatch_event(&evt);
    });
    loader.set_onload(Some(onload.as_ref().unchecked_ref()));
    onload.forget(); // keep the closure alive

    head.append_child(&loader)?;
    Ok(())
}
#[component]
pub fn Playground() -> impl IntoView {
    let initial_md = r#"
# Live‑Markdown


Schreibe **Markdown** links, die _rechte Seite_ rendert live.


- *Kursiv*, **Fett**, `Code`
- Listen, Links, Ueberschriften


> Tipp: Tabellen funktionieren auch


| Spalte | Wert |
|-------:|-----:|
| Foo | 42 |
| Bar | 99 |


```rust
fn main() { println!("Hallo Markdown!"); }
"#;

    let (md, set_md) = signal(initial_md.to_string());
    let editor_ref = NodeRef::<leptos::html::Div>::new();
    // on_mount(move || {
    #[cfg(target_arch = "wasm32")]
    Effect::new(move || {
        // Start loading (no-op if already done)
        let _ = load_monaco_into_head();

        // If editor node not yet in DOM, nothing to do
        let Some(div) = editor_ref.get() else { return };

        // Helper: perform init now
        let do_init = {
            let div = div.clone();
            let set_md = set_md.clone();
            move || {
                // Build callback for Monaco -> Rust
                let closure =
                    Closure::<dyn FnMut(js_sys::JsString)>::new(move |v: js_sys::JsString| {
                        set_md(v.into())
                    });
                let f: &js_sys::Function = closure.as_ref().unchecked_ref();

                // Call the JS shim
                init_monaco_lang(&div.clone().unchecked_into(), "markdown", initial_md, f);

                // Leak the closure so the callback stays alive
                closure.forget();
            }
        };

        // If shim is already present, init immediately
        let win = web_sys::window().unwrap();
        let ready =
            js_sys::Reflect::has(&win, &JsValue::from_str("initMonacoLang")).unwrap_or(false);

        if ready {
            do_init();
            return;
        }

        // Otherwise wait exactly once for "monaco-ready"
        let cb = Closure::<dyn FnMut(web_sys::CustomEvent)>::new(move |_| {
            do_init();
        });
        win.add_event_listener_with_callback("monaco-ready", cb.as_ref().unchecked_ref())
            .unwrap();
        cb.forget(); // one-time listener is fine to keep around
    });

    let html_out = Memo::new(move |_| {
        let mut opts = Options::empty();
        opts.insert(Options::ENABLE_TABLES);
        opts.insert(Options::ENABLE_STRIKETHROUGH);
        let md = md();
        let parser = Parser::new_ext(&md, opts);
        let mut buf = String::new();
        html::push_html(&mut buf, parser);
        ammonia::Builder::default().clean(&buf).to_string()
    });

    view! {
        <section class="slide">
            <h2 class="h2">"Playground: Live‑Markdown"</h2>
            <div style="display:grid; grid-template-columns: 1fr 1fr; gap:1rem; min-height: 420px;">
                <div
                    node_ref=editor_ref
                    style="border:1px solid #1e2a3a; border-radius:12px; min-height:420px;"
                ></div>
                <div class="code" inner_html=move || html_out.get()></div>
            </div>
        </section>
    }
}
