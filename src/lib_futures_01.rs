//! Internal only library implementation based on futures v0.1.0

use std::env;
use std::time::Duration;

use futures01::Future;
use reqwest09::r#async::Client as ReqwestClient;
use reqwest09::Error as ReqwestError;
use serde_json::json;

use crate::types::*;

// TODO: add `Error` type and improve error handling
// TODO: make `AccessToken` type to differentiate from `PublicToken` etc.
// TODO: avoid allocation for all URLs
// TODO: determine public (& private) organization of modules/types etc.
// TODO: allow overriding `client_id` etc. for requests?

/// **[Plaid](https://plaid.com/docs) API client**.
///
/// See official documentation at: [https://plaid.com/docs](https://plaid.com/docs).
#[allow(dead_code)]
#[derive(Clone)]
pub struct Client {
    client_id: String,
    secret: Secret,
    url: String,
    client: ReqwestClient,
}

impl Client {
    /// Creates a new `Client`.
    #[allow(dead_code)]
    pub fn new<C, S>(client_id: C, secret: S, environment: Environment) -> Client
    where
        C: Into<String>,
        S: Into<Secret>,
    {
        Client {
            client_id: client_id.into(),
            secret: secret.into(),
            url: format!("https://{}.plaid.com", environment),
            client: ReqwestClient::builder()
                .connect_timeout(Duration::from_secs(30))
                .build()
                .expect("could not create Reqwest client"),
        }
    }

    /// Creates a new `Client` from the following environment variables:
    /// - `PLAID_CLIENT_ID`
    /// - `PLAID_SECRET`
    /// - `PLAID_ENVIRONMENT`
    #[allow(dead_code)]
    pub fn from_env() -> Result<Client, Box<dyn std::error::Error>> {
        let client = Client::new(
            env::var("PLAID_CLIENT_ID")?,
            env::var("PLAID_SECRET")?,
            env::var("PLAID_ENVIRONMENT")?.parse()?,
        );
        Ok(client)
    }

    /// Create a test Item
    ///
    /// [/sandbox/public_token/create]
    ///
    /// Use the [/sandbox/public_token/create] endpoint to create a valid
    /// public_token for an arbitrary institution ID, initial products, and test
    /// credentials. The created public_token maps to a new Sandbox Item. You
    /// can then call [/item/public_token/exchange] to exchange the
    /// `public_token` for an access_token and perform all API actions.
    /// [/sandbox/public_token/create] can also be used with the [`user_custom`]
    /// test username to generate a test account with custom data.
    ///
    /// [/sandbox/public_token/create]: https://plaid.com/docs/api/sandbox/#sandboxpublic_tokencreate
    /// [/item/public_token/exchange]: https://plaid.com/docs/api/tokens/#itempublic_tokenexchange
    /// [`user_custom`]: https://plaid.com/docs/sandbox/user-custom/
    #[allow(dead_code)]
    pub fn sandbox_create_public_token(
        &self,
        request: &SandboxCreatePublicTokenRequest,
    ) -> impl Future<Item = CreatePublicTokenResponse, Error = ReqwestError> {
        // TODO: figure out a better way to do this...
        let mut body = json!(request);
        body["client_id"] = json!(&self.client_id);
        body["secret"] = json!(&self.secret);

        self.client
            .post(&format!("{}/sandbox/public_token/create", self.url))
            .json(&body)
            .send()
            .and_then(|res| res.error_for_status())
            .and_then(|mut res| res.json())
            .from_err()
    }

    /// Create Link Token
    ///
    /// [/link/token/create]
    ///
    /// Creates a `link_token`, which is required as a parameter when
    /// initializing Link. Once Link has been initialized, it returns a
    /// `public_token`, which can then be exchanged for an `access_token` via
    /// [/item/public_token/exchange] as part of the [main Link flow].
    ///
    /// A `link_token` generated by [/link/token/create] is also used to
    /// initialize other Link flows, such as the update mode flow for tokens
    /// with expired credentials, or the Payment Initiation (Europe) flow.
    ///
    /// [/link/token/create]: https://plaid.com/docs/api/tokens/#linktokencreate
    /// [/item/public_token/exchange]: https://plaid.com/docs/api/tokens/#itempublic_tokenexchange
    /// [main Link flow]: https://plaid.com/docs/link/#link-flow
    #[allow(dead_code)]
    pub fn create_link_token(
        &self,
    ) -> impl Future<Item = CreateLinkTokenResponse, Error = ReqwestError> {
        let body = json!({
            "client_id": &self.client_id,
            "secret": &self.secret,
            "institution_id": "ins_1",
            "initial_products": ["auth", "identity"]
        });

        self.client
            .post(&format!("{}/link/token/create", self.url))
            .json(&body)
            .send()
            .and_then(|res| res.error_for_status())
            .and_then(|mut res| res.json())
            .from_err()
    }

    /// Exchange a public token for an access token
    ///
    /// [/item/public_token/exchange]
    ///
    /// Exchanges a `Link` `public_token` for an API a`ccess_token`. `Link`
    /// hands off the `public_token` client-side via the `onSuccess` callback
    /// once a user has successfully created an `Item`. The `public_token` is
    /// ephemeral and expires after 30 minutes.
    ///
    /// The response also includes an `item_id` that should be stored with the
    /// `access_token`. The `item_id` is used to identify an Item in a webhook.
    /// The `item_id` can also be retrieved by making an [/item/get] request.
    ///
    /// [/item/public_token/exchange]: https://plaid.com/docs/api/tokens/#itempublic_tokenexchange
    /// [/item/get]: https://plaid.com/docs/api/items/#itemget
    #[allow(dead_code)]
    pub fn exchange_public_token(
        &self,
        public_token: &str,
    ) -> impl Future<Item = ExchangePublicTokenResponse, Error = ReqwestError> {
        // TODO: make this strongly typed?
        let body = json!({
            "client_id": &self.client_id,
            "secret": &self.secret,
            "public_token": public_token,
        });

        self.client
            .post(&format!("{}/item/public_token/exchange", self.url))
            .json(&body)
            .send()
            .and_then(|res| res.error_for_status())
            .and_then(|mut res| res.json())
            .from_err()
    }

