use crate::pb::entity::value::Typed;
use crate::pb::entity::{Array, Field, Value};
use std::str;
use substreams::pb::substreams::store_delta::Operation;
use substreams::scalar::{BigDecimal, BigInt};
use substreams::store::{
    DeltaArray, DeltaBigDecimal, DeltaBigInt, DeltaBool, DeltaBytes, DeltaI32, DeltaString,
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

impl ToField for DeltaI32 {
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
    use crate::pb::entity::{Field, Value};
    use substreams::pb::substreams::store_delta::Operation;
    use substreams::scalar::BigDecimal;
    use substreams::store::DeltaBigDecimal;

    #[test]
    fn delta_big_decimal_change() {
        let delta = DeltaBigDecimal {
            operation: Operation::Update,
            ordinal: 0,
            key: "change".to_string(),
            old_value: BigDecimal::from(10),
            new_value: BigDecimal::from(20),
        };

        let expected_field = Field {
            name: "field.name.1".to_string(),
            new_value: Some(Value {
                typed: Some(Typed::Bigdecimal("20".to_string())),
            }),
            old_value: Some(Value {
                typed: Some(Typed::Bigdecimal("10".to_string())),
            }),
        };

        let actual_field = delta.to_field("field.name.1");
        assert_eq!(expected_field, actual_field);
    }

    #[test]
    fn big_decimal_change() {
        let bd = BigDecimal::from(1 as i32);

        let expected_field = Field {
            name: "field.name.1".to_string(),
            new_value: Some(Value {
                typed: Some(Typed::Bigdecimal("1".to_string())),
            }),
            old_value: None,
        };

        let actual_field = bd.to_field("field.name.1");
        assert_eq!(expected_field, actual_field);
    }
}
