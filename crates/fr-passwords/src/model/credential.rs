use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub struct Credential {
    pub origin: String,
    pub username: String,
    pub password: String,
}

impl Credential {
    pub fn new(origin: &str, username: &str, password: &str) -> Self {
        Self {
            origin: origin.to_owned(),
            username: username.to_owned(),
            password: password.to_owned(),
        }
    }
}

impl fmt::Debug for Credential {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Credential")
            .field("origin", &self.origin)
            .field("username", &self.username)
            .field("password", &"<hidden>")
            .finish()
    }
}
