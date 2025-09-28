use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <head>
            <meta charset="utf-8"/>
            <meta name="viewport" content="width=device-width, initial-scale=1"/>
            <link rel="stylesheet" href="/assets/style.css"/>
            <Title text="Leptos SSR + Tailwind"/>
            <Script src="/pkg/rust-give-me-diet.js"/>  // wstrzykuje bundle klienta (cargo-leptos)
        </head>
        <body class="bg-slate-50">
            <Router>
                <main class="mx-auto max-w-xl p-6 space-y-6">
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
    // lokalny stan (hydration)
    let (count, set_count) = signal(0);

    view! {
        <section class="space-y-3">
            <h1 class="text-3xl font-bold text-slate-900">"Hello"</h1>

            <div class="space-x-2">
                <button
                    class="rounded bg-indigo-600 px-4 py-2 text-white hover:bg-indigo-500"
                    on:click=move |_| set_count.update(|n| *n += 1)
                >"Increment"</button>
                <span class="text-slate-700">{move || format!("Count = {}", count.get())}</span>
            </div>

        </section>
    }
}

#[component]
fn About() -> impl IntoView {
    view! { <h1 class="text-2xl font-semibold">"About"</h1> }
}

#[component]
fn NotFound() -> impl IntoView {
    view! { <h1 class="text-xl text-red-600">"404 – Not found"</h1> }
}
