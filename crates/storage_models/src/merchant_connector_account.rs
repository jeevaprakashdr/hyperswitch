use std::fmt::Debug;

use common_utils::pii;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use masking::{ExposeInterface, Secret};

use crate::{encryption::Encryption, enums as storage_enums, schema::merchant_connector_account};

#[derive(
    Clone,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    Identifiable,
    Queryable,
    router_derive::DebugAsDisplay,
)]
#[diesel(table_name = merchant_connector_account)]
pub struct MerchantConnectorAccount {
    pub id: i32,
    pub merchant_id: String,
    pub connector_name: String,
    pub connector_account_details: Encryption,
    pub test_mode: Option<bool>,
    pub disabled: Option<bool>,
    pub merchant_connector_id: String,
    #[diesel(deserialize_as = super::OptionalDieselArray<serde_json::Value>)]
    pub payment_methods_enabled: Option<Vec<serde_json::Value>>,
    pub connector_type: storage_enums::ConnectorType,
    pub metadata: Option<pii::SecretSerdeValue>,
    pub connector_label: String,
    pub business_country: storage_enums::CountryAlpha2,
    pub business_label: String,
    pub business_sub_label: Option<String>,
    pub frm_configs: Option<masking::Secret<serde_json::Value>>,
    pub created_at: time::PrimitiveDateTime,
    pub modified_at: time::PrimitiveDateTime,
}

#[derive(Clone, Debug, Insertable, router_derive::DebugAsDisplay)]
#[diesel(table_name = merchant_connector_account)]
pub struct MerchantConnectorAccountNew {
    pub merchant_id: Option<String>,
    pub connector_type: Option<storage_enums::ConnectorType>,
    pub connector_name: Option<String>,
    pub connector_account_details: Option<Encryption>,
    pub test_mode: Option<bool>,
    pub disabled: Option<bool>,
    pub merchant_connector_id: String,
    pub payment_methods_enabled: Option<Vec<serde_json::Value>>,
    pub metadata: Option<pii::SecretSerdeValue>,
    pub connector_label: String,
    pub business_country: storage_enums::CountryAlpha2,
    pub business_label: String,
    pub business_sub_label: Option<String>,
    pub frm_configs: Option<masking::Secret<serde_json::Value>>,
    pub created_at: time::PrimitiveDateTime,
    pub modified_at: time::PrimitiveDateTime,
}

#[derive(Clone, Debug, Default, AsChangeset, router_derive::DebugAsDisplay)]
#[diesel(table_name = merchant_connector_account)]
pub struct MerchantConnectorAccountUpdateInternal {
    pub merchant_id: Option<String>,
    pub connector_type: Option<storage_enums::ConnectorType>,
    pub connector_name: Option<String>,
    pub connector_account_details: Option<Encryption>,
    pub test_mode: Option<bool>,
    pub disabled: Option<bool>,
    pub merchant_connector_id: Option<String>,
    pub payment_methods_enabled: Option<Vec<serde_json::Value>>,
    pub metadata: Option<pii::SecretSerdeValue>,
    pub frm_configs: Option<masking::Secret<serde_json::Value>>,
    pub modified_at: Option<time::PrimitiveDateTime>,
}

impl MerchantConnectorAccountUpdateInternal {
    pub fn create_account(self, source: MerchantConnectorAccount) -> MerchantConnectorAccount {
        MerchantConnectorAccount {
            merchant_id: self.merchant_id.unwrap_or(source.merchant_id),
            connector_type: self.connector_type.unwrap_or(source.connector_type),
            connector_account_details: self.connector_account_details.unwrap_or_default().expose(),
            test_mode: self.test_mode,
            disabled: self.disabled,
            merchant_connector_id: self
                .merchant_connector_id
                .unwrap_or(source.merchant_connector_id),
            payment_methods_enabled: self.payment_methods_enabled,
            frm_configs: self.frm_configs,
            modified_at: self.modified_at,

            ..source
        }
    }
}

impl From<MerchantConnectorAccountUpdate> for MerchantConnectorAccountUpdateInternal {
    fn from(merchant_connector_account_update: MerchantConnectorAccountUpdate) -> Self {
        match merchant_connector_account_update {
            MerchantConnectorAccountUpdate::Update {
                merchant_id,
                connector_type,
                connector_account_details,
                test_mode,
                disabled,
                merchant_connector_id,
                payment_methods_enabled,
                metadata,
                frm_configs,
            } => Self {
                merchant_id,
                connector_type,
                connector_account_details,
                test_mode,
                disabled,
                merchant_connector_id,
                payment_methods_enabled,
                metadata,
                frm_configs,
                modified_at: common_utils::date_time::now(),
            },
        }
    }
}
