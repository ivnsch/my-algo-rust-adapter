use algonaut::{
    core::Address,
    transaction::{transaction::AssetParams, Transaction, TransactionType},
};
use anyhow::{anyhow, Result};
use data_encoding::{BASE32, BASE64};
use serde_json::Value;
use serde_with::skip_serializing_none;
use wasm_bindgen::JsValue;

pub fn to_my_algo_transaction(t: &Transaction) -> Result<JsValue> {
    let common_fields = to_my_algo_transaction_common_fields(t)?;
    let type_fields = to_my_algo_transaction_type_fields(t)?;

    let mut all_fields = common_fields;
    merge(&mut all_fields, type_fields);
    Ok(JsValue::from_serde(&all_fields)?)
}

// Preferring camel case fields over #[serde(rename_all = "camelCase")]: it's convenient for text search.
// Ok here as these structs are an intermediate, only used in this file
fn to_my_algo_transaction_common_fields(t: &Transaction) -> Result<Value> {
    Ok(serde_json::to_value(&MyAlgoTransactionCommonFields {
        fee: t.fee.0,
        flatFee: true, // per-txn fee
        firstRound: t.first_valid.0,
        genesisHash: BASE64.encode(&t.genesis_hash.0),
        lastRound: t.last_valid.0,
        genesisId: t.genesis_id.clone(),
        group: t.group.map(|d| BASE64.encode(&d.0)),
        lease: t.lease.map(|d| BASE64.encode(&d.0)),
        note: t.note.clone(),
        rekeyTo: t.rekey_to.map(|a| a.to_string()),
        type_: to_api_transaction_type(&t.txn_type).to_owned(),
    })?)
}

fn to_my_algo_transaction_type_fields(t: &Transaction) -> Result<Value> {
    match &t.txn_type {
        TransactionType::Payment(p) => Ok(serde_json::to_value(&MyAlgoPaymentTransactionFields {
            from: p.sender.to_string(),
            amount: p.amount.0,
            to: p.receiver.to_string(),
            closeRemainderTo: p.close_remainder_to.map(|a| a.to_string()),
        })?),
        TransactionType::KeyRegistration(r) => Ok(serde_json::to_value(
            &MyAlgoKeyRegistrationTransactionFields {
                from: r.sender.to_string(),
                voteKey: r.vote_pk.map(|v| BASE32.encode(&v.0)),
                selectionKey: r.selection_pk.map(|s| BASE32.encode(&s.0)),
                voteFirst: r.vote_first.map(|r| r.0),
                voteLast: r.vote_last.map(|r| r.0),
                voteKeyDilution: r.vote_key_dilution,
            },
        )?),
        TransactionType::AssetConfigurationTransaction(c) => Ok(serde_json::to_value(
            to_my_algo_asset_configuration_transaction_fields(c.sender, c.params.clone()),
        )?),
        TransactionType::AssetTransferTransaction(t) => Ok(serde_json::to_value(
            &MyAlgoAssetTransferTransactionFields {
                from: t.sender.to_string(),
                assetIndex: t.xfer,
                to: t.receiver.to_string(),
                amount: Some(t.amount),
                closeRemainderTo: t.close_to.map(|a| a.to_string()),
                assetSender: None,
            },
        )?),
        TransactionType::AssetAcceptTransaction(t) => Ok(serde_json::to_value(
            &MyAlgoAssetTransferTransactionFields {
                from: t.sender.to_string(),
                assetIndex: t.xfer,
                to: t.sender.to_string(),
                amount: None,
                closeRemainderTo: None,
                assetSender: None,
            },
        )?),
        TransactionType::AssetClawbackTransaction(t) => Ok(serde_json::to_value(
            &MyAlgoAssetTransferTransactionFields {
                from: t.sender.to_string(),
                assetIndex: t.xfer,
                to: t.asset_receiver.to_string(),
                amount: Some(t.asset_amount),
                closeRemainderTo: None,
                assetSender: Some(t.asset_sender.to_string()),
            },
        )?),
        TransactionType::AssetFreezeTransaction(t) => {
            Ok(serde_json::to_value(&MyAlgoAssetFreezeTransactionFields {
                from: t.sender.to_string(),
                assetIndex: t.asset_id,
                freezeAccount: t.freeze_account.to_string(),
                freezeState: t.frozen,
            })?)
        }
        _ => Result::Err(anyhow!("Not supported transaction type: {:?}", t.txn_type)),
    }
}

