use mongodb::{Client, Collection};
use crate::modals::User;

const DB_URI: &str = "mongodb+srv://skumar131431:Sandeep%40123@cluster0.rnpmyr2.mongodb.net/";
const DB_NAME: &str = "userdb";
const COLLECTION_NAME: &str = "users";

pub async fn connect_db() -> Collection<User> {
    let client = Client::with_uri_str(DB_URI)
        .await
        .expect("MongoDB connection failed");
    
    client.database(DB_NAME).collection(COLLECTION_NAME)
}