//extern crate mongodb;

use mongodb::error::Error;
use mongodb::sync::{Client, Collection};
use mongodb::bson::{doc, bson};

// mod mongo;

// use mongo::{mongo_connect, insert};

//creating a fuction to connect with perticular mongodb
pub fn mongo_connect(coll: &str) -> Result<Collection, Error> {
    let client = Client::with_uri_str("mongodb://localhost:27017")?;
    let db = client.database("practice");
    let collection = db.collection(coll);
    Ok(collection)
}
fn insert(coll: Collection, data: bson::Document){
    let data_in_bson = data.insert(key: KT, val: BT);

    coll.insert_one(data, None);

}


fn main() {
    let collection = mongo_connect("family_members").unwrap();

    let doc = bson!("name": "oadfa");

    insert(collection, doc);


    
    //collection.insert_one(doc,None);

}