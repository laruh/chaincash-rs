use chaincash_offchain::{node::node_from_config, TransactionService};
use chaincash_server::{Server, ServerState};
use chaincash_store::{ChainCashStore, Update};
use config::{Environment, File};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("store error: {0}")]
    Store(#[from] chaincash_store::Error),

    #[error("server error: {0}")]
    Server(#[from] chaincash_server::Error),

    #[error("offchain error: {0}")]
    OffChain(#[from] chaincash_offchain::Error),

    #[error("config error: {0}")]
    Config(#[from] config::ConfigError),
}

#[derive(serde::Deserialize, Debug)]
pub struct ChainCashConfig {
    server: chaincash_server::Config,
    store: chaincash_store::Config,
    node: chaincash_offchain::node::Config,
}

impl ChainCashConfig {
    pub fn new() -> Result<Self, Error> {
        let c = config::Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name("config/local").required(false))
            .add_source(Environment::with_prefix("chaincash"))
            .build()?;

        Ok(c.try_deserialize()?)
    }
}

/// Facade class encompassing all components that make up the `chaincash` application.
pub struct ChainCashApp {
    config: ChainCashConfig,
}

impl ChainCashApp {
    pub fn new(config: ChainCashConfig) -> Self {
        Self { config }
    }

    pub async fn run(&self) -> Result<(), Error> {
        let store = ChainCashStore::open(&self.config.store.url)?;

        if store.has_updates()? {
            store.update()?;
        }

        let listener = listenfd::ListenFd::from_env()
            .take_tcp_listener(0)
            .unwrap()
            .unwrap_or_else(|| {
                std::net::TcpListener::bind(format!(
                    "{}:{}",
                    self.config.server.url, self.config.server.port
                ))
                .unwrap()
            });

        let node = node_from_config(&self.config.node)?;
        let tx_service = TransactionService::new(node.clone());

        let state = ServerState {
            store,
            node,
            tx_service,
        };

        Ok(Server::serve(listener, state).await?)
    }
}
