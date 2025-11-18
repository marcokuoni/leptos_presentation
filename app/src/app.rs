use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
#[cfg(feature = "ssr")]
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
struct RepoInfo {
    stargazers_count: u64,
}

#[server]
pub async fn get_repo_stars(owner: String, repo: String) -> Result<u64, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let url = format!("https://api.github.com/repos/{owner}/{repo}");
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("leptos-praesentation"));
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;
        let resp = client.get(url).send().await?.error_for_status()?;
        let info: RepoInfo = resp.json().await?;
        Ok(info.stargazers_count)
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::ServerError(
            "Nur auf dem Server verfuegbar".into(),
        ))
    }
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/app.css"/>
        <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/prism-themes/1.9.0/prism-vsc-dark-plus.min.css" />

        // sets the document title
        <Title text="Leptos Präsentation"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=crate::slides::Intro/>
                    <Route path=StaticSegment("architektur") view=crate::slides::Architektur/>
                    <Route path=StaticSegment("reaktivitaet") view=crate::slides::Reaktivitaet/>
                    <Route path=StaticSegment("serverfunktionen") view=crate::slides::Serverfunktionen/>
                    <Route path=StaticSegment("performance") view=crate::slides::Performance/>
                    <Route path=StaticSegment("ecosystem") view=crate::slides::Ecosystem/>
                    <Route path=StaticSegment("playground") view=crate::slides::Playground/>
                    <Route path=StaticSegment("fazit") view=crate::slides::Fazit/>
                </Routes>
            </main>
            <crate::components::Nav />
        </Router>
    }
}
