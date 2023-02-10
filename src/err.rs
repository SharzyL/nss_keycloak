use std::fmt::{Debug, Display, Formatter};

pub enum KeycloakError {
    RequestError(reqwest::Error),
    JSONParseError(String, serde_json::Error),
    DataError(String),
}

impl Display for KeycloakError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            KeycloakError::RequestError(err) => write!(f, "Keycloak request error: {:?}", err),
            KeycloakError::JSONParseError(str, err) => write!(f, "Keycloak error occurs parsing `{}`: {:?}", str, err),
            KeycloakError::DataError(err) => write!(f, "Keycloak data error: {:?}", err)
        }
    }
}

impl Debug for KeycloakError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl From<reqwest::Error> for KeycloakError {
    fn from(err: reqwest::Error) -> Self {
        KeycloakError::RequestError(err)
    }
}
