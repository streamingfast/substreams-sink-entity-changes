use crate::pb::entity::value::Typed;
use crate::pb::entity::{Array, Field, Value};
use std::str;
use substreams::pb::substreams::store_delta::Operation;
use substreams::scalar::{BigDecimal, BigInt};
use substreams::store::{
    DeltaArray, DeltaBigDecimal, DeltaBigInt, DeltaBool, DeltaBytes, DeltaInt32, DeltaString,
};

pub trait ToField {
    fn to_field<N: AsRef<str>>(self, name: N) -> Field;
}

impl Into<Typed> for i32 {
    fn into(self) -> Typed {
        Typed::Int32(self)
    }
}

impl ToField for DeltaInt32 {
    fn to_field<N: AsRef<str>>(self, name: N) -> Field {
        ToField::to_field(&self, name)
    }
}

impl ToField for &DeltaInt32 {
    fn to_field<N: AsRef<str>>(self, name: N) -> Field {
        delta_to_field(
            name.as_ref(),
            self.operation,
            self.old_value,
            self.new_value,
        )
    }
}

// ---------- BigDecimal ----------
impl Into<Typed> for BigDecimal {
    fn into(self) -> Typed {
        Into::into(&self)
    }
}

impl Into<Typed> for &BigDecimal {
    fn into(self) -> Typed {
        Typed::Bigdecimal(self.to_string())
    }
}

impl ToField for DeltaBigDecimal {
    fn to_field<N: AsRef<str>>(self, name: N) -> Field {
        delta_to_field(
            name.as_ref(),
            self.operation,
            self.old_value,
            self.new_value,
        )
    }
}

impl ToField for &DeltaBigDecimal {
    fn to_field<N: AsRef<str>>(self, name: N) -> Field {
        delta_to_field(
            name.as_ref(),
            self.operation,
            &self.old_value,
            &self.new_value,
        )
    }
}
// ---------- BigInt ----------
impl Into<Typed> for BigInt {
    fn into(self) -> Typed {
        Into::into(&self)
    }
}

impl Into<Typed> for &BigInt {
    fn into(self) -> Typed {
        Typed::Bigint(self.to_string())
    }
}

impl ToField for DeltaBigInt {
    fn to_field<N: AsRef<str>>(self, name: N) -> Field {
        delta_to_field(
            name.as_ref(),
            self.operation,
            self.old_value,
            self.new_value,
        )
    }
}

impl ToField for &DeltaBigInt {
    fn to_field<N: AsRef<str>>(self, name: N) -> Field {
        delta_to_field(
            name.as_ref(),
            self.operation,
            &self.old_value,
            &self.new_value,
        )
    }
}

// ---------- String ----------
impl Into<Typed> for String {
    fn into(self) -> Typed {
        Typed::String(self)
    }
}

impl Into<Typed> for &String {
    fn into(self) -> Typed {
        Typed::String(self.clone())
    }
}

impl ToField for DeltaString {
    fn to_field<N: AsRef<str>>(self, name: N) -> Field {
        delta_to_field(
            name.as_ref(),
            self.operation,
            self.old_value,
            self.new_value,
        )
    }
}

impl ToField for &DeltaString {
    fn to_field<N: AsRef<str>>(self, name: N) -> Field {
        delta_to_field(
            name.as_ref(),
            self.operation,
            &self.old_value,
            &self.new_value,
        )
    }
}

// ---------- Bytes ----------
impl Into<Typed> for Vec<u8> {
    fn into(self) -> Typed {
        Into::into(&self)
    }
}

impl Into<Typed> for &Vec<u8> {
    fn into(self) -> Typed {
        Typed::Bytes(base64::encode(self))
    }
}

impl ToField for DeltaBytes {
    fn to_field<N: AsRef<str>>(self, name: N) -> Field {
        ToField::to_field(&self, name)
    }
}

impl ToField for &DeltaBytes {
    fn to_field<N: AsRef<str>>(self, name: N) -> Field {
        delta_to_field(
            name.as_ref(),
            self.operation,
            &self.old_value,
            &self.new_value,
        )
    }
}

fn delta_to_field<T: Into<Typed>>(
    name: &str,
    operation: Operation,
    old_value: T,
    new_value: T,
) -> Field {
    match Operation::from(operation) {
        Operation::Update => ToField::to_field((old_value, new_value), name),
        Operation::Create => ToField::to_field(new_value, name),
        x => panic!("unsupported operation {:?}", x),
    }
}

// ---------- BoolChange ----------
impl Into<Typed> for bool {
    fn into(self) -> Typed {
        Typed::Bool(self)
    }
}

impl ToField for DeltaBool {
    fn to_field<N: AsRef<str>>(self, name: N) -> Field {
        ToField::to_field(&self, name)
    }
}

