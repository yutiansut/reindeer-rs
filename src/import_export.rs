use serde::{Serialize, de::DeserializeOwned};
use serde_derive::{Serialize, Deserialize};
use sled::Db;
use crate::{Result, AsBytes};

use crate::relation::Relation;
use crate::{Entity, relation::EntityRelations};

#[derive(Serialize,Deserialize)]
pub struct JsonWrapper<T>(Vec<(T,Option<EntityRelations>)>);


impl<T: Entity> JsonWrapper<T> {
    pub fn from(source_vec : Vec<T>, db : &Db) -> Result<Self> {
        let entries : Result<Vec<(T,Option<EntityRelations>)>> = source_vec.into_iter().map(|source| {
            let relations = Relation::get_descriptor_with_key_and_tree_name(T::store_name(), &source.get_key().as_bytes(), db)?;
            if relations.related_entities.len() > 0 {
                Ok((source,Some(relations)))
            }
            else {
                Ok((source,None))
            }
        }).collect();
        Ok(Self(entries?))
    }
    pub fn save(self, db : &Db) -> Result<()> {
        for (entity,relations) in self.0 {
            entity.save(db)?;
            if let Some(relations) = relations {
                Relation::save_descriptor(&entity, &relations, db)?;
            }
        }
        Ok(())
    }
}