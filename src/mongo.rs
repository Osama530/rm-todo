use mongodb::error::Error;
use mongodb::sync::{Client, Collection};

//creating a fuction to connect with perticular collection
pub fn mongo_connect(coll: &str) -> Result<Collection, Error> {
    let client = Client::with_uri_str("mongodb://localhost:27017")?;
    let db = client.database("practice");
    let collection = db.collection(coll);
    Ok(collection)
}

//match and return a perticular collection
pub fn collection(coll_name: &str) -> Collection {
    let my_collection = match mongo_connect(coll_name) {
        Ok(coll) => coll,
        Err(_) => panic!("error in collection"),
    };
    my_collection
}

pub fn insert(data: String) {
    
}
