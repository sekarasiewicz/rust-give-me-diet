use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::{A, Route, Router, Routes};
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let opts = expect_context::<LeptosOptions>();

    view! {
        <head>
            <meta charset="utf-8"/>
            <meta name="viewport" content="width=device-width, initial-scale=1"/>
            <link rel="stylesheet" href="/assets/style.css"/>
            <Title text="Leptos SSR + Tailwind"/>
            <HydrationScripts options=opts.clone()/>
            <script type="module" src="/pkg/rust-give-me-diet.js"></script>
        </head>
        <body>
            <div id="root">
                // <Router>
                    <main class="mx-auto max-w-xl p-6 space-y-6">
                    <aside class="p-4 border-r">
                        <h2>"Admin"</h2>
                        <nav>
                            // <A href="/">"Dashboard"</A>
                            // <A href="/about">"Settings"</A>
                        </nav>
                    </aside>
                        // <Routes fallback=|| view! { <NotFound/> }>
                        //     <Route path=path!("") view=Home/>
                        //     <Route path=path!("/about") view=About/>
                        // </Routes>
                    </main>
                // </Router>
            </div>
        </body>
    }
}

#[component]
fn Home() -> impl IntoView {
    let (count, set_count) = signal(0);

    let increment = move |_| {
        set_count.update(|n| *n += 1);
    };

    view! {
        <section class="space-y-3">
            <h1 class="text-3xl font-bold text-slate-900">"Hello"</h1>

            <div class="space-x-2">
                <button
                    type="button"
                    class="rounded bg-indigo-600 px-4 py-2 text-white hover:bg-indigo-500"
                    on:click=increment
                >"Increment"</button>
                <span class="text-slate-700">
                    {move || format!("Count = {}", count.get())}
                </span>
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

// #[component]
// fn Home() -> impl IntoView {
//     // lokalny stan (hydration)
//     let (count, set_count) = create_signal(0);

//     // server action -> #[server] greet
//     let (name, set_name) = create_signal(String::new());
//     let greet_action = create_action(move |n: &String| {
//         let n = n.clone();
//         async move { greet(n).await.ok() }
//     });
//     let on_submit = move |e: web_sys::SubmitEvent| {
//         e.prevent_default();
//         if !name.get().trim().is_empty() {
//             greet_action.dispatch(name.get());
//         }
//     };

//     view! {
//         <section class="space-y-3">
//             <h1 class="text-3xl font-bold text-slate-900">"Hello"</h1>

//             <div class="space-x-2">
//                 <button
//                     class="rounded bg-indigo-600 px-4 py-2 text-white hover:bg-indigo-500"
//                     on:click=move |_| set_count.update(|n| *n += 1)
//                 >"Increment"</button>
//                 <span class="text-slate-700">{move || format!("Count = {}", count.get())}</span>
//             </div>

//             <form class="flex gap-2" on:submit=on_submit>
//                 <input
//                     class="flex-1 rounded border border-slate-300 px-3 py-2"
//                     type="text"
//                     placeholder="Your name"
//                     on:input=move |e| set_name(event_target_value(&e))
//                     prop:value=name
//                 />
//                 <button class="rounded bg-emerald-600 px-4 py-2 text-white hover:bg-emerald-500" type="submit">
//                     "Greet"
//                 </button>
//             </form>

//             <Show when=move || greet_action.value().get().is_some()>
//                 <p class="rounded bg-white p-3 shadow">
//                     {move || greet_action.value().get().flatten().unwrap_or_default()}
//                 </p>
//             </Show>
//         </section>
//     }
// }
