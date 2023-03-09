use super::Event;
use anyhow::Result;
use async_trait::async_trait;
use std::future::Future;

#[async_trait]
pub trait On<E>
where
    E: Event,
{
    async fn on<F, R>(&self, f: F)
    where
        F: Fn(E) -> R + Send + Sync + 'static,
        R: Future<Output = Result<()>> + Send + 'static;
}
