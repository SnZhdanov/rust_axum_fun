use futures::stream::TryStreamExt;
use mongodb::bson::Document;
use mongodb::Cursor;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

pub struct CollectCusrorResult<BsonStruct> {
    successfully_deserialized: Vec<BsonStruct>,
    failed_deserialized: Vec<String>,
    dropped: u64,
}

impl<BsonStruct> CollectCusrorResult<BsonStruct>
where
    BsonStruct: Clone,
{
    pub fn get_results(&self) -> (Vec<BsonStruct>, Vec<String>, u64) {
        return (
            self.successfully_deserialized.clone(),
            self.failed_deserialized.clone(),
            self.dropped,
        );
    }
}

pub async fn collect_cursor<JsonStruct, BsonStruct>(
    cursor: Cursor<Document>,
) -> CollectCusrorResult<BsonStruct>
where
    BsonStruct: Clone,
    BsonStruct: DeserializeOwned,
    BsonStruct: Debug,
    JsonStruct: Into<BsonStruct>,
    JsonStruct: Debug,
    JsonStruct: DeserializeOwned,
    JsonStruct: Clone,
{
    let documents: Vec<Document> = match cursor.try_collect().await {
        Ok(docs) => docs,
        Err(e) => todo!(),
    };

    let mut dropped: u64 = 0;
    let mut successfully_deserialized: Vec<BsonStruct> = [].to_vec();
    let mut failed_deserialized: Vec<String> = [].to_vec();
    for doc in documents.into_iter() {
        let deserialized: BsonStruct = match mongodb::bson::from_document::<JsonStruct>(doc.clone())
        {
            Ok(deserialized) => {
                println!(" json form {:?}", deserialized);
                let y = deserialized.into();
                println!("response form {:?}", y);
                y
            }
            Err(e) => {
                dropped += 1;

                match doc.get("_id") {
                    Some(id) => failed_deserialized.push(id.to_string()),
                    None => (),
                }
                continue;
            }
        };

        successfully_deserialized.push(deserialized)
    }
    CollectCusrorResult {
        successfully_deserialized,
        failed_deserialized,
        dropped,
    }
}
