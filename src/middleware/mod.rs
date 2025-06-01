use crate::{state::AppState, utils::models::auth_tokens::AuthTokenModel};
use axum::{extract::{Request, State}, http::header};
use std::{sync::Arc, task::{Context, Poll}};
use tower::{Layer, Service};
// pub mod auth;

#[derive(Clone)]
pub struct AuthLayer {
    pub state: Arc<AppState>
}

#[derive(Clone)]
pub struct AuthService<S> {
    pub inner: S,
    pub state: Arc<AppState>,
}

impl<S, B> Service<Request<B>> for AuthService<S>
where
    S: Service<Request<B>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        // Do something with `self.state`.
        //
        // See `axum::RequestExt` for how to run extractors directly from
        // a `Request`.
        let token = req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_owned())
                    } else {
                        None
                    }
                }).unwrap_or("none".to_string());
        let _ = AuthTokenModel::find_by_value(token, &State(self.state.clone().into()));


        self.inner.call(req)
    }
}

impl<S> Layer<S> for AuthLayer {
    type Service = AuthService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthService {
            inner,
            state: self.state.clone(),
        }
    }
}