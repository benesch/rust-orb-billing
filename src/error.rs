// Copyright Materialize, Inc. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License in the LICENSE file at the
// root of this repository, or online at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt;

use reqwest::StatusCode;

/// An error returned by a [`Client`].
///
/// [`Client`]: crate::Client
#[derive(Debug)]
pub enum Error {
    /// An error in the underlying transport.
    Transport(reqwest::Error),
    /// An error returned by the API.
    Api(ApiError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Transport(e) => write!(f, "orb error: transport: {e}"),
            Error::Api(e) => write!(f, "orb error: api: {e}"),
        }
    }
}

impl std::error::Error for Error {}

/// An error returned by the Orb API.
///
/// For details, see: <https://docs.withorb.com/docs/orb-docs/error-responses>
#[derive(Debug, Clone)]
pub struct ApiError {
    /// The HTTP status code.
    pub status_code: StatusCode,
    /// A brief description of the error message.
    pub title: String,
    /// A human-readable description providing more insight into the error.
    pub detail: Option<String>,
    /// Errors that occurred while validating the request.
    pub validation_errors: Vec<String>,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.title)?;
        if let Some(detail) = &self.detail {
            write!(f, ": {detail}")?;
        }
        write!(f, " (status {})", self.status_code)?;
        write!(
            f,
            "; validation errors: {}",
            self.validation_errors.join(",")
        )
    }
}

impl std::error::Error for ApiError {}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Error {
        Error::Transport(e)
    }
}

impl From<ApiError> for Error {
    fn from(e: ApiError) -> Error {
        Error::Api(e)
    }
}
