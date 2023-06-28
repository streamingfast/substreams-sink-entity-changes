/// Tables is a collection of rows, which is a collection of columns and make it easy to build
/// up EntityChanges protobuf objects.
///
/// ```rust
///  use substreams::{scalar::{BigInt, BigDecimal}, Hex};
///  use substreams_entity_change::tables::Tables;
///
///  let mut tables = Tables::new();
///  let bigint0 = BigInt::from(0);
///  let bigdecimal0 = BigDecimal::from(0);
///
///  tables
///    .create_row("Factory", "0x0000000")
///    .set("poolCount", &bigint0)
///    .set("txCount", &bigint0)
///    .set("totalVolumeUSD", &bigdecimal0)
///    .set("totalVolumeETH", &bigdecimal0)
///    .set("totalFeesUSD", &bigdecimal0)
///    .set("totalFeesETH", &bigdecimal0)
///    .set("untrackedVolumeUSD", &bigdecimal0)
///    .set("totalValueLockedUSD", &bigdecimal0)
///    .set("totalValueLockedETH", &bigdecimal0)
///    .set("totalValueLockedUSDUntracked", &bigdecimal0)
///    .set("totalValueLockedETHUntracked", &bigdecimal0)
///    .set("owner", "0x0000000000000000000000000000000000000000");
/// ```
///
/// In the code above, we create a new row in the `Factory` table with the primary key `0x0000000`
/// and set the fields for the entity.
///
/// Later to update the row, we can do:
///
/// ```rust
///  use substreams::{scalar::{BigInt, BigDecimal}, Hex};
///  use substreams_entity_change::tables::Tables;
///
///  let mut tables = Tables::new();
///  let new_count = BigInt::from(1);
///
///  tables
///    .update_row("Factory", "0x0000000")
///    .set("txCount", new_count);
/// ```
///
/// When you have populated the table changes, you can convert them into an EntityChanges protobuf
/// so that the Graph Node Substreams Sink can ingest your data correctly:
///
/// ```rust
///  use substreams_entity_change::tables::Tables;
///
/// let mut tables = Tables::new();
/// // ... populate tables
/// let changes = tables.to_entity_changes();
/// ```
///
use crate::pb::entity::entity_change::Operation;
use crate::pb::entity::value::Typed;
use crate::pb::entity::{Array, EntityChange, EntityChanges, Field, Value};
use std::collections::HashMap;
use substreams::scalar::{BigDecimal, BigInt};

#[derive(Debug)]
pub struct Tables {
    // Map from table name to the primary keys within that table
    pub tables: HashMap<String, Rows>,
}

impl Tables {
    pub fn new() -> Self {
        Tables {
            tables: HashMap::new(),
        }
    }

    pub fn create_row<K: AsRef<str>>(&mut self, table: &str, key: K) -> &mut Row {
        let rows = self.tables.entry(table.to_string()).or_insert(Rows::new());
        let row = rows
            .pks
            .entry(key.as_ref().to_string())
            .or_insert(Row::new());
        match row.operation {
            Operation::Unspecified => {
                row.operation = Operation::Create;
            }
            Operation::Create => {}
            Operation::Update => {
                panic!("cannot create a row that was marked for update")
            }
            Operation::Delete => {
                panic!(
                    "cannot create a row after a scheduled delete operation - table: {} key: {}",
                    table,
                    key.as_ref().to_string()
                )
            }
            Operation::Final => {}
        }
        row
    }

    pub fn update_row<K: AsRef<str>>(&mut self, table: &str, key: K) -> &mut Row {
        let rows = self.tables.entry(table.to_string()).or_insert(Rows::new());
        let row = rows
            .pks
            .entry(key.as_ref().to_string())
            .or_insert(Row::new());
        match row.operation {
            Operation::Unspecified => {
                row.operation = Operation::Update;
            }
            Operation::Create => {}
            Operation::Update => {}
            Operation::Delete => {
                panic!(
                    "cannot update a row after a scheduled delete operation - table: {} key: {}",
                    table,
                    key.as_ref().to_string()
                )
            }
            Operation::Final => {
                panic!("impossible state")
            }
        }
        row
    }

    pub fn delete_row<K: AsRef<str>>(&mut self, table: &str, key: K) -> &mut Row {
        let rows = self.tables.entry(table.to_string()).or_insert(Rows::new());
        let row = rows
            .pks
            .entry(key.as_ref().to_string())
            .or_insert(Row::new());
        match row.operation {
            Operation::Unspecified => {
                row.operation = Operation::Delete;
            }
            Operation::Create => {
                // simply clear the thing
                row.operation = Operation::Unspecified;
                row.columns = HashMap::new();
            }
            Operation::Update => {
                row.columns = HashMap::new();
            }
            Operation::Delete => {}
            Operation::Final => {
                panic!("impossible state");
            }
        }
        row.operation = Operation::Delete;
        row.columns = HashMap::new();
        row
    }

    // Convert Tables into an EntityChanges protobuf object
    pub fn to_entity_changes(self) -> EntityChanges {
        let mut entities = EntityChanges::default();
        for (table, rows) in self.tables.into_iter() {
            for (pk, row) in rows.pks.into_iter() {
                if row.operation == Operation::Unspecified {
                    continue;
                }

                // Doing it right now removes the need to perform a `pk.clone()` when creating the real change
                // below. We assume finalized row happen much less often than standard, so we save a bunch of
                // clone by eagerly creating the finalized row here.
                let mut pk_finalized: Option<EntityChange> = None;
                if row.finalized {
                    pk_finalized = Some(EntityChange::new(
                        table.clone(),
                        pk.clone(),
                        0,
                        Operation::Final,
                    ))
                }

                // Map the row.operation into an EntityChange.Operation
                let mut change = EntityChange::new(table.clone(), pk, 0, row.operation);
                for (field, value) in row.columns.into_iter() {
                    #[allow(deprecated)]
                    change.fields.push(Field {
                        name: field,
                        new_value: Some(value),
                        old_value: None,
                    });
                }

                entities.entity_changes.push(change);

                if let Some(finalized_row) = pk_finalized {
                    entities.entity_changes.push(finalized_row);
                }
            }
        }
        entities
    }
}