impl ToField for &DeltaBool {
    fn to_field<N: AsRef<str>>(self, name: N) -> Field {
        let new_value: Option<Value> = Some(Value {
            typed: Some(self.new_value.into()),
        });
        let mut old_value: Option<Value> = None;

        match Operation::from(self.operation) {
            Operation::Update => {
                old_value = Some(Value {
                    typed: Some(self.old_value.into()),
                });
            }
            Operation::Create => {}
            _ => panic!("unsupported operation {:?}", self.operation),
        }

        Field {
            name: name.as_ref().to_string(),
            new_value,
            old_value,
        }
    }
}

// ---------- StringArray ----------

impl Into<Typed> for Vec<String> {
    fn into(self) -> Typed {
        Into::into(&self)
    }
}

impl Into<Typed> for &Vec<String> {
    fn into(self) -> Typed {
        let mut list: Vec<Value> = vec![];
        for item in self.iter() {
            list.push(Value {
                typed: Some(Typed::String(item.clone())),
            });
        }

        Typed::Array(Array { value: list })
    }
}

impl ToField for DeltaArray<String> {
    fn to_field<N: AsRef<str>>(self, name: N) -> Field {
        ToField::to_field(&self, name)
    }
}

impl ToField for &DeltaArray<String> {
    fn to_field<N: AsRef<str>>(self, name: N) -> Field {
        let new_value: Option<Value> = Some(Value {
            typed: Some((&self.new_value).into()),
        });
        let mut old_value: Option<Value> = None;

        match Operation::from(self.operation) {
            Operation::Update => {
                old_value = Some(Value {
                    typed: Some((&self.old_value).into()),
                });
            }
            Operation::Create => {}
            _ => panic!("unsupported operation {:?}", self.operation),
        }

        Field {
            name: name.as_ref().to_string(),
            new_value,
            old_value,
        }
    }
}

impl Into<Typed> for u64 {
    fn into(self) -> Typed {
        Typed::Bigint(self.to_string())
    }
}

impl Into<Typed> for ::prost_types::Timestamp {
    fn into(self) -> Typed {
        Into::into(&self)
    }
}

impl Into<Typed> for &::prost_types::Timestamp {
    fn into(self) -> Typed {
        Typed::String(self.to_string())
    }
}

impl<T: Into<Typed>> ToField for T {
    fn to_field<N: AsRef<str>>(self, name: N) -> Field {
        Field {
            name: name.as_ref().to_string(),
            old_value: None,
            new_value: Some(Value {
                typed: Some(self.into()),
            }),
        }
    }
}

impl<T: Into<Typed>> ToField for (T, T) {
    fn to_field<N: AsRef<str>>(self, name: N) -> Field {
        Field {
            name: name.as_ref().to_string(),
            old_value: Some(Value {
                typed: Some(self.0.into()),
            }),
            new_value: Some(Value {
                typed: Some(self.1.into()),
            }),
        }
    }
}

impl<T: Into<Typed>> ToField for (T, Option<T>) {
    fn to_field<N: AsRef<str>>(self, name: N) -> Field {
        match self.1 {
            Some(x) => ToField::to_field((self.0, x), name),
            None => Field {
                name: name.as_ref().to_string(),
                old_value: Some(Value {
                    typed: Some(self.0.into()),
                }),
                new_value: None,
            },
        }
    }
}

#[cfg(test)]
mod test {
    use crate::change::ToField;
    use crate::pb::entity::value::Typed;
    use crate::pb::entity::{Array, Field, Value};
    use substreams::pb::substreams::store_delta::Operation;
    use substreams::scalar::{BigDecimal, BigInt};
    use substreams::store::{
        DeltaArray, DeltaBigDecimal, DeltaBigInt, DeltaBool, DeltaBytes, DeltaString,
    };

    const FIELD_NAME: &str = "field.name.1";

    #[test]
    fn i32_change() {
        let i32_change: i32 = 1;
        assert_eq!(
            create_expected_field(FIELD_NAME, None, Some(Typed::Int32(1))),
            i32_change.to_field(FIELD_NAME)
        );
    }

    #[test]
    fn big_decimal_change() {
        let bd_change = BigDecimal::from(1 as i32);
        assert_eq!(
            create_expected_field(FIELD_NAME, None, Some(Typed::Bigdecimal("1".to_string()))),
            bd_change.to_field(FIELD_NAME)
        );
    }

    #[test]
    fn delta_big_decimal_change() {
        let delta = DeltaBigDecimal {
            operation: Operation::Update,
            ordinal: 0,
            key: "change".to_string(),
            old_value: BigDecimal::from(10),
            new_value: BigDecimal::from(20),
        };

        assert_eq!(
            create_expected_field(
                FIELD_NAME,
                Some(Typed::Bigdecimal("10".to_string())),
                Some(Typed::Bigdecimal("20".to_string()))
            ),
            delta.to_field(FIELD_NAME)
        );
    }

    #[test]
    fn big_int_change() {
        let bi_change = BigInt::from(1 as i32);
        assert_eq!(
            create_expected_field(FIELD_NAME, None, Some(Typed::Bigint("1".to_string()))),
            bi_change.to_field(FIELD_NAME)
        );
    }

