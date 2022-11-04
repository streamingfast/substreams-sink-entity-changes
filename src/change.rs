use crate::pb::entity::value::Typed;
use crate::pb::entity::{Array, Field, Value};
use std::str;
use substreams::pb::substreams::store_delta::Operation;
use substreams::scalar::{BigDecimal, BigInt};
use substreams::store::{
    DeltaArray, DeltaBigDecimal, DeltaBigInt, DeltaBool, DeltaBytes, DeltaInt32, DeltaString,
};

pub trait ToField {
    fn to_field<N: AsRef<str>>(&self, name: N) -> Field;
}

// ---------- Int32 ----------
impl ToField for i32 {
    fn to_field<N: AsRef<str>>(&self, name: N) -> Field {
        Field {
            name: name.as_ref().to_string(),
            new_value: Some(Value {
                typed: Some(Typed::Int32(*self)),
            }),
            old_value: None,
        }
    }
}

impl ToField for DeltaInt32 {
    fn to_field<N: AsRef<str>>(&self, name: N) -> Field {
        let new_value: Option<Value> = Some(Value {
            typed: Some(Typed::Int32(self.new_value)),
        });
        let mut old_value: Option<Value> = None;

        match Operation::from(self.operation) {
            Operation::Update => {
                old_value = Some(Value {
                    typed: Some(Typed::Int32(self.old_value)),
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

// ---------- BigDecimal ----------
impl ToField for BigDecimal {
    fn to_field<N: AsRef<str>>(&self, name: N) -> Field {
        Field {
            name: name.as_ref().to_string(),
            new_value: Some(Value {
                typed: Some(Typed::Bigdecimal(self.to_string().clone())),
            }),
            old_value: None,
        }
    }
}

impl ToField for DeltaBigDecimal {
    fn to_field<N: AsRef<str>>(&self, name: N) -> Field {
        let new_value: Option<Value> = Some(Value {
            typed: Some(Typed::Bigdecimal(self.new_value.to_string().clone())),
        });
        let mut old_value: Option<Value> = None;

        match Operation::from(self.operation) {
            Operation::Update => {
                old_value = Some(Value {
                    typed: Some(Typed::Bigdecimal(self.old_value.to_string().clone())),
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
// ---------- BigInt ----------
impl ToField for BigInt {
    fn to_field<N: AsRef<str>>(&self, name: N) -> Field {
        Field {
            name: name.as_ref().to_string(),
            new_value: Some(Value {
                typed: Some(Typed::Bigint(self.to_string().clone())),
            }),
            old_value: None,
        }
    }
}

impl ToField for DeltaBigInt {
    fn to_field<N: AsRef<str>>(&self, name: N) -> Field {
        let new_value: Option<Value> = Some(Value {
            typed: Some(Typed::Bigint(self.new_value.to_string().clone())),
        });
        let mut old_value: Option<Value> = None;

        match Operation::from(self.operation) {
            Operation::Update => {
                old_value = Some(Value {
                    typed: Some(Typed::Bigint(self.old_value.to_string().clone())),
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

// ---------- String ----------
impl ToField for String {
    fn to_field<N: AsRef<str>>(&self, name: N) -> Field {
        Field {
            name: name.as_ref().to_string(),
            new_value: Some(Value {
                typed: Some(Typed::String(self.to_string())),
            }),
            old_value: None,
        }
    }
}

impl ToField for DeltaString {
    fn to_field<N: AsRef<str>>(&self, name: N) -> Field {
        let new_value: Option<Value> = Some(Value {
            typed: Some(Typed::String(self.new_value.to_string())),
        });
        let mut old_value: Option<Value> = None;

        match Operation::from(self.operation) {
            Operation::Update => {
                old_value = Some(Value {
                    typed: Some(Typed::String(self.old_value.to_string())),
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

// ---------- Bytes ----------
impl ToField for Vec<u8> {
    fn to_field<N: AsRef<str>>(&self, name: N) -> Field {
        Field {
            name: name.as_ref().to_string(),
            new_value: Some(Value {
                typed: Some(Typed::Bytes(base64::encode(self.clone()))),
            }),
            old_value: None,
        }
    }
}

impl ToField for DeltaBytes {
    fn to_field<N: AsRef<str>>(&self, name: N) -> Field {
        let new_value: Option<Value> = Some(Value {
            typed: Some(Typed::Bytes(base64::encode(self.new_value.clone()))),
        });
        let mut old_value: Option<Value> = None;

        match Operation::from(self.operation) {
            Operation::Update => {
                old_value = Some(Value {
                    typed: Some(Typed::Bytes(base64::encode(self.old_value.clone()))),
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

// ---------- BoolChange ----------
impl ToField for bool {
    fn to_field<N: AsRef<str>>(&self, name: N) -> Field {
        Field {
            name: name.as_ref().to_string(),
            new_value: Some(Value {
                typed: Some(Typed::Bool(*self)),
            }),
            old_value: None,
        }
    }
}

impl ToField for DeltaBool {
    fn to_field<N: AsRef<str>>(&self, name: N) -> Field {
        let new_value: Option<Value> = Some(Value {
            typed: Some(Typed::Bool(self.new_value)),
        });
        let mut old_value: Option<Value> = None;

        match Operation::from(self.operation) {
            Operation::Update => {
                old_value = Some(Value {
                    typed: Some(Typed::Bool(self.old_value)),
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
impl ToField for Vec<String> {
    fn to_field<N: AsRef<str>>(&self, name: N) -> Field {
        Field {
            name: name.as_ref().to_string(),
            new_value: Some(str_vec_to_pb(self.to_vec())),
            old_value: None,
        }
    }
}

impl ToField for DeltaArray<String> {
    fn to_field<N: AsRef<str>>(&self, name: N) -> Field {
        let new_value: Option<Value> = Some(str_vec_to_pb(self.new_value.clone()));
        let mut old_value: Option<Value> = None;

        match Operation::from(self.operation) {
            Operation::Update => {
                old_value = Some(str_vec_to_pb(self.old_value.clone()));
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

fn str_vec_to_pb(items: Vec<String>) -> Value {
    let mut list: Vec<Value> = vec![];
    for item in items.iter() {
        list.push(Value {
            typed: Some(Typed::String(item.clone())),
        });
    }
    Value {
        typed: Some(Typed::Array(Array { value: list })),
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
