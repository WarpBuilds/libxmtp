/*
XLI is a Commandline client using XMTPv3.


```
$ RUST_LOG=info cargo run -- --db ~/hello2.db3 send 0x5c1c5699cc216366723fd172e9acf5091dff8811 hiD
$ RUST_LOG=info cargo run -- --db ~/hello2.db3 send 0x5c1c5699cc216366723fd172e9acf5091dff8811 hiD

```
*/


extern crate ethers;
extern crate log;
extern crate xmtp;

use clap::{Parser, Subcommand};
use ethers_core::types::H160;
use log::{debug, error, info};
use std::path::PathBuf;
use thiserror::Error;
use url::ParseError;
use walletconnect::client::{CallError, ConnectorError, SessionError};
use walletconnect::{qr, Client as WcClient, Metadata};
use xmtp::builder::{AccountStrategy, ClientBuilderError};
use xmtp::client::ClientError;
use xmtp::conversations::Conversations;
use xmtp::storage::{EncryptedMessageStore, EncryptionKey, StorageError, StorageOption};
use xmtp::types::Address;
use xmtp::InboxOwner;
use xmtp::Save;
use xmtp_cryptography::signature::{h160addr_to_string, RecoverableSignature, SignatureError};
use xmtp_cryptography::utils::{rng, LocalWallet};
use xmtp_networking::grpc_api_helper::Client as ApiClient;

// use xmtp_networking::Client as ApiClient;
// use xmtp_cryptography::signature::{h160addr_to_string, RecoverableSignature, SignatureError};
// use xmtp_cryptography::utils::{rng, LocalWallet};


// type ApiClient = xmtp_networking::Client;
type Client = xmtp::client::Client<ApiClient>;
type ClientBuilder = xmtp::builder::ClientBuilder<ApiClient, Wallet>;

/// A fictional versioning CLI
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "xli")]
#[command(about = "A lightweight XMTP console client", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    /// Sets a custom config file
    #[arg(long, value_name = "FILE", global = true)]
    db: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Register Account on XMTP Network
    #[command(arg_required_else_help = true)]
    Reg {
        /// use wallect connect to associate an EOA
        #[clap(short = 'W', long = "use_wc", conflicts_with = "use_local")]
        use_wc: bool,
        /// Produce a report of selected PO
        #[clap(short = 'L', long, conflicts_with = "use_wc")]
        use_local: bool,
    },
    /// Send Message
    Send {
        #[arg(value_name = "ADDR")]
        addr: String,
        #[arg(value_name = "Message")]
        msg: String,
    },

    Test {
        #[arg(value_name = "FOO")]
        var: String,
    },
}

