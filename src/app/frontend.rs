use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes, ProtectedRoute},
    StaticSegment,
    path,
};
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

#[server(Authenticate)]
pub async fn authenticate() -> Result<(), ServerFnError> {
    use leptos_axum::extract;
    use axum_login::AuthSession;
    use crate::auth::Backend;

    let auth_session: AuthSession<Backend> = extract().await?;

    
    
    match auth_session.user {
        Some(user) => {
            dbg!(user.user_name);
            Ok(())
        },
        None => Err(ServerFnError::ServerError("UNAUTHORIZED".to_string())),
    }
}

#[component]
pub fn Frontend() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (cookie, _) = use_cookie::<String, FromToStringCodec>("id");
    let authenticated = Resource::new(cookie, |_| async move {
        authenticate().await.is_ok()
    });
    provide_context(authenticated);

    view! {
        <Stylesheet id="leptos" href="pkg/taggit.css"/>
        <Title text="Welcome to Leptos"/>

        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("/") view=home_page::HomePage/>
                    <ProtectedRoute 
                        path=StaticSegment("create") 
                        view=create_note_page::CreateNotePage
                        condition=move || authenticated.get()
                        redirect_path=|| "/login"
                    />
                    <Route path=StaticSegment("/register") view=auth::RegisterPage />
                    <Route path=StaticSegment("/login") view=auth::LoginPage />
                    <Route path=path!("/view/note/:id") view=view::NoteViewPage />
                </Routes>
            </main>
        </Router>
    }
}
