use notion_client::endpoints::{
    search::title::{request::{Filter, SearchByTitleRequestBuilder, Sort, SortDirection, Timestamp}, response::{PageOrDatabase, SearchByTitleResponse}},
    Client,
};
use anyhow::Result;
use serde_json::{json, Value};

pub async fn setup_notion_client(notion_token: &str) -> Result<Client> {
    
    let client =  Client::new(notion_token.to_string(), None)?; //using the default reqwest client provided by notion_client crate
    //let db = client.databases.retreive_a_database(db_id).await?;
    println!("Connected to client"); 
    Ok(client)
}

pub async fn get_database(client: Client, db_id: &str) -> Result<notion_client::objects::database::Database> {
    let db = client.databases.retrieve_a_database(db_id).await?;
    Ok(db)
}

pub async fn get_all_databases(client: Client) -> Result<Vec<(String, String)>> {
    
    let mut request = SearchByTitleRequestBuilder::default();
    request.filter(Filter {
            value: notion_client::endpoints::search::title::request::FilterValue::Database,
            property: notion_client::endpoints::search::title::request::FilterProperty::Object,
        });
    request.sort(Sort {
            timestamp: Timestamp::LastEditedTime,
            direction: SortDirection::Ascending,
        });
    
    let response = client.search.search_by_title(request.build().unwrap()).await?;
    
    let databases = response.results
        .iter()
        .filter_map(|entry| {
            if let PageOrDatabase::Database(db) = entry {
                let id = db.id.clone().unwrap_or_default();
                let name = db.title
                    .get(0)
                    .and_then(|text_block| text_block.plain_text().clone())
                    .unwrap_or_else(|| "<Untitled>".to_string());
                Some((id, name))    
            } else {
                None
            }
    }).collect();
    
    Ok(databases)
}

