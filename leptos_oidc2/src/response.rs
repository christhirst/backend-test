/*
* The MIT License (MIT)
*
* Copyright (c) 2023 Daniél Kerkmann <daniel@kerkmann.dev>
*
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*/

use leptos_router::{Params, ParamsError, ParamsMap};
use serde::{Deserialize, Serialize};

/// An enumeration representing different callback responses during the
/// authentication process.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum CallbackResponse {
    SuccessLogin(SuccessCallbackResponse),
    SuccessLogout(SuccessLogoutResponse),
    Error(ErrorResponse),
}

/// A structure representing a successful login callback response.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct SuccessCallbackResponse {
    pub session_state: Option<String>,
    pub code: String,
}

/// A structure representing a successful logout callback response.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct SuccessLogoutResponse {
    pub destroy_session: bool,
}

/// An enumeration representing the response to token requests, including
/// success and error responses.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TokenResponse {
    Success(SuccessTokenResponse),
    Error(ErrorResponse),
}

/// A structure representing a successful token response.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct SuccessTokenResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_expires_in: Option<i64>,
    pub refresh_token: String,
    pub token_type: Option<String>,
    pub id_token: String,
    #[serde(rename = "not-before-policy")]
    pub not_before_policy: Option<i64>,
    pub session_state: Option<String>,
    pub scope: Option<String>,
}

/// A structure representing an error response during the authentication
/// process.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub error_description: String,
}

/// A trait for converting parameters from a map to a structure for
/// `SuccessCallbackResponse`.
impl Params for SuccessCallbackResponse {
    fn from_map(map: &ParamsMap) -> Result<Self, ParamsError> {
        if let (session_state, Some(code)) = (map.get("session_state"), map.get("code")) {
            return Ok(SuccessCallbackResponse {
                session_state: session_state.cloned(),
                code: code.clone(),
            });
        }
        Err(ParamsError::MissingParam(
            "Missing parameter 'code'".to_string(),
        ))
    }
}

/// A trait for converting parameters from a map to a structure for
/// `SuccessLogoutResponse`.
impl Params for SuccessLogoutResponse {
    fn from_map(map: &ParamsMap) -> Result<Self, ParamsError> {
        if let Some(destroy_session) = map.get("destroy_session") {
            return Ok(SuccessLogoutResponse {
                destroy_session: destroy_session.parse().unwrap_or_default(),
            });
        }
        Err(ParamsError::MissingParam(
            "Missing parameter 'destroy_session'".to_string(),
        ))
    }
}

/// A trait for converting parameters from a map to a structure for
/// `ErrorResponse`.
impl Params for ErrorResponse {
    fn from_map(map: &ParamsMap) -> Result<Self, ParamsError> {
        if let (Some(error), Some(error_description)) =
            (map.get("error"), map.get("error_description"))
        {
            return Ok(ErrorResponse {
                error: error.clone(),
                error_description: error_description.clone(),
            });
        }
        Err(ParamsError::MissingParam(
            "Missing parameter 'error' and 'error_description'".to_string(),
        ))
    }
}

/// A trait for converting parameters from a map to a structure for
/// `CallbackResponse`.
impl Params for CallbackResponse {
    fn from_map(map: &ParamsMap) -> Result<Self, ParamsError> {
        if let Ok(response) = SuccessCallbackResponse::from_map(map) {
            return Ok(CallbackResponse::SuccessLogin(response));
        } else if let Ok(reponse) = SuccessLogoutResponse::from_map(map) {
            return Ok(CallbackResponse::SuccessLogout(reponse));
        } else if let Ok(reponse) = ErrorResponse::from_map(map) {
            return Ok(CallbackResponse::Error(reponse));
        }

        Err(ParamsError::MissingParam(
            "Missing parameter 'session_state' and 'code' or 'error' and 'error_description'"
                .to_string(),
        ))
    }
}
