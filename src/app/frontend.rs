use leptos::prelude::*;
use leptos::logging::log;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes, ProtectedRoute},
    StaticSegment,
};
use crate::user::PublicUserProfile;
use leptos_use::use_cookie;
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

#[component]
pub fn Frontend() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // Defining a session_token signal using leptos_use::use_cookie to track the presence of the token that axum-login creates
    // Defining an authenticated resource and a corresponding server function
    // Setting a route condition that checks the result of authenticated
    let (cookie, _) = use_cookie::<String, FromToStringCodec>("id");
    let user_profile = Resource::new(cookie, |_| async move {
        auth::authenticate().await
    });
    let user_profile = move || match user_profile.get() {
        Some(re) => match re {
            Ok(_) => true,
            Err(e) => {
                let err = e.to_string();
                log!("no user: {err:#?}");
                false
            },
        },
        None => false,
    };
    provide_context(PublicUserProfile{authenticated: user_profile()});
    let user_profile = move || Some(user_profile());


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
                        condition=user_profile
                        redirect_path=|| "/login"
                    />
                    <Route path=StaticSegment("/register") view=auth::RegisterPage />
                    <Route path=StaticSegment("/login") view=auth::LoginPage />
                </Routes>
            </main>
        </Router>
    }
}