    #[test]
    fn delta_big_int_change() {
        let delta = DeltaBigInt {
            operation: Operation::Update,
            ordinal: 0,
            key: "change".to_string(),
            old_value: BigInt::from(10),
            new_value: BigInt::from(20),
        };

        assert_eq!(
            create_expected_field(
                FIELD_NAME,
                Some(Typed::Bigint("10".to_string())),
                Some(Typed::Bigint("20".to_string()))
            ),
            delta.to_field(FIELD_NAME)
        );
    }

    #[test]
    fn string_change() {
        let string_change = String::from("string");
        assert_eq!(
            create_expected_field(FIELD_NAME, None, Some(Typed::String("string".to_string()))),
            string_change.to_field(FIELD_NAME)
        );
    }

    #[test]
    fn delta_string_change() {
        let delta = DeltaString {
            operation: Operation::Update,
            ordinal: 0,
            key: "change".to_string(),
            old_value: String::from("string1"),
            new_value: String::from("string2"),
        };

        assert_eq!(
            create_expected_field(
                FIELD_NAME,
                Some(Typed::String("string1".to_string())),
                Some(Typed::String("string2".to_string()))
            ),
            delta.to_field(FIELD_NAME)
        );
    }

    #[test]
    fn bytes_change() {
        let bytes_change = Vec::from("bytes");
        assert_eq!(
            create_expected_field(FIELD_NAME, None, Some(Typed::Bytes("Ynl0ZXM=".to_string()))),
            bytes_change.to_field(FIELD_NAME)
        );
    }

    #[test]
    fn delta_bytes_change() {
        let delta = DeltaBytes {
            operation: Operation::Update,
            ordinal: 0,
            key: "change".to_string(),
            old_value: Vec::from("bytes1"),
            new_value: Vec::from("bytes2"),
        };

        assert_eq!(
            create_expected_field(
                FIELD_NAME,
                Some(Typed::Bytes("Ynl0ZXMx".to_string())),
                Some(Typed::Bytes("Ynl0ZXMy".to_string()))
            ),
            delta.to_field(FIELD_NAME)
        );
    }

    #[test]
    fn bool_change() {
        let bool_change = true;
        assert_eq!(
            create_expected_field(FIELD_NAME, None, Some(Typed::Bool(true))),
            bool_change.to_field(FIELD_NAME)
        );
    }

    #[test]
    fn delta_bool_change() {
        let delta = DeltaBool {
            operation: Operation::Update,
            ordinal: 0,
            key: "change".to_string(),
            old_value: true,
            new_value: false,
        };

        assert_eq!(
            create_expected_field(
                FIELD_NAME,
                Some(Typed::Bool(true)),
                Some(Typed::Bool(false)),
            ),
            delta.to_field(FIELD_NAME)
        );
    }

    #[test]
    fn vec_string_change() {
        let vec_string_change: Vec<String> = vec![
            String::from("string1"),
            String::from("string2"),
            String::from("string3"),
        ];
        assert_eq!(
            create_expected_field(
                FIELD_NAME,
                None,
                Some(Typed::Array(Array {
                    value: vec![
                        Value {
                            typed: Some(Typed::String("string1".to_string()))
                        },
                        Value {
                            typed: Some(Typed::String("string2".to_string()))
                        },
                        Value {
                            typed: Some(Typed::String("string3".to_string()))
                        },
                    ]
                }))
            ),
            vec_string_change.to_field(FIELD_NAME)
        );
    }

    #[test]
    fn delta_vec_string_change() {
        let delta = DeltaArray {
            operation: Operation::Update,
            ordinal: 0,
            key: "change".to_string(),
            old_value: vec![
                "string1".to_string(),
                "string2".to_string(),
                "string3".to_string(),
            ],
            new_value: vec![
                "string1.1".to_string(),
                "string2.1".to_string(),
                "string3.1".to_string(),
            ],
        };

        assert_eq!(
            create_expected_field(
                FIELD_NAME,
                Some(Typed::Array(Array {
                    value: vec![
                        Value {
                            typed: Some(Typed::String("string1".to_string()))
                        },
                        Value {
                            typed: Some(Typed::String("string2".to_string()))
                        },
                        Value {
                            typed: Some(Typed::String("string3".to_string()))
                        },
                    ]
                })),
                Some(Typed::Array(Array {
                    value: vec![
                        Value {
                            typed: Some(Typed::String("string1.1".to_string()))
                        },
                        Value {
                            typed: Some(Typed::String("string2.1".to_string()))
                        },
                        Value {
                            typed: Some(Typed::String("string3.1".to_string()))
                        },
                    ]
                })),
            ),
            delta.to_field(FIELD_NAME)
        );
    }

    fn create_expected_field<N: AsRef<str>>(
        name: N,
        old_value: Option<Typed>,
        new_value: Option<Typed>,
    ) -> Field {
        let mut field = Field {
            name: name.as_ref().to_string(),
            ..Default::default()
        };
        if old_value.is_some() {
            field.old_value = Some(Value { typed: old_value })
        }
        if new_value.is_some() {
            field.new_value = Some(Value { typed: new_value })
        }
        field
    }
}
