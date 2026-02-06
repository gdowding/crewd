use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};


#[cfg(feature = "ssr")]
use tokio::time::{sleep, Duration};

use leptos::logging::log;
//use leptos::task::spawn_local;

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
    log!("beginning of App");
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/crewd.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}


#[server]
pub async fn get_schedule(n: i32) -> Result<String, ServerFnError> {
    log!("get_schedule");
    sleep(Duration::from_secs(1)).await;
    log!("after sleep");
    Ok(format!("schedule string '{n}'"))
    //Err(ServerFnError::ServerError("server error".to_string()))
}


/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    log!("begin homepage");
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;


    // let sched_data = move || { match sched_resource.get() {
    // 	None => view! {"Loading"}.into_any(),
    // 	Some(schedule) => view!{ schedule }.into_any()
    // }};

    let s_count = RwSignal::new(0);
    let sched_resource = Resource::new(move || s_count, |n| get_schedule(n.get()));
    let reload_schedule = move |_| *s_count.write() += 1;

    view! {
        <h1>"Welcome to Leptos!"</h1>
            <button on:click=on_click>"Click Me: " {count}</button>
	    <br/>

	    <button on:click=reload_schedule>Reload Schedule</button>
	    <br/>
	<Suspense
	    fallback=move || view! { <p>"Loading..."</p> }
	>
	{sched_resource}
        </Suspense>
	    <p>"after suspense"</p>

    }
}
