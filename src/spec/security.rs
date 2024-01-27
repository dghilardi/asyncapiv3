use std::collections::HashMap;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum SecurityScheme {
    UserPassword {
        /// A short description for security scheme. CommonMark syntax MAY be used for rich text representation.
        description: Option<String>,
    },
    ApiKey {
        /// A short description for security scheme. CommonMark syntax MAY be used for rich text representation.
        description: Option<String>,
        /// The location of the API key. Valid values are "user" and "password" for apiKey and "query", "header" or "cookie" for httpApiKey.
        #[serde(rename = "in")]
        location: ApiKeyLocation,
    },
    X509 {
        /// A short description for security scheme. CommonMark syntax MAY be used for rich text representation.
        description: Option<String>,
    },
    SymmetricEncryption {
        /// A short description for security scheme. CommonMark syntax MAY be used for rich text representation.
        description: Option<String>,
    },
    AsymmetricEncryption {
        /// A short description for security scheme. CommonMark syntax MAY be used for rich text representation.
        description: Option<String>,
    },
    HttpApiKey {
        /// A short description for security scheme. CommonMark syntax MAY be used for rich text representation.
        description: Option<String>,
        /// The name of the header, query or cookie parameter to be used.
        name: String,
        /// The location of the API key. Valid values are "user" and "password" for apiKey and "query", "header" or "cookie" for httpApiKey.
        #[serde(rename = "in")]
        location: HttpApiKeyLocation,
    },
    #[serde(rename_all="camelCase")]
    Http {
        /// A short description for security scheme. CommonMark syntax MAY be used for rich text representation.
        description: Option<String>,
        /// The name of the HTTP Authorization scheme to be used in the [Authorization header as defined in RFC7235](https://tools.ietf.org/html/rfc7235#section-5.1).
        scheme: String,
        /// A hint to the client to identify how the bearer token is formatted. Bearer tokens are usually generated by an authorization server, so this information is primarily for documentation purposes.
        bearer_format: Option<String>,
    },
    Oauth2 {
        /// A short description for security scheme. CommonMark syntax MAY be used for rich text representation.
        description: Option<String>,
        /// An object containing configuration information for the flow types supported.
        flows: OAuthFlows,
        /// List of the needed scope names. An empty array means no scopes are needed.
        #[serde(default)]
        scopes: Vec<String>,
    },
    #[serde(rename_all="camelCase")]
    OpenIdConnect {
        /// A short description for security scheme. CommonMark syntax MAY be used for rich text representation.
        description: Option<String>,
        /// OpenId Connect URL to discover OAuth2 configuration values. This MUST be in the form of an absolute URL.
        open_id_connect_url: String,
        /// List of the needed scope names. An empty array means no scopes are needed.
        #[serde(default)]
        scopes: Vec<String>,
    },
    Plain {
        /// A short description for security scheme. CommonMark syntax MAY be used for rich text representation.
        description: Option<String>,
    },
    ScramSha256 {
        /// A short description for security scheme. CommonMark syntax MAY be used for rich text representation.
        description: Option<String>,
    },
    ScramSha512 {
        /// A short description for security scheme. CommonMark syntax MAY be used for rich text representation.
        description: Option<String>,
    },
    Gssapi {
        /// A short description for security scheme. CommonMark syntax MAY be used for rich text representation.
        description: Option<String>,
    },
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OAuthFlows {
    /// Configuration for the OAuth Implicit flow.
    #[serde(rename_all="camelCase")]
    Implicit {
        /// The authorization URL to be used for this flow. This MUST be in the form of an absolute URL.
        authorization_url: String,
        refresh_url: Option<String>,
        /// The available scopes for the OAuth2 security scheme. A map between the scope name and a short description for it.
        available_scopes: HashMap<String, String>,
    },
    /// Configuration for the OAuth Resource Owner Protected Credentials flow.
    #[serde(rename_all="camelCase")]
    Password {
        /// The token URL to be used for this flow. This MUST be in the form of an absolute URL.
        token_url: String,
        /// The URL to be used for obtaining refresh tokens. This MUST be in the form of an absolute URL.
        refresh_url: Option<String>,
        /// The available scopes for the OAuth2 security scheme. A map between the scope name and a short description for it.
        available_scopes: HashMap<String, String>,
    },
    /// Configuration for the OAuth Client Credentials flow.
    #[serde(rename_all="camelCase")]
    ClientCredentials {
        /// The token URL to be used for this flow. This MUST be in the form of an absolute URL.
        token_url: String,
        /// The URL to be used for obtaining refresh tokens. This MUST be in the form of an absolute URL.
        refresh_url: Option<String>,
        /// The available scopes for the OAuth2 security scheme. A map between the scope name and a short description for it.
        available_scopes: HashMap<String, String>,
    },
    /// Configuration for the OAuth Authorization Code flow.
    #[serde(rename_all="camelCase")]
    AuthorizationCode {
        /// The authorization URL to be used for this flow. This MUST be in the form of an absolute URL.
        authorization_url: String,
        /// The token URL to be used for this flow. This MUST be in the form of an absolute URL.
        token_url: String,
        /// The URL to be used for obtaining refresh tokens. This MUST be in the form of an absolute URL.
        refresh_url: Option<String>,
        /// The available scopes for the OAuth2 security scheme. A map between the scope name and a short description for it.
        available_scopes: HashMap<String, String>,
    },
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ApiKeyLocation { User, Password }

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HttpApiKeyLocation { Query, Header, Cookie }