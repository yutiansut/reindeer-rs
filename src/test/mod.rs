mod test_entities;

use test_entities::{Entity1,Entity2,Entity3,ChildEntity1,ChildEntity2,set_up,set_up_content,tear_down};
use crate::{relation::FamilyDescriptor, Entity,AutoIncrementEntity, DeletionBehaviour};
use uuid::Uuid;


fn get_random_name() -> String {
    format!("sled-entity-test-{}",Uuid::new_v4().to_string())
}
#[test]
fn create_and_register() -> Result<(), std::io::Error> {
    let name = get_random_name();
    let db = set_up(&name)?;
    assert!(FamilyDescriptor::exists(&String::from("entity_1"), &db)?);
    assert!(FamilyDescriptor::exists(&String::from("entity_2"), &db)?);
    assert!(FamilyDescriptor::exists(&String::from("child_entity_1"), &db)?);
    let fam_desc = FamilyDescriptor::get(&String::from("entity_1"),&db)?;
    assert!(fam_desc.is_some());
    assert_eq!(fam_desc.unwrap().sibling_trees.len(),1);
    tear_down(&name)?;
    Ok(())
}

#[test]
fn test_save_save_next_and_get() -> Result<(), std::io::Error> {
    let name = get_random_name();
    let db = set_up(&name)?;
    set_up_content(&db)?;
    let e1_0 = Entity1::get(&0,&db)?;
    let e1_1 = Entity1::get(&1,&db)?;
    let e2_1 = Entity2::get(&String::from("id1"),&db)?;
    let e2_2 = Entity2::get(&String::from("id2"),&db)?;
    assert!(e1_0.is_some());
    assert!(e1_1.is_some());
    assert!(e2_1.is_some());
    assert!(e2_2.is_some());
    let e1_0 = e1_0.unwrap();
    let e1_1 = e1_1.unwrap();
    let e2_1 = e2_1.unwrap();
    let e2_2 = e2_2.unwrap();
    assert_eq!(e1_0.id,0);
    assert_eq!(e1_0.prop1,"Hello, World!");
    assert_eq!(e1_1.id,1);
    assert_eq!(e1_1.prop1,"Hello, Nancy!");
    assert!(Entity1::get(&8,&db)?.is_none());
    assert_eq!(e2_1.prop2,3);
    assert_eq!(e2_2.prop2,5);
    tear_down(&name)?;
    Ok(())
}

#[test]
fn test_save_and_get_children() -> Result<(), std::io::Error> {
    let name = get_random_name();
    let db = set_up(&name)?;
    set_up_content(&db)?;
    let child_1 = ChildEntity1::get(&(String::from("id3"),0), &db)?;
    assert!(child_1.is_some());
    let e2_3 = Entity2::get(&String::from("id3"),&db)?.unwrap();
    let children = e2_3.get_children::<ChildEntity1>(&db)?;
    assert_eq!(children.len(),3);
    tear_down(&name)?;
    Ok(())
}


#[test]
fn test_cascade_children() -> Result<(), std::io::Error> {
    let name = get_random_name();
    let db = set_up(&name)?;
    set_up_content(&db)?;
    let e2_3 = Entity2::get(&String::from("id3"),&db)?.unwrap();
    let children = e2_3.get_children::<ChildEntity1>(&db)?;
    assert_eq!(children.len(),3);
    Entity2::remove(&String::from("id3"), &db)?;
    assert!(Entity2::get(&String::from("id3"),&db)?.is_none());
    assert_eq!(e2_3.get_children::<ChildEntity1>(&db)?.len(),0);
    tear_down(&name)?;
    Ok(())
}

