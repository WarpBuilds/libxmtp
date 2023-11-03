use ethers::{
    contract::abigen,
    prelude::{Provider, SignerMiddleware, LocalWallet},
    providers::{Middleware, Ws},
    signers::Signer,
    types::{H160, U256}
};
use tracing_wasm::WASMLayerConfigBuilder;
use wasm_bindgen::prelude::*;
use anyhow::Error;
use std::{str::FromStr, sync::Arc};


pub mod utils;

pub const ONE_WEEK: u64 = 604_800;

// Generate rust bindings to our solidity contract 
abigen!(DIDRegistry, "./src/abi/DIDRegistry.json", derives(serde::Deserialize, serde::Serialize));

type PrototypeMiddleware = SignerMiddleware<Provider<Ws>, LocalWallet>;

async fn deploy_registry() -> Result<String, Error> {
    utils::set_panic_hook();
    
    
    tracing::debug!(
        "DIDRegistry ABI {:?}",
        &serde_wasm_bindgen::to_value(&*DIDREGISTRY_ABI).unwrap(),
    );

    // our ethereum development node
    let endpoint = "ws://127.0.0.1:8545";
    let provider = Provider::<Ws>::connect(endpoint).await.unwrap();
    tracing::info!("Connected to: `{endpoint}`");
    let chain_id = provider.get_chainid().await.unwrap();

    // wallet/signer info
    let wallet = utils::key(0).with_chain_id(chain_id.as_u64());
    tracing::info!("Wallet: {wallet:?}");
    let client = Arc::new(SignerMiddleware::new(provider, wallet));

    tracing::info!("Deploying contract...");
    
    let did_contract = DIDRegistry::deploy(client.clone(), ()).unwrap().send().await.unwrap();
    tracing::info!("🎉Deployment Success!🎉 Deployed contract at address {:?}", did_contract.address());

    Ok(format!("{:x}", did_contract.address()))
}

#[wasm_bindgen]
pub fn set_logger() {
    let tracing_config = WASMLayerConfigBuilder::new()
        .set_max_level(tracing::Level::INFO)
        .build();
    tracing_wasm::set_as_global_default_with_config(tracing_config);
} 


#[wasm_bindgen]
pub struct Registry {
    contract: DIDRegistry<PrototypeMiddleware>,
    signer: Arc<PrototypeMiddleware> 
}

/// The registry.
/// We would pass things like contract address signer and provider to it from the client SDK
#[wasm_bindgen]
impl Registry {
    #[wasm_bindgen(constructor)]
    pub async fn new() -> Result<Registry, JsError> {
        // this could be better, but shows how we may accept environment variables from the outside
        let registry_address = deploy_registry().await.map_err(|e| JsError::new(&e.to_string()))?;
        let endpoint = "ws://127.0.0.1:8545";
        let provider = Provider::<Ws>::connect(endpoint).await?;
        let chain_id = provider.get_chainid().await?;
        tracing::info!("???");
        // wallet/signer info
        let user_wallet = utils::key(3).with_chain_id(chain_id.as_u64());
        tracing::info!("Wallet: {user_wallet:?}");

        let signer = Arc::new(SignerMiddleware::new(provider, user_wallet));
        tracing::info!("Registry Contract address: {registry_address}");
        let registry_address = H160::from_str(&registry_address).unwrap();
        let contract = DIDRegistry::new(registry_address, signer.clone());

        Ok(Self { contract, signer })
    }
    
    pub async fn set_attribute(&self, attribute: String) -> Result<String, JsError> {
        let tx = self.contract.set_attribute(self.signer.address(), *b"did:eth some attribute0000000000", attribute.as_bytes().to_vec().into(), U256::from(ONE_WEEK));
        let receipt = tx.send().await?.await?;
        tracing::info!("Receipt: {receipt:?}");
        Ok(format!("{receipt:?}"))
    }
}
