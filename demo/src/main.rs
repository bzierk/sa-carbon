use std::{
    collections::{HashMap, HashSet},
    env,
    sync::Arc,
    time::Duration,
};

use async_trait::async_trait;
use carbon_core::{
    account::AccountProcessorInputType,
    error::CarbonResult,
    instruction::InstructionProcessorInputType,
    metrics::{self, MetricsCollection},
    pipeline::Pipeline,
    processor::Processor,
};
use carbon_log_metrics::LogMetrics;
use carbon_rpc_program_subscribe_datasource::{Filters as SubscribeFilters, RpcProgramSubscribe};
use carbon_rpc_transaction_crawler_datasource::{Filters as CrawlerFilters, RpcTransactionCrawler};
use carbon_yellowstone_grpc_datasource::YellowstoneGrpcGeyserClient;
use sage_decoder::{PROGRAM_ID, instructions::SageInstruction};
use solana_account_decoder::UiAccountEncoding;
use solana_client::rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig};

use sage_decoder::{SageDecoder, accounts::SageAccount};
use solana_commitment_config::{CommitmentConfig, CommitmentLevel};
use tokio::sync::RwLock;
use yellowstone_grpc_proto::geyser::{
    SubscribeRequestFilterAccounts, SubscribeRequestFilterTransactions,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DatasourceType {
    TransactionCrawler,
    Geyser,
    ProgramSubscribe,
}

impl DatasourceType {
    fn from_env() -> Self {
        match env::var("DATASOURCE_TYPE").unwrap_or_default().as_str() {
            "transaction_crawler" | "crawler" => Self::TransactionCrawler,
            "geyser" | "grpc" | "yellowstone" => Self::Geyser,
            "program_subscribe" | "subscribe" => Self::ProgramSubscribe,
            _ => panic!("Invalid datasource type"),
        }
    }
}

async fn create_pipeline() -> anyhow::Result<Pipeline> {
    let datasource_type = DatasourceType::from_env();
    log::info!("Datasource type: {:?}", datasource_type);

    let mut builder = Pipeline::builder()
        .account(SageDecoder, SageAccountProcessor)
        .instruction(SageDecoder, SageInstructionProcessor)
        .metrics(Arc::new(LogMetrics::new()));

    match datasource_type {
        DatasourceType::Geyser => {
            let mut account_filters: HashMap<String, SubscribeRequestFilterAccounts> =
                HashMap::new();
            account_filters.insert(
                "sage_account_filter".to_string(),
                SubscribeRequestFilterAccounts {
                    account: vec![],
                    owner: vec![PROGRAM_ID.to_string().clone()],
                    filters: vec![],
                    nonempty_txn_signature: None,
                },
            );

            let transaction_filter = SubscribeRequestFilterTransactions {
                vote: Some(false),
                failed: Some(false),
                account_include: vec![],
                account_exclude: vec![],
                account_required: vec![PROGRAM_ID.to_string().clone()],
                signature: None,
            };

            let mut transaction_filters: HashMap<String, SubscribeRequestFilterTransactions> =
                HashMap::new();

            transaction_filters.insert("sage_transaction_filter".to_string(), transaction_filter);

            builder = builder.datasource(YellowstoneGrpcGeyserClient::new(
                env::var("GEYSER_URL").unwrap_or_default(),
                env::var("X_TOKEN").ok(),
                Some(yellowstone_grpc_proto::geyser::CommitmentLevel::Confirmed),
                account_filters,
                transaction_filters,
                Arc::new(RwLock::new(HashSet::new())),
            ))
        }
        DatasourceType::TransactionCrawler => {
            builder = builder.datasource(RpcTransactionCrawler::new(
                env::var("RPC_URL").unwrap_or_default(),
                PROGRAM_ID,
                5,
                Duration::from_secs(2),
                CrawlerFilters::new(None, None, None),
                Some(CommitmentConfig {
                    commitment: CommitmentLevel::Confirmed,
                }),
                10,
            ));
        }
        DatasourceType::ProgramSubscribe => {
            builder = builder.datasource(RpcProgramSubscribe::new(
                env::var("RPC_WS_URL").unwrap_or_default(),
                SubscribeFilters::new(
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
            ));
        }
    }

    Ok(builder.build()?)
}

pub struct SageInstructionProcessor;

#[async_trait]
impl Processor for SageInstructionProcessor {
    type InputType = InstructionProcessorInputType<SageInstruction>;

    async fn process(
        &mut self,
        update: Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        println!("Processing instruction");
        let (_metadata, instruction, nested_instructions) = update;

        log::info!(
            "Instruction: {:#?} with nested instructions: {:#?}",
            instruction,
            nested_instructions
        );
        Ok(())
    }
}

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
            SageAccount::CombatConfig(combat_config) => {
                log::debug!("CombatConfig: {:#?}", combat_config);
            }
            SageAccount::CraftingInstance(crafting_instance) => {
                log::debug!("CraftingInstance: {:#?}", crafting_instance);
            }
            SageAccount::DisbandedFleet(disbanded_fleet) => {
                log::debug!("DisbandedFleet: {:#?}", disbanded_fleet);
            }
            SageAccount::FleetShips(fleet_ships) => {
                log::debug!("FleetShips: {:#?}", fleet_ships);
            }
            SageAccount::Fleet(fleet) => {
                log::debug!("Fleet: {:#?}", fleet);
            }
            SageAccount::GameState(game_state) => {
                log::debug!("GameState: {:#?}", game_state);
            }
            SageAccount::Game(game) => {
                log::debug!("Game: {:#?}", game);
            }
            SageAccount::Loot(loot) => {
                log::debug!("Loot: {:#?}", loot);
            }
            SageAccount::MineItem(mine_item) => {
                log::debug!("MineItem: {:#?}", mine_item);
            }
            SageAccount::Planet(planet) => {
                log::debug!("Planet: {:#?}", planet);
            }
            SageAccount::Resource(resource) => {
                log::debug!("Resource: {:#?}", resource);
            }
            SageAccount::SageCrewConfig(sage_crew_config) => {
                log::debug!("SageCrewConfig: {:#?}", sage_crew_config);
            }
            SageAccount::SagePlayerProfile(sage_player_profile) => {
                log::debug!("SagePlayerProfile: {:#?}", sage_player_profile);
            }
            SageAccount::Sector(sector) => {
                log::debug!("Sector: {:#?}", sector);
            }
            SageAccount::Ship(ship) => {
                log::debug!("Ship: {:#?}", ship);
            }
            SageAccount::Star(star) => {
                log::debug!("Star: {:#?}", star);
            }
            SageAccount::StarbasePlayer(starbase_player) => {
                log::debug!("StarbasePlayer: {:#?}", starbase_player);
            }
            SageAccount::Starbase(starbase) => {
                log::debug!("Starbase: {:#?}", starbase);
            }
            SageAccount::SurveyDataUnitTracker(survey_data_unit_tracker) => {
                log::debug!("SurveyDataUnitTracker: {:#?}", survey_data_unit_tracker);
            }
            _ => {
                log::warn!("Some other update: {:#?}", account);
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    log::info!("Starting pipeline");
    let mut pipeline = create_pipeline().await?;
    pipeline.run().await?;

    Ok(())
}
