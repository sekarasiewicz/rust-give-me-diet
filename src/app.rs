use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::{A, Route, Router, Routes};
use leptos_router::path;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <head>
            <Meta charset="utf-8"/>
            <Meta name="viewport" content="width=device-width, initial-scale=1"/>
            <Title text="My Leptos App"/>
        </head>
        <body>
            <Router>
                <header>
                    <nav class="bg-gray-800 p-4">
                        <A href="/">"Home"</A>
                        " | "
                        <A href="/about">"About"</A>
                    </nav>
                </header>
                <main>
                    <Routes fallback=NotFound>
                        <Route path=path!("") view=Home/>
                        <Route path=path!("/about") view=About/>
                    </Routes>
                </main>
            </Router>
        </body>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! { <h1>"Hello World"</h1> }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <Title text="About · Hello SSR"/>
        <h1>"About"</h1>
        <p>"Minimal Leptos 0.8 + Axum 0.8."</p>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! { <h1>"404"</h1><p>"Not found."</p> }
}
