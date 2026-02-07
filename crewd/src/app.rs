use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use serde::{Deserialize, Serialize};

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
pub async fn get_schedule_string(n: i32) -> Result<String, ServerFnError> {
    log!("get_schedule_string");
    sleep(Duration::from_secs(1)).await;
    log!("after sleep");
    Ok(format!("schedule string '{n}'"))
}

#[server]
pub async fn get_events() -> Result<Vec<Event>, ServerFnError> {

    log!("get_events");
    let events = vec![
	Event{
	    key: "sfoo".to_string(),
	    name: "race 4".to_string(),
	},
	Event{
	    key: "sbar".to_string(),
	    name: "race 5".to_string(),
	},
	Event{
	    key: "sbaz".to_string(),
	    name: "race 6".to_string(),
	},
    ];
    Ok(events)
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    key: String,
    name: String,
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    log!("begin homepage");

    let (events, _set_events) = signal(vec![
	Event{
	    key: "foo".to_string(),
	    name: "race 1".to_string(),
	},
	Event{
	    key: "bar".to_string(),
	    name: "race 2".to_string(),
	},
	Event{
	    key: "baz".to_string(),
	    name: "race 3".to_string(),
	},
    ]);

    let s_count = RwSignal::new(0);
    let sched_resource = Resource::new(move || s_count, |n| get_schedule_string(n.get()));
    let reload_schedule = move |_| *s_count.write() += 1;

    // server events
    let events_r = Resource::new(move || (), |_| get_events());

    view! {
        <h1>"Welcome to Crewd"</h1>
	    <button on:click=reload_schedule>Reload Schedule</button>
	    <br/>
	<Suspense
	    fallback=move || view! { <p>"Loading..."</p> }
	>
	{sched_resource}
        </Suspense>
	    <p>"after suspense"</p>
	    <For
	    each=move || events.get()
	    key=|event| event.key.clone()
	    let(event)
	    >
	    <p>{event.name}</p>
	    </For>

	    <h2>server events</h2>
	<Suspense
	    fallback=move || view! {<p>"Loading..."</p>}
	>
	{move ||
	 match events_r.get() {
	     None => view! {<p>"got nothing"</p>}.into_any(),
	     Some(result) =>
		 match result {
		     Ok(events) => events.into_iter().map(move |event| view! {<p>{event.name}</p>}).collect::<Vec<_>>().into_any(),
		     Err(e) => view! { <p>"got error"</p> }.into_any()
		 }
	 }
	}
	    </Suspense>



    }
}
