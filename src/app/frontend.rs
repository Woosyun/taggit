use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes, ProtectedRoute},
    StaticSegment,
};
use leptos_use::{use_cookie_with_options, UseCookieOptions};
use codee::string::FromToStringCodec;

use super::pages::*;

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
                <Frontend/>
            </body>
        </html>
    }
}

#[derive(Clone)]
pub struct Authenticated(pub Resource<bool>);

#[component]
pub fn Frontend() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // Defining a session_token signal using leptos_use::use_cookie to track the presence of the token that axum-login creates
    // Defining an authenticated resource and a corresponding server function
    // Setting a route condition that checks the result of authenticated
    let (cookie, _) = use_cookie_with_options::<String, FromToStringCodec>(
        "id",
        UseCookieOptions::<String, _, _>::default()
            .readonly(true)
            .max_age(3000)
            .default_value(None)
    );
    let authenticated = Resource::new(cookie, |_| async move {
        match auth::authenticate().await {
            Ok(_) => true,
            Err(e) => {
                dbg!(e);
                false
            }
        }
    });
    // let authenticated = move || authenticated.get().expect("missing authentication info");
    provide_context(Authenticated(authenticated));
    let authenticated = move || authenticated.get().expect("missing authentication info");


    view! {
        <Stylesheet id="leptos" href="pkg/taggit.css"/>
        // <Stylesheet id="leptos" href="pkg/taildwind.css"/>
        <Title text="Welcome to Leptos"/>

        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("/") view=home_page::HomePage/>
                    // <Route path=StaticSegment("edit") view=EditPage/>
                    // <Route path=StaticSegment("/create") view=create_note_page::CreateNotePage/>
                    <ProtectedRoute 
                        path=StaticSegment("create") 
                        view=create_note_page::CreateNotePage
                        condition=move || Some(authenticated())
                        redirect_path=|| "/login"
                    />
                    <Route path=StaticSegment("/register") view=auth::RegisterPage />
                    <Route path=StaticSegment("/login") view=auth::LoginPage />
                </Routes>
            </main>
        </Router>
    }
}
