use diesel::{AsChangeset, Identifiable, Insertable, Queryable};

use crate::{enums as storage_enums, schema::payout_create};

#[derive(Clone, Debug, Default, Eq, PartialEq, Identifiable, Queryable)]
#[diesel(table_name = payout_create)]
pub struct PayoutCreate {
    pub id: i32,
    pub payout_id: String,
    pub customer_id: String,
    pub merchant_id: String,
    pub address_id: String,
    pub connector: String,
    pub connector_payout_id: String,
    pub payout_token: Option<String>,
    pub status: storage_enums::PayoutStatus,
    pub is_eligible: Option<bool>,
    pub encoded_data: Option<String>,
    pub error_message: Option<String>,
    pub error_code: Option<String>,
}

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Insertable,
    router_derive::DebugAsDisplay,
    router_derive::Setter,
)]
#[diesel(table_name = payout_create)]
pub struct PayoutCreateNew {
    pub payout_id: String,
    pub customer_id: String,
    pub merchant_id: String,
    pub address_id: String,
    pub connector: String,
    pub connector_payout_id: String,
    pub payout_token: Option<String>,
    pub status: storage_enums::PayoutStatus,
    pub is_eligible: Option<bool>,
    pub encoded_data: Option<String>,
    pub error_message: Option<String>,
    pub error_code: Option<String>,
}

#[derive(Debug)]
pub enum PayoutCreateUpdate {
    StatusUpdate {
        connector_payout_id: String,
        status: storage_enums::PayoutStatus,
        error_message: Option<String>,
        error_code: Option<String>,
        is_eligible: Option<bool>,
    },
    PayoutTokenUpdate {
        payout_token: String,
        status: storage_enums::PayoutStatus,
    },
}

#[derive(Clone, Debug, Default, AsChangeset, router_derive::DebugAsDisplay)]
#[diesel(table_name = payout_create)]
pub struct PayoutCreateUpdateInternal {
    pub payout_token: Option<String>,
    pub connector_payout_id: Option<String>,
    pub status: Option<storage_enums::PayoutStatus>,
    pub error_message: Option<String>,
    pub error_code: Option<String>,
    pub is_eligible: Option<bool>,
}

impl From<PayoutCreateUpdate> for PayoutCreateUpdateInternal {
    fn from(payout_update: PayoutCreateUpdate) -> Self {
        match payout_update {
            PayoutCreateUpdate::PayoutTokenUpdate {
                payout_token,
                status,
            } => Self {
                payout_token: Some(payout_token),
                status: Some(status),
                ..Default::default()
            },
            PayoutCreateUpdate::StatusUpdate {
                connector_payout_id,
                status,
                error_message,
                error_code,
                is_eligible,
            } => Self {
                connector_payout_id: Some(connector_payout_id),
                status: Some(status),
                error_message,
                error_code,
                is_eligible,
                ..Default::default()
            },
        }
    }
}