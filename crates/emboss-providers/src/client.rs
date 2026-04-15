//! Minimal HTTP client seam for provider-backed retrieval.

use std::time::Duration;

use emboss_diagnostics::{ErrorCategory, PlatformError};

/// Minimal GET request description for provider adapters.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpRequest {
    /// Target URL.
    pub url: String,
    /// Optional `Accept` header value.
    pub accept: Option<String>,
    /// User agent string.
    pub user_agent: String,
}

impl HttpRequest {
    /// Creates a request for the supplied URL.
    #[must_use]
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            accept: None,
            user_agent: format!("emboss-rs/{}", env!("CARGO_PKG_VERSION")),
        }
    }

    /// Attaches an `Accept` header.
    #[must_use]
    pub fn with_accept(mut self, accept: impl Into<String>) -> Self {
        self.accept = Some(accept.into());
        self
    }

    /// Overrides the user agent.
    #[must_use]
    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = user_agent.into();
        self
    }
}

/// Minimal text response used by provider adapters.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpResponse {
    /// HTTP status code.
    pub status: u16,
    /// Response body text.
    pub body: String,
    /// Optional content type.
    pub content_type: Option<String>,
}

impl HttpResponse {
    /// Creates a text response payload.
    #[must_use]
    pub fn new(status: u16, body: impl Into<String>) -> Self {
        Self {
            status,
            body: body.into(),
            content_type: None,
        }
    }

    /// Attaches a content type.
    #[must_use]
    pub fn with_content_type(mut self, content_type: impl Into<String>) -> Self {
        self.content_type = Some(content_type.into());
        self
    }
}

/// Minimal HTTP client interface used by provider adapters.
pub trait ProviderHttpClient {
    /// Executes a GET request and returns the response body as text.
    fn get_text(&self, request: &HttpRequest) -> Result<HttpResponse, PlatformError>;
}

impl<T: ProviderHttpClient + ?Sized> ProviderHttpClient for &T {
    fn get_text(&self, request: &HttpRequest) -> Result<HttpResponse, PlatformError> {
        (**self).get_text(request)
    }
}

/// `reqwest`-backed blocking HTTP client for provider retrieval.
#[derive(Clone, Debug)]
pub struct ReqwestHttpClient {
    client: reqwest::blocking::Client,
}

impl ReqwestHttpClient {
    /// Creates a client with a conservative default timeout.
    pub fn new() -> Result<Self, PlatformError> {
        Self::with_timeout(Duration::from_secs(20))
    }

    /// Creates a client with an explicit timeout.
    pub fn with_timeout(timeout: Duration) -> Result<Self, PlatformError> {
        let client = reqwest::blocking::Client::builder()
            .timeout(timeout)
            .build()
            .map_err(|error| {
                PlatformError::new(
                    ErrorCategory::Configuration,
                    "failed to construct provider HTTP client",
                )
                .with_code("providers.http.client_build_failed")
                .with_detail(error.to_string())
            })?;

        Ok(Self { client })
    }
}

impl ProviderHttpClient for ReqwestHttpClient {
    fn get_text(&self, request: &HttpRequest) -> Result<HttpResponse, PlatformError> {
        let mut builder = self
            .client
            .get(&request.url)
            .header("User-Agent", &request.user_agent);
        if let Some(accept) = &request.accept {
            builder = builder.header("Accept", accept);
        }

        let response = builder.send().map_err(|error| {
            PlatformError::new(
                ErrorCategory::Invocation,
                "provider request failed during transport",
            )
            .with_code("providers.http.transport_failed")
            .with_detail(error.to_string())
        })?;

        let status = response.status().as_u16();
        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .map(ToOwned::to_owned);
        let body = response.text().map_err(|error| {
            PlatformError::new(
                ErrorCategory::Invocation,
                "provider response body could not be decoded as text",
            )
            .with_code("providers.http.body_decode_failed")
            .with_detail(error.to_string())
        })?;

        Ok(HttpResponse {
            status,
            body,
            content_type,
        })
    }
}