#[derive(Debug, Error)]
enum CliError {
    #[error("Walletconnect connection failed")]
    WcConnection(#[from] ConnectorError),
    #[error("Walletconnect session failed")]
    WcSession(#[from] SessionError),
    #[error("Walletconnect parse failed")]
    WcParse(#[from] ParseError),
    #[error("Walletconnect call failed")]
    WcCall(#[from] CallError),
    #[error("signature failed to generate")]
    Signature(#[from] SignatureError),
    #[error("stored error occured")]
    MessageStore(#[from] StorageError),
    #[error("client error")]
    ClientError(#[from] ClientError),
    #[error("clientbuilder error")]
    ClientBuilder(#[from] ClientBuilderError),
    #[error("no recipient {0} found")]
    NoRecipient(Address),
}

/// This is an abstraction which allows the CLI to choose between different wallet types.
enum Wallet {
    WalletConnectWallet(WalletConnectWallet),
    LocalWallet(LocalWallet),
}

impl InboxOwner for Wallet {
    fn get_address(&self) -> String {
        match self {
            Wallet::WalletConnectWallet(w) => w.get_address(),
            Wallet::LocalWallet(w) => w.get_address(),
        }
    }

    fn sign(&self, text: &str) -> Result<RecoverableSignature, SignatureError> {
        match self {
            Wallet::WalletConnectWallet(w) => w.sign(text),
            Wallet::LocalWallet(w) => w.sign(text),
        }
    }
}

/// A complete example of a minimal xmtp client which can send and recieve messages.
/// run this example from the cli:  `RUST_LOG=DEBUG cargo run --example cli-client`
#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Starting CLI Client....");

    let cli = Cli::parse();

    match &cli.command {
        Commands::Reg { use_wc, use_local } => {
            info!("'REG: {use_wc:?} {use_local:?} {:?}", cli.db);
            if let Err(e) = register(cli.db, *use_local).await {
                error!("reg failed: {:?}", e)
            }
        }
        Commands::Send { addr, msg } => {
            info!("Send");
            let client = create_client(cli.db, AccountStrategy::CachedOnly("nil".into()))
                .await
                .unwrap();
            info!("Address is: {}", client.wallet_address());
            send(client, addr, msg).await.unwrap();
        }
        Commands::Test {var }=> {
            info!("TEst");
            let client = create_client(cli.db, AccountStrategy::CachedOnly("nil".into())).await.unwrap();
            client.refresh_user_installations(&client.wallet_address()).await;
        }
    }
}

async fn create_client(
    db: Option<PathBuf>,
    account: AccountStrategy<Wallet>,
) -> Result<Client, CliError> {
    let msg_store = get_encrypted_store(db).unwrap();

    let client_result = ClientBuilder::new(account)
        .network(xmtp::Network::Dev)
        .api_client(
            ApiClient::create("http://localhost:5556".into(), false)
                .await
                .unwrap(),
        )
        .store(msg_store)
        .build();

    client_result.map_err(CliError::ClientBuilder)
}

async fn register(db: Option<PathBuf>, use_local: bool) -> Result<(), CliError> {
    let w = if use_local {
        info!("Fallback to LocalWallet");
        Wallet::LocalWallet(LocalWallet::new(&mut rng()))
    } else {
        Wallet::WalletConnectWallet(WalletConnectWallet::create().await?)
    };

    let mut client = create_client(db, AccountStrategy::CreateIfNotFound(w)).await?;
    info!("Address is: {}", client.wallet_address());

    if let Err(e) = client.init().await {
        error!("Initialization Failed: {}", e.to_string());
        panic!("Could not init");
    };

    info!(" Closing XLI");

    Ok(())
}

async fn send(client: Client, addr: &String, msg: &String) -> Result<(), CliError> {
    let contacts = client
        .get_contacts(addr)
        .await
        .map_err(CliError::ClientError)?;

    if contacts.is_empty() {
        error!("Recipient({}) is not registered", addr);
        return Err(CliError::NoRecipient(addr.clone()));
    }

    for contact in contacts {
        let mut session = client
            .get_session(&contact)
            .map_err(CliError::ClientError)?;
        debug!("Session: {}", session.id());

        let om = session.encrypt(msg.as_bytes());
        debug!("{:?} ", om);
        session.save(&client.store).unwrap();
        info!("Encryption successful")
    }

    let conversations = Conversations::new(&client);
    let conversation = conversations
        .new_secret_conversation(addr.to_string())
        .await
        .unwrap();
    conversation.send_message(msg).unwrap();
    info!("Message locally committed");

    Ok(())
}

fn static_enc_key() -> EncryptionKey {
    [2u8; 32]
}

fn get_encrypted_store(db: Option<PathBuf>) -> Result<EncryptedMessageStore, CliError> {
    let store = match db {
        Some(path) => {
            let s = path.as_path().to_string_lossy().to_string();
            info!("Using persistent storage:{} ", s);
            EncryptedMessageStore::new_unencrypted(StorageOption::Persistent(s))
        }

        None => {
            info!("USing ephemeral Store");
            EncryptedMessageStore::new(StorageOption::Ephemeral, static_enc_key())
        }
    };

    store.map_err(|e| e.into())
}

/// This wraps a Walletconnect::client into a struct which could be used in the xmtp::client.
struct WalletConnectWallet {
    addr: String,
    client: WcClient,
}

impl WalletConnectWallet {
    pub async fn create() -> Result<Self, CliError> {
        let client = WcClient::new(
            "examples-cli",
            Metadata {
                description: "XMTP CLI.".into(),
                url: "https://github.com/xmtp/libxmtp".parse()?,
                icons: vec![
                    "https://gateway.ipfs.io/ipfs/QmaSZuaXfNUwhF7khaRxCwbhohBhRosVX1ZcGzmtcWnqav"
                        .parse()?,
                ],
                name: "XMTP CLI".into(),
            },
        )?;

        let (accounts, _) = client.ensure_session(qr::print_with_url).await?;

        for account in &accounts {
            info!(" Connected account: {:?}", account);
        }

        Ok(Self {
            addr: h160addr_to_string(H160::from_slice(accounts[0].as_bytes())),
            client,
        })
    }
}

impl InboxOwner for WalletConnectWallet {
    fn get_address(&self) -> String {
        self.addr.clone()
    }

    fn sign(
        &self,
        text: &str,
    ) -> Result<
        xmtp_cryptography::signature::RecoverableSignature,
        xmtp_cryptography::signature::SignatureError,
    > {
        let sig = futures::executor::block_on(async { self.client.personal_sign(&[text]).await })
            .map_err(|e| SignatureError::ThirdPartyError(e.to_string()))?;

        Ok(RecoverableSignature::Eip191Signature(sig.to_vec()))
    }
}