#[derive(Debug)]
pub struct Rows {
    // Map of primary keys within this table, to the fields within
    pub pks: HashMap<String, Row>,
}

impl Rows {
    pub fn new() -> Self {
        Rows {
            pks: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct Row {
    // Verify that we don't try to delete the same row as we're creating it
    pub operation: Operation,
    // Map of field name to its last change
    pub columns: HashMap<String, Value>,
    // Finalized: Last update or delete
    pub finalized: bool,
}

impl Row {
    pub fn new() -> Self {
        Row {
            operation: Operation::Unspecified,
            columns: HashMap::new(),
            finalized: false,
        }
    }

    pub fn set<T: ToValue>(&mut self, name: &str, value: T) -> &mut Self {
        if self.operation == Operation::Delete {
            panic!("cannot set fields on a delete operation")
        }
        self.columns.insert(name.to_string(), value.to_value());
        self
    }

    pub fn set_bigint(&mut self, name: &str, value: &String) -> &mut Self {
        self.columns.insert(
            name.to_string(),
            Value {
                typed: Some(Typed::Bigint(value.clone())),
            },
        );
        self
    }

    pub fn set_bigdecimal(&mut self, name: &str, value: &String) -> &mut Self {
        self.columns.insert(
            name.to_string(),
            Value {
                typed: Some(Typed::Bigdecimal(value.clone())),
            },
        );
        self
    }

    pub fn set_bigint_or_zero(&mut self, name: &str, value: &String) -> &mut Self {
        if value.len() == 0 {
            self.set_bigint(name, &"0".to_string())
        } else {
            self.set_bigint(name, value)
        }
    }

    pub fn _mark_final(&mut self) -> &mut Self {
        self.finalized = true;
        self
    }
}

pub trait ToValue {
    fn to_value(self) -> Value;
}

impl ToValue for &bool {
    fn to_value(self) -> Value {
        Value {
            typed: Some(Typed::Bool(*self)),
        }
    }
}

impl ToValue for &BigDecimal {
    fn to_value(self) -> Value {
        Value {
            typed: Some(Typed::Bigdecimal(self.to_string())),
        }
    }
}

impl ToValue for &str {
    fn to_value(self) -> Value {
        Value {
            typed: Some(Typed::String(self.to_string())),
        }
    }
}

impl ToValue for &String {
    fn to_value(self) -> Value {
        Value {
            typed: Some(Typed::String(self.clone())),
        }
    }
}

impl ToValue for String {
    fn to_value(self) -> Value {
        Value {
            typed: Some(Typed::String(self)),
        }
    }
}

impl ToValue for &Vec<u8> {
    fn to_value(self) -> Value {
        Value {
            typed: Some(Typed::Bytes(base64::encode(self))),
        }
    }
}

impl ToValue for Vec<String> {
    fn to_value(self) -> Value {
        Value {
            typed: Some(Typed::Array(Array {
                value: self.into_iter().map(ToValue::to_value).collect(),
            })),
        }
    }
}

impl ToValue for &Vec<String> {
    fn to_value(self) -> Value {
        Value {
            typed: Some(Typed::Array(Array {
                value: self.iter().map(ToValue::to_value).collect(),
            })),
        }
    }
}

impl ToValue for &::prost_types::Timestamp {
    fn to_value(self) -> Value {
        Value {
            typed: Some(Typed::String(self.to_string())),
        }
    }
}

macro_rules! impl_to_database_value_proxy_to_ref {
    ($name:ty) => {
        impl ToValue for $name {
            fn to_value(self) -> Value {
                ToValue::to_value(&self)
            }
        }
    };
}

// Those owns the received value (so once called, the value is dropped)
impl_to_database_value_proxy_to_ref!(bool);
impl_to_database_value_proxy_to_ref!(BigDecimal);
impl_to_database_value_proxy_to_ref!(Vec<u8>);
impl_to_database_value_proxy_to_ref!(::prost_types::Timestamp);

macro_rules! impl_to_bigint_value_proxy_to_string {
    ($name:ty) => {
        impl ToValue for $name {
            fn to_value(self) -> Value {
                Value {
                    typed: Some(Typed::Bigint(self.to_string())),
                }
            }
        }
    };
}

impl_to_bigint_value_proxy_to_string!(i64);
impl_to_bigint_value_proxy_to_string!(u8);
impl_to_bigint_value_proxy_to_string!(u16);
impl_to_bigint_value_proxy_to_string!(u32);
impl_to_bigint_value_proxy_to_string!(u64);
impl_to_bigint_value_proxy_to_string!(BigInt);
impl_to_bigint_value_proxy_to_string!(&BigInt);

macro_rules! impl_to_int32_value {
    ($name:ty) => {
        impl ToValue for $name {
            fn to_value(self) -> Value {
                Value {
                    typed: Some(Typed::Int32(self as i32)),
                }
            }
        }
    };
}

impl_to_int32_value!(i8);
impl_to_int32_value!(i16);
impl_to_int32_value!(i32);
