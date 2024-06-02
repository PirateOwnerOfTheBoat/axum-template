use axum::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use deadpool_postgres::{Pool, PoolError};
use password_auth::verify_password;
use tokio::task;

use crate::cornucopia::queries::users;

#[derive(Clone)]
pub struct User {
    id: i64,
    pub username: String,
    password_hash: String,
}

impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("username", &self.username)
            .field("password_hash", &"[redacted]")
            .finish()
    }
}

impl AuthUser for User {
    type Id = i64;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password_hash.as_bytes()
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Clone)]
pub struct Backend {
    pool: Pool,
}

impl Backend {
    pub fn new(pool: Pool) -> Self {
        Backend { pool }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    PostgresError(#[from] tokio_postgres::Error),

    #[error(transparent)]
    TaskJoin(#[from] task::JoinError),

    #[error(transparent)]
    PoolError(#[from] PoolError),
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = Error;

    async fn authenticate(&self, creds: Credentials) -> Result<Option<Self::User>, Self::Error> {
        let client = self.pool.get().await?;
        let user: Option<User> = users::find_by_username()
            .bind(&client, &creds.username)
            .opt()
            .await?
            .map(|user| User {
                id: user.id,
                username: user.username,
                password_hash: user.password_hash,
            });

        task::spawn_blocking(|| {
            Ok(user.filter(|user| verify_password(creds.password, &user.password_hash).is_ok()))
        })
        .await?
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let client = self.pool.get().await?;
        let user: Option<User> =
            users::find_by_id()
                .bind(&client, user_id)
                .opt()
                .await?
                .map(|user| User {
                    id: user.id,
                    username: user.username,
                    password_hash: user.password_hash,
                });

        Ok(user)
    }
}

pub type AuthSession = axum_login::AuthSession<Backend>;
