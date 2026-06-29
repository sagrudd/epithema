//! Minimal HTTP client seam for provider-backed retrieval.

use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

use epithema_diagnostics::{ErrorCategory, PlatformError};

/// Minimal GET request description for provider adapters.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpRequest {
    /// Target URL.
    pub url: String,
    /// Optional `Accept` header value.
    pub accept: Option<String>,
    /// User agent string.
    pub user_agent: String,
    /// Optional byte offset used to resume a direct download.
    pub range_start: Option<u64>,
}

impl HttpRequest {
    /// Creates a request for the supplied URL.
    #[must_use]
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            accept: None,
            user_agent: format!("epithema/{}", env!("CARGO_PKG_VERSION")),
            range_start: None,
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

    /// Requests bytes from the supplied offset onward.
    #[must_use]
    pub fn with_range_start(mut self, range_start: u64) -> Self {
        self.range_start = Some(range_start);
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

/// Minimal byte response used by provider-backed file materialization.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpBytesResponse {
    /// HTTP status code.
    pub status: u16,
    /// Response body bytes.
    pub body: Vec<u8>,
    /// Optional content type.
    pub content_type: Option<String>,
}

/// Response metadata from a provider-backed file download.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpDownloadResponse {
    /// HTTP status code.
    pub status: u16,
    /// Number of bytes written to the target path.
    pub bytes_written: u64,
    /// Server-reported content length when available.
    pub content_length: Option<u64>,
    /// Byte offset that was reused from an existing local partial file.
    pub resumed_from: Option<u64>,
    /// Optional content type.
    pub content_type: Option<String>,
}

/// Download progress event kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HttpDownloadProgressState {
    /// The response body is about to be streamed.
    Started,
    /// More bytes were written to disk.
    Advanced,
    /// The transfer has reached the expected size and the external downloader is finalizing.
    Finalizing,
    /// The local file is being read for checksum verification.
    Verifying,
    /// The response body has been fully written.
    Finished,
}

/// Progress event emitted while streaming a provider download to disk.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpDownloadProgress {
    /// Event kind.
    pub state: HttpDownloadProgressState,
    /// Download URL.
    pub url: String,
    /// Local path being written.
    pub path: PathBuf,
    /// Bytes written so far.
    pub bytes_downloaded: u64,
    /// Server-reported content length when available.
    pub total_bytes: Option<u64>,
}

impl HttpBytesResponse {
    /// Creates a byte response payload.
    #[must_use]
    pub fn new(status: u16, body: impl Into<Vec<u8>>) -> Self {
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

    /// Executes a GET request and returns the response body as bytes.
    fn get_bytes(&self, request: &HttpRequest) -> Result<HttpBytesResponse, PlatformError> {
        let response = self.get_text(request)?;
        Ok(HttpBytesResponse {
            status: response.status,
            body: response.body.into_bytes(),
            content_type: response.content_type,
        })
    }

    /// Executes a GET request and streams a successful response body to `path`.
    fn download_to_path(
        &self,
        request: &HttpRequest,
        path: &Path,
        progress: Option<&dyn Fn(HttpDownloadProgress)>,
    ) -> Result<HttpDownloadResponse, PlatformError> {
        let response = self.get_bytes(request)?;
        let bytes_written = u64::try_from(response.body.len()).map_err(|error| {
            PlatformError::new(
                ErrorCategory::Invocation,
                "provider response body length could not be represented as u64",
            )
            .with_code("providers.http.body_size_overflow")
            .with_detail(error.to_string())
        })?;
        if !(200..300).contains(&response.status) {
            return Ok(HttpDownloadResponse {
                status: response.status,
                bytes_written: 0,
                content_length: None,
                resumed_from: None,
                content_type: response.content_type,
            });
        }
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|error| {
                PlatformError::new(
                    ErrorCategory::Invocation,
                    "failed to create provider download directory",
                )
                .with_code("providers.http.download_directory_failed")
                .with_detail(error.to_string())
            })?;
        }
        if let Some(progress) = progress {
            progress(HttpDownloadProgress {
                state: HttpDownloadProgressState::Started,
                url: request.url.clone(),
                path: path.to_path_buf(),
                bytes_downloaded: request.range_start.unwrap_or(0),
                total_bytes: Some(request.range_start.unwrap_or(0) + bytes_written),
            });
        }
        let mut file = if request.range_start.unwrap_or(0) > 0 {
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .map_err(|error| {
                    PlatformError::new(
                        ErrorCategory::Invocation,
                        "failed to open provider download file for append",
                    )
                    .with_code("providers.http.download_append_failed")
                    .with_detail(error.to_string())
                })?
        } else {
            fs::File::create(path).map_err(|error| {
                PlatformError::new(
                    ErrorCategory::Invocation,
                    "failed to create provider download file",
                )
                .with_code("providers.http.download_create_failed")
                .with_detail(error.to_string())
            })?
        };
        file.write_all(&response.body).map_err(|error| {
            PlatformError::new(
                ErrorCategory::Invocation,
                "failed to write provider download body",
            )
            .with_code("providers.http.download_write_failed")
            .with_detail(error.to_string())
        })?;
        if let Some(progress) = progress {
            progress(HttpDownloadProgress {
                state: HttpDownloadProgressState::Finished,
                url: request.url.clone(),
                path: path.to_path_buf(),
                bytes_downloaded: request.range_start.unwrap_or(0) + bytes_written,
                total_bytes: Some(request.range_start.unwrap_or(0) + bytes_written),
            });
        }
        Ok(HttpDownloadResponse {
            status: response.status,
            bytes_written: request.range_start.unwrap_or(0) + bytes_written,
            content_length: Some(request.range_start.unwrap_or(0) + bytes_written),
            resumed_from: request.range_start.filter(|offset| *offset > 0),
            content_type: response.content_type,
        })
    }
}

