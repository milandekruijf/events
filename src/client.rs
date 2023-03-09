use super::On;
use anyhow::Result;
use async_trait::async_trait;
use std::{future::Future, pin::Pin, sync::Arc};
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct Client<E>
where
    E: Clone + Send + Sync + 'static,
{
    handlers: Arc<
        RwLock<
            Vec<
                Arc<
                    Box<
                        dyn Fn(E) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'static>>
                            + Send
                            + Sync
                            + 'static,
                    >,
                >,
            >,
        >,
    >,
}

impl<E> Client<E>
where
    E: Clone + Send + Sync + 'static,
{
    /// Create a new client.
    ///
    /// # Example
    ///
    /// ```
    /// use events::Client;
    ///
    /// let client: Client<Event> = Client::new();
    /// ```
    pub fn new() -> Self
    {
        Self {
            handlers: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Emit an event.
    ///
    /// # Example
    ///
    /// ```
    /// use events::Client;
    ///
    /// let client: Client<Event> = Client::new();
    ///
    /// client.emit(Event::Example(vec![]));
    /// ```
    pub async fn emit(&self, event: E)
    {
        for handler in self.handlers.read().await.iter()
        {
            let event = event.clone();
            let handler = handler.clone();

            tokio::spawn(async move {
                if let Err(err) = handler(event).await
                {
                    eprintln!("Error: {err:?}");
                }
            });
        }
    }
}

#[async_trait]
impl<E> On<E> for Client<E>
where
    E: Clone + Send + Sync + 'static,
{
    /// Register an event handler.
    ///
    /// # Example
    ///
    /// ```
    /// use events::{Client, On};
    ///
    /// let client: Client<Event> = Client::new();
    ///
    /// client.on(|event| async move {
    ///   ..
    /// });
    async fn on<F, R>(&self, f: F)
    where
        F: Fn(E) -> R + Send + Sync + 'static,
        R: Future<Output = Result<()>> + Send + 'static,
    {
        self.handlers
            .write()
            .await
            .push(Arc::new(Box::new(move |event| Box::pin(f(event)))));
    }
}