#[test]
fn test_delete_children_error() -> Result<(), std::io::Error> {
    let name = get_random_name();
    let db = set_up(&name)?;
    set_up_content(&db)?;
    let e3_2 = Entity3::get(&2,&db)?.unwrap();
    let children = e3_2.get_children::<ChildEntity2>(&db)?;
    assert_eq!(children.len(),3);
    assert!(Entity3::remove(&2, &db).is_err());
    let e3_2 = Entity3::get(&2,&db)?;
    assert!(e3_2.is_some());
    assert_eq!(e3_2.unwrap().get_children::<ChildEntity2>(&db)?.len(),3);
    tear_down(&name)?;
    Ok(())
}

#[test]
fn test_add_sibling() -> Result<(), std::io::Error> {
    let name = get_random_name();
    let db = set_up(&name)?;
    set_up_content(&db)?;
    let mut e1 = Entity1 { id : 0, prop1 : String::from("First Sibling")};
    e1.save_next(&db)?;
    let mut e3 = Entity3 { id : 0 };
    e1.save_sibling(&mut e3, &db)?;
    assert_eq!(e3.id,e1.id);
    assert_eq!(e3.id,3);
    tear_down(&name)?;
    Ok(())
}

#[test]
fn test_delete_sibling_cascade() -> Result<(), std::io::Error> {
    let name = get_random_name();
    let db = set_up(&name)?;
    set_up_content(&db)?;
    let mut e1 = Entity1 { id : 0, prop1 : String::from("First Sibling")};
    e1.save_next(&db)?;
    let mut e3 = Entity3 { id : 0 };
    e1.save_sibling(&mut e3, &db)?;
    assert!(Entity1::remove(&e1.get_key(), &db).is_ok());
    assert!(Entity1::get(&e1.get_key(),&db)?.is_none());
    assert!(Entity3::get(&e3.get_key(),&db)?.is_none());
    tear_down(&name)?;
    Ok(())
}


#[test]
fn test_delete_sibling_error() -> Result<(), std::io::Error> {
    let name = get_random_name();
    let db = set_up(&name)?;
    set_up_content(&db)?;
    let mut e1 = Entity1 { id : 0, prop1 : String::from("First Sibling")};
    e1.save_next(&db)?;
    let mut e3 = Entity3 { id : 0 };
    e1.save_sibling(&mut e3, &db)?;
    assert!(Entity3::remove(&e1.get_key(), &db).is_err());
    assert!(Entity1::get(&e1.get_key(),&db)?.is_some());
    assert!(Entity3::get(&e3.get_key(),&db)?.is_some());
    tear_down(&name)?;
    Ok(())
}

#[test]
fn test_free_relation() -> Result<(), std::io::Error> {
    let name = get_random_name();
    let db = set_up(&name)?;
    set_up_content(&db)?;
    let e1 = Entity1::get(&2,&db)?.unwrap();
    let e2_1 = Entity2::get(&String::from("id1"),&db)?.unwrap();
    let e2_2 = Entity2::get(&String::from("id2"),&db)?.unwrap();
    assert!(e1.create_relation(&e2_1, DeletionBehaviour::Cascade, DeletionBehaviour::Error, &db).is_ok());
    assert!(e1.create_relation(&e2_2, DeletionBehaviour::Cascade, DeletionBehaviour::Error, &db).is_ok());
    let related = e1.get_related::<Entity2>(&db)?;
    assert_eq!(related.len(),2);
    assert_eq!(related[0].get_key(),"id1");
    assert_eq!(related[1].get_key(),"id2");
    tear_down(&name)?;
    Ok(())
}

#[test]
fn test_free_relation_cascade() -> Result<(), std::io::Error> {
    let name = get_random_name();
    let db = set_up(&name)?;
    set_up_content(&db)?;
    let mut e1 = Entity1 { id : 0, prop1 : String::from("First Sibling")};
    e1.save_next(&db)?;
    let e2_1 = Entity2::get(&String::from("id1"),&db)?.unwrap();
    let e2_2 = Entity2::get(&String::from("id2"),&db)?.unwrap();
    assert!(e1.create_relation(&e2_1, DeletionBehaviour::Cascade, DeletionBehaviour::Error, &db).is_ok());
    assert!(e1.create_relation(&e2_2, DeletionBehaviour::Cascade, DeletionBehaviour::Error, &db).is_ok());
    let related = e1.get_related::<Entity2>(&db)?;
    assert_eq!(related.len(),2);
    assert!(Entity1::remove(&e1.get_key(), &db).is_ok());
    assert_eq!(e1.get_related::<Entity2>(&db)?.len(),0);
    assert!(Entity2::get(&String::from("id1"),&db)?.is_none());
    assert!(Entity2::get(&String::from("id2"),&db)?.is_none());
    tear_down(&name)?;
    Ok(())
}

