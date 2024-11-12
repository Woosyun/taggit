#[derive(Clone)]
pub struct Config {
    pub root_url: String,
    pub google_oauth_client_id: String,
    pub google_oauth_client_secret: String,
    pub github_oauth_client_id: String,
    pub github_oauth_client_secret: String,
}

impl Config {
    pub fn new() -> Self {
        Config {
            root_url: env::var("ROOT_URL").expect("ROOT_URL should be set"),
            google_oauth_client_id: env::var("GOOGLE_OAUTH_CLIENT_ID")
                .expect("GOOGLE_OAUTH_CLIENT_ID should be set"),
            google_oauth_client_secret: env::var("GOOGLE_OAUTH_CLIENT_SECRET")
                .expect("GOOGLE_OAUTH_CLIENT_SECRET must be set"),
            github_oauth_client_id: env::var("GITHUB_OAUTH_CLIENT_ID")
                .expect("GITHUB_OAUTH_CLIENT_ID should be set"),
            github_oauth_client_secret: env::var("GITHUB_OAUTH_CLIENT_SECRET")
                .expect("GITHUB_OAUTH_CLIENT_SECRET should be set"),
        }
    }
}