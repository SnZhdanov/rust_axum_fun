use futures::stream::TryStreamExt;
use mongodb::bson::Document;
use mongodb::Cursor;
use serde::de::DeserializeOwned;
use serde::Deserialize;

pub struct CollectCusrorResult<JsonStruct> {
    successfully_deserialized: Vec<JsonStruct>,
    failed_deserialized: Vec<String>,
    dropped: u64,
}

impl<JsonStruct> CollectCusrorResult<JsonStruct>
where
    JsonStruct: Clone,
{
    pub fn get_results(&self) -> (Vec<JsonStruct>, Vec<String>, u64) {
        return (
            self.successfully_deserialized.clone(),
            self.failed_deserialized.clone(),
            self.dropped,
        );
    }
}

pub async fn collect_cursor<BsonStruct, JsonStruct>(
    cursor: Cursor<Document>,
) -> CollectCusrorResult<JsonStruct>
where
    BsonStruct: Clone,
    BsonStruct: Into<JsonStruct>,
    BsonStruct: DeserializeOwned,
    JsonStruct: DeserializeOwned,
    JsonStruct: Clone,
{
    let documents: Vec<Document> = match cursor.try_collect().await {
        Ok(docs) => docs,
        Err(e) => todo!(),
    };

    let mut dropped: u64 = 0;
    let mut successfully_deserialized: Vec<JsonStruct> = [].to_vec();
    let mut failed_deserialized: Vec<String> = [].to_vec();
    for doc in documents.into_iter() {
        let deserialized: JsonStruct = match mongodb::bson::from_document::<BsonStruct>(doc.clone())
        {
            Ok(deserialized) => deserialized.into(),
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