#[test]
fn test_free_relation_error() -> Result<(), std::io::Error> {
    let name = get_random_name();
    let db = set_up(&name)?;
    set_up_content(&db)?;
    let mut e1 = Entity1 { id : 0, prop1 : String::from("First Sibling")};
    e1.save_next(&db)?;
    let e2_1 = Entity2::get(&String::from("id1"),&db)?.unwrap();
    let e2_2 = Entity2::get(&String::from("id2"),&db)?.unwrap();
    assert!(e1.create_relation(&e2_1, DeletionBehaviour::Cascade, DeletionBehaviour::Error, &db).is_ok());
    assert!(e1.create_relation(&e2_2, DeletionBehaviour::Cascade, DeletionBehaviour::Error, &db).is_ok());
    let related = e1.get_related::<Entity2>(&db)?;
    assert_eq!(related.len(),2);
    assert!(Entity2::remove(&e2_1.get_key(), &db).is_err());
    assert_eq!(e1.get_related::<Entity2>(&db)?.len(),2);
    tear_down(&name)?;
    Ok(())
}

#[test]
fn test_recursive_cascade() -> Result<(), std::io::Error> {
    let name = get_random_name();
    let db = set_up(&name)?;
    set_up_content(&db)?;
    let mut e1 = Entity1 { id : 0, prop1 : String::from("First Sibling")};
    e1.save_next(&db)?;
    let e2_1 = Entity2::get(&String::from("id1"),&db)?.unwrap();
    let e2_3 = Entity2::get(&String::from("id3"),&db)?.unwrap();
    assert!(e1.create_relation(&e2_1, DeletionBehaviour::Cascade, DeletionBehaviour::Error, &db).is_ok());
    assert!(e1.create_relation(&e2_3, DeletionBehaviour::Cascade, DeletionBehaviour::Error, &db).is_ok());
    let related = e1.get_related::<Entity2>(&db)?;
    assert_eq!(related.len(),2);
    assert!(Entity1::remove(&e1.get_key(), &db).is_ok());
    assert_eq!(e1.get_related::<Entity2>(&db)?.len(),0);
    assert_eq!(ChildEntity1::get_number(&db)?,0);
    tear_down(&name)?;
    Ok(())
}

#[test]
fn test_recursive_error() -> Result<(), std::io::Error> {
    let name = get_random_name();
    let db = set_up(&name)?;
    set_up_content(&db)?;
    let e1 = Entity1::get(&2,&db)?.unwrap();
    let e2_1 = Entity2::get(&String::from("id1"),&db)?.unwrap();
    let e2_3 = Entity2::get(&String::from("id3"),&db)?.unwrap();
    assert!(e1.create_relation(&e2_1, DeletionBehaviour::Cascade, DeletionBehaviour::Error, &db).is_ok());
    assert!(e1.create_relation(&e2_3, DeletionBehaviour::Cascade, DeletionBehaviour::Error, &db).is_ok());
    let related = e1.get_related::<Entity2>(&db)?;
    assert_eq!(related.len(),2);
    assert!(Entity1::remove(&e1.get_key(), &db).is_err());
    assert_eq!(e1.get_related::<Entity2>(&db)?.len(),2);
    assert_eq!(ChildEntity1::get_number(&db)?,3);
    tear_down(&name)?;
    Ok(())
}