    /// Retrieve accounts
    ///
    /// [/accounts/get]
    ///
    /// Retrieves information for any linked Item. Note that some information is
    /// nullable. Plaid will only return active bank accounts, i.e. accounts
    /// that are not closed and are capable of carrying a balance.
    ///
    /// [/accounts/get]: https://plaid.com/docs/api/accounts/#accountsget
    #[allow(dead_code)]
    pub fn accounts(
        &self,
        access_token: &str,
    ) -> impl Future<Item = AccountsResponse, Error = ReqwestError> {
        // TODO: make this strongly typed?
        let body = json!({
            "client_id": &self.client_id,
            "secret": &self.secret,
            "access_token": access_token,
        });

        self.client
            .post(&format!("{}/accounts/get", self.url))
            .json(&body)
            .send()
            .and_then(|res| res.error_for_status())
            .and_then(|mut res| res.json())
            .from_err()
    }

    /// Fetch real-time balance data
    ///
    /// [/accounts/balance/get]
    ///
    /// Returns the real-time balance for each of an Item's accounts. While
    /// other endpoints may return a balance object, only this endpoint forces
    /// the available and current balance fields to be refreshed rather than
    /// cached. This endpoint can be used for existing Items that were added via
    /// any of Plaid’s other products. This endpoint can be used as long as Link
    /// has been initialized with any other product, `balance` itself is not a
    /// product that can be used to initialize Link.
    ///
    /// [/accounts/balance/get]: https://plaid.com/docs/api/products/#accountsbalanceget
    #[allow(dead_code)]
    pub fn balance(
        &self,
        access_token: &str,
        options: BalanceRequestOptions,
    ) -> impl Future<Item = AccountsResponse, Error = ReqwestError> {
        // TODO: make this strongly typed?
        let body = json!({
            "client_id": &self.client_id,
            "secret": &self.secret,
            "access_token": access_token,
            "options": options,
        });

        self.client
            .post(&format!("{}/accounts/balance/get", self.url))
            .json(&body)
            .send()
            .and_then(|res| res.error_for_status())
            .and_then(|mut res| res.json())
            .from_err()
    }

    /// Fetch auth data
    ///
    /// [/auth/get]
    ///
    /// Returns the bank account and bank identification numbers (such as
    /// routing numbers, for US accounts) associated with an Item's checking and
    /// savings accounts, along with high-level account data and balances when
    /// available.
    ///
    /// *Note*: This request may take some time to complete if auth was not
    /// specified as an initial product when creating the Item. This is because
    /// Plaid must communicate directly with the institution to retrieve the
    /// data.
    ///
    /// [/auth/get]: https://plaid.com/docs/api/products/#authget
    #[allow(dead_code)]
    pub fn auth(
        &self,
        access_token: &str,
        options: AuthRequestOptions,
    ) -> impl Future<Item = AuthResponse, Error = ReqwestError> {
        // TODO: make this strongly typed?
        let body = json!({
            "client_id": &self.client_id,
            "secret": &self.secret,
            "access_token": access_token,
            "options": options,
        });

        self.client
            .post(&format!("{}/auth/get", self.url))
            .json(&body)
            .send()
            .and_then(|res| res.error_for_status())
            .and_then(|mut res| res.json())
            .from_err()
    }

    /// Fetch identity data
    ///
    /// [/identity/get]
    ///
    /// Retrieves various account holder information on file with the financial
    /// institution, including names, emails, phone numbers, and addresses. Only
    /// name data is guaranteed to be returned; other fields will be empty
    /// arrays if not provided by the institution.
    ///
    /// *Note*: This request may take some time to complete if identity was not
    /// specified as an initial product when creating the Item. This is because
    /// Plaid must communicate directly with the institution to retrieve the
    /// data.
    ///
    /// [/identity/get]: https://plaid.com/docs/api/products/#identityget
    #[allow(dead_code)]
    pub fn identity(
        &self,
        access_token: &str,
    ) -> impl Future<Item = AccountsResponse, Error = ReqwestError> {
        // TODO: make this strongly typed?
        let body = json!({
            "client_id": &self.client_id,
            "secret": &self.secret,
            "access_token": access_token,
        });

        self.client
            .post(&format!("{}/identity/get", self.url))
            .json(&body)
            .send()
            .and_then(|res| res.error_for_status())
            .and_then(|mut res| res.json())
            .from_err()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use futures01::future;
    use tokio01::runtime::Runtime;

    #[test]
    fn test_all() -> Result<(), Box<dyn std::error::Error>> {
        let mut rt = Runtime::new().unwrap();

        let client_id = dotenv::var("PLAID_CLIENT_ID")?;
        let secret = dotenv::var("PLAID_SECRET")?;
        let client = Client::new(client_id, secret, Environment::Sandbox);

        // NOTE: we do this once so we don't burn any of our tokens
        let public_token = rt
            .block_on(client.sandbox_create_public_token(&Default::default()))?
            .public_token;
        let token_response = rt.block_on(client.exchange_public_token(&public_token))?;
        let token = token_response.access_token;

        rt.block_on(client.accounts(&token).join4(
            client.balance(&token, Default::default()),
            client.auth(&token, Default::default()),
            client.identity(&token),
        ))?;

        rt.shutdown_now().wait().unwrap();

        Ok(())
    }
}
