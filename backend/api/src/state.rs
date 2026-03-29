use crate::auth::AuthManager;
use crate::cache::{CacheConfig, CacheLayer};
use crate::health_monitor::HealthMonitorStatus;
use crate::resource_tracking::ResourceManager;
use prometheus::Registry;
use shared::error::RegistryError;
use sqlx::PgPool;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, RwLock};
use std::time::Instant;
use tokio::sync::broadcast;
use shared::models::Network;
use shared::source_storage::SourceStorage;
use crate::error::ApiError;
use crate::contract_events::ContractEventHub;
use serde_json::Value;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContractEventVisibility {
    Public,
    Private,
}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(tag = "type", content = "data", rename_all = "camelCase")]
pub enum RealtimeEvent {
    ContractDeployed {
        contract_id: String,
        contract_name: String,
        publisher: String,
        version: String,
        timestamp: String,
        network: Network,
    },
    ContractUpdated {
        contract_id: String,
        update_type: String,
        details: Value,
        timestamp: String,
    },
    CicdPipeline {
        contract_id: String,
        status: String,
        steps_completed: u32,
        total_steps: u32,
        timestamp: String,
    },
    VersionCreated {
        contract_id: String,
        version: String,
        network: Network,
        timestamp: String,
    },
    MetadataUpdated {
        contract_id: String,
        timestamp: String,
        changes: Value,
        visibility: ContractEventVisibility,
    },
    StatusUpdated {
        contract_id: String,
        status: String,
        timestamp: String,
        is_verified: bool,
        details: Option<Value>,
        visibility: ContractEventVisibility,
    },
}

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub started_at: Instant,
    pub cache: Arc<CacheLayer>,
    pub contract_events: Arc<ContractEventHub>,
    pub registry: Registry,
    pub job_engine: Arc<soroban_batch::engine::JobEngine>,
    pub is_shutting_down: Arc<AtomicBool>,
    pub health_monitor_status: HealthMonitorStatus,
    pub auth_mgr: Arc<RwLock<AuthManager>>,
    pub resource_mgr: Arc<RwLock<ResourceManager>>,
    pub source_storage: Arc<SourceStorage>,
    pub event_broadcaster: broadcast::Sender<RealtimeEvent>,
}

impl AppState {
    pub async fn new(
        db: PgPool,
        registry: Registry,
        job_engine: Arc<soroban_batch::engine::JobEngine>,
        is_shutting_down: Arc<AtomicBool>,
    ) -> Result<Self, shared::error::RegistryError> {
        let config = CacheConfig::from_env();
        let auth_mgr = Arc::new(RwLock::new(
            AuthManager::from_env().expect("JWT config validated at startup"),
        ));
        let resource_mgr = Arc::new(RwLock::new(ResourceManager::new()));
        let contract_events = Arc::new(ContractEventHub::from_env());
        let source_storage = Arc::new(SourceStorage::new().await?);
        let (event_broadcaster, _) = broadcast::channel(100);
        Ok(Self {
            db,
            started_at: Instant::now(),
            cache: Arc::new(CacheLayer::new(config).await),
            contract_events,
            registry,
            job_engine,
            is_shutting_down,
            health_monitor_status: HealthMonitorStatus::default(),
            auth_mgr,
            resource_mgr,
            source_storage,
            event_broadcaster,
        }
        })
    }
}
