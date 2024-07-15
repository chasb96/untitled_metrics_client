use std::{ops::Deref, sync::OnceLock};
use axum::{async_trait, extract::FromRequestParts, http::{request::Parts, StatusCode}};

use crate::MetricsClient as ClientInner;

static USERS_CLIENT: OnceLock<ClientInner> = OnceLock::new();

pub struct MetricsClient(pub &'static ClientInner);

impl Deref for MetricsClient {
    type Target = ClientInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait]
impl<T> FromRequestParts<T> for MetricsClient {
    type Rejection = StatusCode;

    async fn from_request_parts<'a, 'b>(_: &'a mut Parts, _: &'b T) -> Result<Self, Self::Rejection> {
        let client = USERS_CLIENT.get_or_init(Default::default);

        Ok(MetricsClient(client))
    }
}