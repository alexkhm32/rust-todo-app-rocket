use crate::domain::errors;

pub struct Account {
    pub id: i32,
    #[allow(dead_code)]
    pub login: String,
    pub password: String,
    #[allow(dead_code)]
    pub created_at: time::OffsetDateTime,
}

pub struct RegisterRequest {
    pub login: String,
    pub password: String,
}

pub struct LoginRequest {
    pub login: String,
    pub password: String,
}

pub struct CreateAccountRequest {
    pub login: String,
    pub password: String,
}

pub struct AuthToken {
    pub token: String,
    pub expires: time::OffsetDateTime,
}

pub trait PasswordHasher: Sync + Send {
    fn hash(&self, password: String) -> Result<String, errors::Error>;
    fn verify(&self, password: String, actual: String) -> bool;
}

pub trait AuthTokenGenerator: Sync + Send {
    fn generate(&self, account_id: i32) -> Result<AuthToken, errors::Error>;
    fn parse(&self, token: String) -> Result<i32, errors::Error>;
}

#[async_trait]
pub trait AccountRepository: Send + Sync {
    async fn create(&self, request: CreateAccountRequest) -> Result<Account, errors::Error>;
    async fn get(&self, id: i32) -> Result<Account, errors::Error>;
    async fn get_by_login(&self, login: String) -> Result<Account, errors::Error>;
}

#[async_trait]
pub trait AccountService: Send + Sync {
    async fn register(&self, request: RegisterRequest) -> Result<AuthToken, errors::Error>;
    async fn login(&self, request: LoginRequest) -> Result<AuthToken, errors::Error>;
    async fn authorize(&self, token: String) -> Result<i32, errors::Error>;
}
