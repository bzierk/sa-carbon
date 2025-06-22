use std::{env, sync::Arc};

use async_trait::async_trait;
use carbon_core::{
    account::AccountProcessorInputType, error::CarbonResult, metrics::MetricsCollection,
    pipeline::Pipeline, processor::Processor,
};
use carbon_log_metrics::LogMetrics;
use carbon_rpc_program_subscribe_datasource::{Filters, RpcProgramSubscribe};
use sage_decoder::PROGRAM_ID;
use solana_account_decoder::UiAccountEncoding;
use solana_client::rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig};

use sage_decoder::{SageDecoder, accounts::SageAccount};

pub struct SageAccountProcessor;

#[async_trait]
impl Processor for SageAccountProcessor {
    type InputType = AccountProcessorInputType<SageAccount>;

    async fn process(
        &mut self,
        update: Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        println!("Processing update");
        let (_metadata, account) = update;

        match account.data {
            SageAccount::Ship(ship) => {
                log::debug!("Ship: {:#?}", ship);
            }
            SageAccount::Fleet(fleet) => {
                log::debug!("Fleet: {:#?}", fleet);
            }
            SageAccount::StarbasePlayer(starbase_player) => {
                log::info!("StarbasePlayer: {:#?}", starbase_player);
            }
            SageAccount::Starbase(starbase) => {
                log::debug!("Starbase: {:#?}", starbase);
            }
            SageAccount::Planet(planet) => {
                log::debug!("Planet: {:#?}", planet);
            }
            SageAccount::MineItem(mine_item) => {
                log::debug!("MineItem: {:#?}", mine_item);
            }
            _ => {
                log::debug!("Some other update: {:#?}", account);
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    println!("Starting pipeline");
    Pipeline::builder()
        .datasource(RpcProgramSubscribe::new(
            env::var("RPC_WS_URL").unwrap_or_default(),
            Filters::new(
                PROGRAM_ID,
                Some(RpcProgramAccountsConfig {
                    filters: None,
                    account_config: RpcAccountInfoConfig {
                        encoding: Some(UiAccountEncoding::Base64),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            ),
        ))
        .account(SageDecoder, SageAccountProcessor)
        .metrics(Arc::new(LogMetrics::new()))
        .build()?
        .run()
        .await?;

    Ok(())
}
