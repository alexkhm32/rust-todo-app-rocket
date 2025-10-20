use crate::domain::{
    AccountRepository, AccountService as AccountServiceTrait, AuthToken, AuthTokenGenerator,
    CreateAccountRequest, Error, LoginRequest, PasswordHasher, RegisterRequest,
};

pub struct AccountService<R, H, G>
where
    R: AccountRepository,
    H: PasswordHasher,
    G: AuthTokenGenerator,
{
    repository: R,
    password_hasher: H,
    token_generator: G,
}

impl<R, H, G> AccountService<R, H, G>
where
    R: AccountRepository,
    H: PasswordHasher,
    G: AuthTokenGenerator,
{
    pub fn new(repository: R, password_hasher: H, token_generator: G) -> Self {
        Self {
            repository,
            password_hasher,
            token_generator: token_generator,
        }
    }
}

#[async_trait]
impl<R, H, G> AccountServiceTrait for AccountService<R, H, G>
where
    R: AccountRepository,
    H: PasswordHasher,
    G: AuthTokenGenerator,
{
    async fn register(&self, request: RegisterRequest) -> Result<AuthToken, Error> {
        let hashed_password = self.password_hasher.hash(request.password)?;
        let create_account_request = CreateAccountRequest {
            login: request.login,
            password: hashed_password,
        };
        let account = self.repository.create(create_account_request).await?;
        self.token_generator.generate(account.id)
    }

    async fn login(&self, request: LoginRequest) -> Result<AuthToken, Error> {
        let account = self.repository.get_by_login(request.login).await?;
        let ok = self
            .password_hasher
            .verify(request.password, account.password);
        if !ok {
            return Err(Error::Unknown("Unauthorized".to_string()));
        }
        self.token_generator.generate(account.id)
    }

    async fn authorize(&self, token: String) -> Result<i32, Error> {
        self.token_generator.parse(token)
    }
}
