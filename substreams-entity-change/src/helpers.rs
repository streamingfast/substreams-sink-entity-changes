use crate::change::ToField;
use crate::pb::entity::entity_change::Operation;
use crate::pb::entity::{EntityChange, EntityChanges};
use std::str;

impl EntityChanges {
    pub fn push_change<V: AsRef<str>>(
        &mut self,
        entity: V,
        id: V,
        ordinal: u64,
        operation: Operation,
    ) -> &mut EntityChange {
        let entity_change = EntityChange::new(entity, id, ordinal, operation);
        self.entity_changes.push(entity_change);
        return self.entity_changes.last_mut().unwrap();
    }
}

impl EntityChange {
    pub fn new<V: AsRef<str>>(
        entity: V,
        id: V,
        ordinal: u64,
        operation: Operation,
    ) -> EntityChange {
        EntityChange {
            entity: entity.as_ref().to_string(),
            id: id.as_ref().to_string(),
            ordinal,
            operation: operation as i32,
            fields: vec![],
        }
    }

    pub fn change<N: AsRef<str>, T: ToField>(&mut self, name: N, change: T) -> &mut EntityChange {
        self.fields.push(change.to_field(name));
        self
    }
}
