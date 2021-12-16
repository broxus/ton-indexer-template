use std::sync::Arc;

use anyhow::{Context, Result};

use self::ton_subscriber::*;
use crate::config::*;

mod ton_subscriber;

pub struct Engine {
    ton_subscriber: Arc<TonSubscriber>,
    ton_engine: Arc<ton_indexer::Engine>,
}

impl Engine {
    pub async fn new(config: AppConfig, global_config: ton_indexer::GlobalConfig) -> Result<Self> {
        let ton_subscriber = TonSubscriber::new();
        let ton_engine = ton_indexer::Engine::new(
            config
                .node_settings
                .build_indexer_config()
                .await
                .context("Failed to build node config")?,
            global_config,
            vec![ton_subscriber.clone() as Arc<dyn ton_indexer::Subscriber>],
        )
        .await
        .context("Failed to start TON node")?;

        Ok(Self {
            ton_subscriber,
            ton_engine,
        })
    }

    pub async fn start(&self) -> Result<()> {
        self.ton_engine.start().await?;
        Ok(())
    }
}
