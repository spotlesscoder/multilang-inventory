use crate::infra::auth::AuthUser;
use crate::infra::cache::RedisCache;
use axum::extract::{FromRequest, FromRequestParts, Request};
use axum::http::{Extensions, StatusCode};
use axum::response::Response;
use axum::RequestExt;
use futures::future::BoxFuture;
use std::convert::Infallible;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;
use tower::{Layer, Service};

#[derive(Clone)]
pub struct RateLimiterLayer {
    cache: Arc<RedisCache>,
    requests_per_minute: u32,
}

impl RateLimiterLayer {
    pub fn new(cache: Arc<RedisCache>, requests_per_minute: u32) -> Self {
        Self {
            cache,
            requests_per_minute,
        }
    }
}

pub trait ResponseType {
    fn into_response(self) -> Response;
}

impl<S> Layer<S> for RateLimiterLayer {
    type Service = RateLimiter<S>;

    fn layer(&self, service: S) -> Self::Service {
        RateLimiter {
            inner: service,
            cache: self.cache.clone(),
            requests_per_minute: self.requests_per_minute,
        }
    }
}

#[derive(Clone)]
pub struct RateLimiter<S> {
    inner: S,
    cache: Arc<RedisCache>,
    requests_per_minute: u32,
}

pub enum RateLimitedResponse<T, E = axum::response::Response> {
    Ok(T),
    Err(E),
}

impl<S> Service<Request> for RateLimiter<S>
where
    S: Clone,
    S: Service<Request, Error = Infallible> + Send + 'static,
    S::Response: Send + ResponseType + Clone + 'static,
    S::Future: Send + 'static,
{
    type Response = RateLimitedResponse<S::Response>;
    type Error = Infallible;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        let cache = self.cache.clone();
        let requests_per_minute = self.requests_per_minute;
        let mut inner = self.inner.clone();

        Box::pin(async move {
            // Extract user first before moving request
            let user = request.extensions().get::<AuthUser>().cloned();

            let current_count = 0;
            let reset_time = 60;
            if let Some(user) = user {
                let user_id = &user.user_id;
                let key = format!("rate_limit:{}", user_id);
                let ttl_key = format!("{}:ttl", key);

                // Get current request count and TTL
                let count: Option<u32> = cache.get(&key).await.unwrap_or(None);

                match count {
                    Some(count) if count >= requests_per_minute => {
                        // Get reset time
                        let ttl: Option<i64> = cache.get(&ttl_key).await.unwrap_or(None);
                        let reset = ttl.unwrap_or(60);

                        // Rate limit exceeded
                        return Ok(RateLimitedResponse::Err(
                            Response::builder()
                                .status(StatusCode::TOO_MANY_REQUESTS)
                                .header("X-RateLimit-Limit", requests_per_minute.to_string())
                                .header("X-RateLimit-Remaining", "0")
                                .header("X-RateLimit-Reset", reset.to_string())
                                .body("Rate limit exceeded. Please try again later.".into())
                                .unwrap(),
                        ));
                    }
                    Some(count) => {
                        let new_count = count + 1;
                        // Get or set TTL
                        let ttl: Option<i64> = cache.get(&ttl_key).await.unwrap_or(None);
                        let reset = ttl.unwrap_or(60);

                        cache
                            .set_with_ttl(&key, &new_count, Duration::from_secs(60))
                            .await
                            .unwrap();
                        cache
                            .set_with_ttl(&ttl_key, &reset, Duration::from_secs(60))
                            .await
                            .unwrap();
                        (new_count, reset)
                    }
                    None => {
                        // First request in this window
                        cache
                            .set_with_ttl(&key, &1, Duration::from_secs(60))
                            .await
                            .unwrap();
                        cache
                            .set_with_ttl(&ttl_key, &60, Duration::from_secs(60))
                            .await
                            .unwrap();
                        (1, 60)
                    }
                };
            }

            let service_response = inner.call(request).await?;
            let mut response = service_response.clone();
            // Add rate limit headers to the response
            let headers = response.headers_mut();
            headers.insert(
                "X-RateLimit-Limit",
                requests_per_minute.to_string().parse().unwrap(),
            );
            headers.insert(
                "X-RateLimit-Remaining",
                (requests_per_minute - current_count)
                    .to_string()
                    .parse()
                    .unwrap(),
            );
            headers.insert("X-RateLimit-Reset", reset_time.to_string().parse().unwrap());

            Ok(RateLimitedResponse::Ok(service_response))
        })
    }
}
