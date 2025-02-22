//! **`reindeer` 🦌 is a small entity-based embedded database with a minimal no-SQL relationnal model, written in pure Rust.**
//!
//! It uses [`sled`](https://docs.rs/sled/latest/sled/), [`serde`](https://docs.rs/serde/latest/serde/)
//! and  [`bincode`](https://docs.rs/bincode/latest/bincode/) under the hood.
//!
//! *`reindeer` 🦌 lifts your `sled`!*
//!
//! It relies on a trait, [`Entity`](entity/trait.Entity.html), to provide basic document store capabilities to any `serde`-serializable struct that implements it.  
//!
//! To use `reindeer`, add a key to identify any instance individually to your struct and [derive the `Entity` trait](derive.Entity.html), or you can implement the [`store_name`](entity/trait.Entity.html#method.store_name),
//! [`get_key`](entity/trait.Entity.html#method.get_key) and [`set_key`](entity/trait.Entity.html#method.set_key) methods to implement
//! the [`reindeer::Entity`](entity/trait.Entity.html) trait
//!
//! You can then use
//!  - the [`Entity::save`](entity/trait.Entity.html#method.save) method to save your struct instance to the database
//!  - the [`Entity::get`](entity/trait.Entity.html#method.get) method to get any entity from the database using its unique key
//!  - the [`Entity::get_all`](entity/trait.Entity.html#method.get_all) method to get all entities from the database using its unique key
//!  - the [`Entity::get_with_filter`](entity/trait.Entity.html#method.get_with_filter) method to get all entities that match a condition (O(n))
//!  - ... And [much more](entity/trait.Entity.html)!
//!
//! If the [`Key`](entity/trait.Entity.html#associatedtype.Key) associated type is `u32`, then your entity can be auto-incremented
//! with [`save_next()`](entity/trait.AutoIncrementEntity.html#tymethod.save_next)
//! from the [`AutoIncrementEntity`](entity/trait.AutoIncrementEntity.html) (which needs to be in scope)
//!
//! Three types of relationships can be achieved :
//!  - Sibling relationship : two or more `Entity` structs that share the same key type for which each entity has 0 or 1 counterpart
//! in their sibling Entity stores (one-to-zero-or-one)
//!  - Parent-Child relationship : An entity has a collection of matching entities in another Entity Store (one-to-many)
//!  - Free relationship : Any two entities can be linked together as a two-way link. (many-to-many)
//!
//! Those provide integrity checks in the form of a [`DeletionBehaviour`](relation/enum.DeletionBehaviour.html) enum, that can either be :
//!  - `DeletionBehaviour::Cascade` : related entities are also removed if this one is removed
//!  - `DeletionBehaviour::Error` : Trying to remove this entity as related entities still exist will cause an error and abort
//!  - `DeletionBehaviour::BreakLink` : Remove this entity and the links with its related entites, leaving the other ones untouched

mod entity;
mod error;
mod import_export;
mod query_builder;
mod relation;
pub use entity::AutoIncrementEntity;
pub use entity::{AsBytes, Entity};
pub use reindeer_macros::Entity;

pub use query_builder::*;
pub use relation::DeletionBehaviour;
pub use serde_derive::{Deserialize, Serialize};

pub use error::{Error, ErrorKind, Result};
/// Opens a `sled` database to store Entities. The resulting Db object can be copied accross threads. This is a re-export of `sled::open`.
///
pub use sled::open;
/// `sled` database struct. It can be copied and used accross threads and is a central item to store entities. This is a re-export of `sled::Db`.
///
pub use sled::Db;

#[cfg(test)]
mod test;