impl<T: ProviderHttpClient + ?Sized> ProviderHttpClient for &T {
    fn get_text(&self, request: &HttpRequest) -> Result<HttpResponse, PlatformError> {
        (**self).get_text(request)
    }

    fn get_bytes(&self, request: &HttpRequest) -> Result<HttpBytesResponse, PlatformError> {
        (**self).get_bytes(request)
    }

    fn download_to_path(
        &self,
        request: &HttpRequest,
        path: &Path,
        progress: Option<&dyn Fn(HttpDownloadProgress)>,
    ) -> Result<HttpDownloadResponse, PlatformError> {
        (**self).download_to_path(request, path, progress)
    }
}

/// `reqwest`-backed blocking HTTP client for provider retrieval.
#[derive(Clone, Debug)]
pub struct ReqwestHttpClient {
    client: reqwest::blocking::Client,
    text_timeout: Option<Duration>,
}

impl ReqwestHttpClient {
    /// Creates a client with a conservative default timeout.
    pub fn new() -> Result<Self, PlatformError> {
        Self::with_timeout(Duration::from_secs(20))
    }

    /// Creates a client with an explicit timeout.
    pub fn with_timeout(timeout: Duration) -> Result<Self, PlatformError> {
        let client = reqwest::blocking::Client::builder()
            .connect_timeout(timeout)
            .build()
            .map_err(|error| {
                PlatformError::new(
                    ErrorCategory::Configuration,
                    "failed to construct provider HTTP client",
                )
                .with_code("providers.http.client_build_failed")
                .with_detail(error.to_string())
            })?;

        Ok(Self {
            client,
            text_timeout: Some(timeout),
        })
    }
}