fn to_api_transaction_type<'a>(type_: &TransactionType) -> &'a str {
    match type_ {
        TransactionType::Payment(_) => "pay",
        TransactionType::KeyRegistration(_) => "keyreg",
        TransactionType::AssetConfigurationTransaction(_) => "acfg",
        TransactionType::AssetTransferTransaction(_) => "axfer",
        TransactionType::AssetAcceptTransaction(_) => "axfer",
        TransactionType::AssetClawbackTransaction(_) => "axfer",
        TransactionType::AssetFreezeTransaction(_) => "afrz",
        TransactionType::ApplicationCallTransaction(_) => "appl",
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize)]
#[allow(non_snake_case)]
struct MyAlgoTransactionCommonFields {
    fee: u64,
    flatFee: bool,
    firstRound: u64,
    genesisHash: String,
    lastRound: u64,
    genesisId: Option<String>,
    group: Option<String>,
    lease: Option<String>,
    note: Option<Vec<u8>>,
    rekeyTo: Option<String>,
    #[serde(rename = "type")]
    type_: String,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize)]
#[allow(non_snake_case)]
struct MyAlgoPaymentTransactionFields {
    from: String,
    amount: u64,
    to: String,
    closeRemainderTo: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize)]
#[allow(non_snake_case)]
struct MyAlgoKeyRegistrationTransactionFields {
    from: String,
    voteKey: Option<String>,
    selectionKey: Option<String>,
    voteFirst: Option<u64>,
    voteLast: Option<u64>,
    voteKeyDilution: Option<u64>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize)]
#[allow(non_snake_case)]
struct MyAlgoAssetConfigurationTransactionFields {
    from: String,
    assetName: Option<String>,
    assetUnitName: Option<String>,
    assetDecimals: Option<u32>,
    assetTotal: Option<u64>,
    assetURL: Option<String>,
    assetFreeze: Option<String>,
    assetManager: Option<String>,
    assetReserve: Option<String>,
    assetDefaultFrozen: Option<bool>,
}

// TODO no clawback example in My Algo, confirm
#[skip_serializing_none]
#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize)]
#[allow(non_snake_case)]
struct MyAlgoAssetTransferTransactionFields {
    from: String,
    assetIndex: u64,
    to: String,
    amount: Option<u64>,
    closeRemainderTo: Option<String>,
    assetSender: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize)]
#[allow(non_snake_case)]
struct MyAlgoAssetFreezeTransactionFields {
    from: String,
    assetIndex: u64,
    freezeAccount: String,
    freezeState: bool,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize)]
#[allow(non_snake_case)]
struct MyAlgoApplicationCallTransactionFields {
    from: String,
    assetIndex: u64,
    freezeAccount: String,
    freezeState: bool,
}

fn to_my_algo_asset_configuration_transaction_fields(
    sender: Address,
    params: Option<AssetParams>,
) -> MyAlgoAssetConfigurationTransactionFields {
    match params {
        Some(p) => MyAlgoAssetConfigurationTransactionFields {
            from: sender.to_string(),
            assetName: p.asset_name,
            assetUnitName: p.unit_name,
            assetDecimals: p.decimals,
            assetTotal: p.total,
            assetURL: p.url,
            assetFreeze: p.freeze.map(|a| a.to_string()),
            assetManager: p.manager.map(|a| a.to_string()),
            assetReserve: p.reserve.map(|a| a.to_string()),
            assetDefaultFrozen: p.default_frozen,
        },
        None => MyAlgoAssetConfigurationTransactionFields {
            from: sender.to_string(),
            assetName: None,
            assetUnitName: None,
            assetDecimals: None,
            assetTotal: None,
            assetURL: None,
            assetFreeze: None,
            assetManager: None,
            assetReserve: None,
            assetDefaultFrozen: None,
        },
    }
}

// https://stackoverflow.com/a/54118457/930450
fn merge(a: &mut Value, b: Value) {
    if let Value::Object(a) = a {
        if let Value::Object(b) = b {
            for (k, v) in b {
                if v.is_null() {
                    a.remove(&k);
                } else {
                    merge(a.entry(k).or_insert(Value::Null), v);
                }
            }
            return;
        }
    }
    *a = b;
}
