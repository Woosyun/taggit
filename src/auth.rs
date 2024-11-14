#![allow(unused)]

use async_trait::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use mongodb::Database;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use crate::{
    db::DB, 
    user::{
        User, 
        UserService
    },
};
use axum_login::AuthManagerLayerBuilder;
use tower_sessions::{
    ExpiredDeletion,
    Expiry,
    SessionManagerLayer,
};

#[derive(Debug, Clone)]
pub struct SessionUser {
    user_name: String,
    user_id: String,
    password_hash: Vec<u8>,
}



impl SessionUser {
    pub fn hash_pw(pw: String) -> Vec<u8> {
        let mut hasher = Sha256::new();       // Create a SHA256 hasher instance
        hasher.update(pw.as_bytes());    // Feed the password bytes to the hasher
        let result = hasher.finalize();        // Get the hash result
        result.to_vec()   
    }
    pub fn from_user(user: User) -> Self {
        Self {
            user_name: user.user_name,
            user_id: user.user_id,
            password_hash: Self::hash_pw(user.password),
        }
    }
}

impl AuthUser for SessionUser {
    type Id = String;

    fn id(&self) -> Self::Id {
        self.user_id.clone()
    }

    fn session_auth_hash(&self) -> &[u8] {
        &self.password_hash
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    user_id: String,
    password: String,
}

impl Credentials {
    pub fn new(id: String, pw: String) -> Result<Self, String> {
        Ok(Self {
            user_id: id,
            password: pw,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Backend {
    users: UserService,
}

impl Backend {
    pub async fn new(db: &Database) -> Self {
        Self {
            users: UserService::new(db),
        }
    }

    //TODO: implement register, find_one
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = SessionUser;
    type Credentials = Credentials;
    type Error = std::convert::Infallible;

    async fn authenticate(
        &self,
        Credentials { user_id, password }: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user: Option<SessionUser> = match self.users.find_one(user_id).await {
            Ok(Some(user)) => if user.password == password {
                Some(Self::User::from_user(user))
            } else {
                None
            },
            _ => None,
        };

        Ok(user)
    }

    async fn get_user(
        &self,
        user_id: &UserId<Self>,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user = match self.users.find_one(user_id.clone()).await {
            Ok(Some(user)) => Some(Self::User::from_user(user)),
            _ => None
        };

        Ok(user)
    }
}