impl ProviderHttpClient for ReqwestHttpClient {
    fn get_text(&self, request: &HttpRequest) -> Result<HttpResponse, PlatformError> {
        let mut builder = self
            .client
            .get(&request.url)
            .header("User-Agent", &request.user_agent);
        if let Some(timeout) = self.text_timeout {
            builder = builder.timeout(timeout);
        }
        if let Some(accept) = &request.accept {
            builder = builder.header("Accept", accept);
        }
        if let Some(range_start) = request.range_start {
            builder = builder.header(reqwest::header::RANGE, format!("bytes={range_start}-"));
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

    fn get_bytes(&self, request: &HttpRequest) -> Result<HttpBytesResponse, PlatformError> {
        let mut builder = self
            .client
            .get(&request.url)
            .header("User-Agent", &request.user_agent);
        if let Some(accept) = &request.accept {
            builder = builder.header("Accept", accept);
        }
        if let Some(range_start) = request.range_start {
            builder = builder.header(reqwest::header::RANGE, format!("bytes={range_start}-"));
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
        let body = response.bytes().map_err(|error| {
            PlatformError::new(
                ErrorCategory::Invocation,
                "provider response body could not be read as bytes",
            )
            .with_code("providers.http.body_read_failed")
            .with_detail(error.to_string())
        })?;

        Ok(HttpBytesResponse {
            status,
            body: body.to_vec(),
            content_type,
        })
    }

    fn download_to_path(
        &self,
        request: &HttpRequest,
        path: &Path,
        progress: Option<&dyn Fn(HttpDownloadProgress)>,
    ) -> Result<HttpDownloadResponse, PlatformError> {
        let mut builder = self
            .client
            .get(&request.url)
            .header("User-Agent", &request.user_agent);
        if let Some(accept) = &request.accept {
            builder = builder.header("Accept", accept);
        }

        let mut response = builder.send().map_err(|error| {
            PlatformError::new(
                ErrorCategory::Invocation,
                "provider request failed during transport",
            )
            .with_code("providers.http.transport_failed")
            .with_detail(error.to_string())
        })?;

        let status = response.status().as_u16();
        let content_length = response.content_length();
        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .map(ToOwned::to_owned);
        if !(200..300).contains(&status) {
            return Ok(HttpDownloadResponse {
                status,
                bytes_written: 0,
                content_length,
                resumed_from: None,
                content_type,
            });
        }

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|error| {
                PlatformError::new(
                    ErrorCategory::Invocation,
                    "failed to create provider download directory",
                )
                .with_code("providers.http.download_directory_failed")
                .with_detail(error.to_string())
            })?;
        }
        let requested_range_start = request.range_start.unwrap_or(0);
        let range_was_accepted = requested_range_start > 0 && status == 206;
        let resumed_from = range_was_accepted.then_some(requested_range_start);
        let starting_size = resumed_from.unwrap_or(0);
        let expected_total = content_length.map(|length| length + starting_size);

        let mut file = if range_was_accepted {
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .map_err(|error| {
                    PlatformError::new(
                        ErrorCategory::Invocation,
                        "failed to open provider download file for append",
                    )
                    .with_code("providers.http.download_append_failed")
                    .with_detail(error.to_string())
                })?
        } else {
            fs::File::create(path).map_err(|error| {
                PlatformError::new(
                    ErrorCategory::Invocation,
                    "failed to create provider download file",
                )
                .with_code("providers.http.download_create_failed")
                .with_detail(error.to_string())
            })?
        };

        if let Some(progress) = progress {
            progress(HttpDownloadProgress {
                state: HttpDownloadProgressState::Started,
                url: request.url.clone(),
                path: path.to_path_buf(),
                bytes_downloaded: starting_size,
                total_bytes: expected_total,
            });
        }

        let mut bytes_written = starting_size;
        let mut buffer = vec![0u8; 1024 * 1024];
        loop {
            let bytes_read = response.read(&mut buffer).map_err(|error| {
                PlatformError::new(
                    ErrorCategory::Invocation,
                    "provider response body could not be streamed to disk",
                )
                .with_code("providers.http.body_stream_failed")
                .with_detail(error.to_string())
            })?;
            if bytes_read == 0 {
                break;
            }
            file.write_all(&buffer[..bytes_read]).map_err(|error| {
                PlatformError::new(
                    ErrorCategory::Invocation,
                    "failed to write provider download chunk",
                )
                .with_code("providers.http.download_write_failed")
                .with_detail(error.to_string())
            })?;
            bytes_written = bytes_written
                .checked_add(u64::try_from(bytes_read).map_err(|error| {
                    PlatformError::new(
                        ErrorCategory::Invocation,
                        "provider download chunk length could not be represented as u64",
                    )
                    .with_code("providers.http.body_size_overflow")
                    .with_detail(error.to_string())
                })?)
                .ok_or_else(|| {
                    PlatformError::new(
                        ErrorCategory::Invocation,
                        "provider download byte count overflowed u64",
                    )
                    .with_code("providers.http.body_size_overflow")
                })?;
            if let Some(progress) = progress {
                progress(HttpDownloadProgress {
                    state: HttpDownloadProgressState::Advanced,
                    url: request.url.clone(),
                    path: path.to_path_buf(),
                    bytes_downloaded: bytes_written,
                    total_bytes: expected_total,
                });
            }
        }
        file.flush().map_err(|error| {
            PlatformError::new(
                ErrorCategory::Invocation,
                "failed to flush provider download file",
            )
            .with_code("providers.http.download_flush_failed")
            .with_detail(error.to_string())
        })?;

        if let Some(progress) = progress {
            progress(HttpDownloadProgress {
                state: HttpDownloadProgressState::Finished,
                url: request.url.clone(),
                path: path.to_path_buf(),
                bytes_downloaded: bytes_written,
                total_bytes: expected_total,
            });
        }

        Ok(HttpDownloadResponse {
            status,
            bytes_written,
            content_length: expected_total,
            resumed_from,
            content_type,
        })
    }
}
